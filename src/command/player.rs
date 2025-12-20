use rodio::{OutputStreamBuilder, Sink, Source, buffer::SamplesBuffer};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    DefaultTerminal, Frame,
};
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::Search;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use ratatui_image::{Resize, StatefulImage};
use image::DynamicImage;

enum PlayerCommand {
    TogglePause,
    Quit,
}

pub fn player(source: SamplesBuffer, filename: String) -> Result<()> {
    let (command_tx, command_rx) = mpsc::channel();
    let (progress_tx, progress_rx) = mpsc::channel();

    let sample_rate = source.sample_rate();
    let total_duration = source.total_duration().unwrap_or_default();

    let audio_thread = thread::spawn(move || -> Result<()> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&stream_handle.mixer());

        sink.append(source);

        let mut elapsed_time = Duration::ZERO;
        let mut last_update = Instant::now();
        let mut is_playing = !sink.is_paused();

        loop {
            match command_rx.try_recv() {
                Ok(PlayerCommand::TogglePause) => {
                    if sink.is_paused() {
                        sink.play();
                        is_playing = true;
                        last_update = Instant::now();
                    } else {
                        sink.pause();
                        is_playing = false;
                        elapsed_time += last_update.elapsed();
                    }
                }
                Ok(PlayerCommand::Quit) => break,
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => break,
            }

            if is_playing {
                let current_pos = elapsed_time + last_update.elapsed();
                if progress_tx.send((current_pos, total_duration)).is_err() {
                    break;
                }
            }

            if sink.empty() {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    });

    // Fetch metadata
    let rt = tokio::runtime::Runtime::new()?;
    let (track_title, artist_name, cover_art) = rt.block_on(async {
        fetch_metadata(&filename).await
    });

    // color_eyre::install()?; // Removed duplicate call
    let mut terminal = ratatui::init();

    let mut picker = Picker::new((8, 16));
    picker.guess_protocol();

    let app_result = run(
        &mut terminal,
        command_tx.clone(),
        progress_rx,
        sample_rate,
        total_duration,
        track_title,
        artist_name,
        cover_art,
        &mut picker,
    );
    ratatui::restore();

    let _ = command_tx.send(PlayerCommand::Quit);
    audio_thread.join().unwrap()?;

    app_result
}

async fn fetch_metadata(filename: &str) -> (String, String, Option<DynamicImage>) {
    // Simple heuristic to extract title from filename
    let query_str = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename);

    let query = query_str.replace('_', " ");

    let query_result = Recording::search(query).execute().await;

    let mut title = "Unknown Title".to_string();
    let mut artist = "Unknown Artist".to_string();
    let mut cover_art = None;

    if let Ok(results) = query_result {
        if let Some(recording) = results.entities.first() {
            title = recording.title.clone();
            if let Some(artist_credit) = &recording.artist_credit {
                if let Some(ac) = artist_credit.first() {
                    artist = ac.name.clone();
                }
            }

            // Try to fetch cover art if release exists
            if let Some(releases) = &recording.releases {
                for release in releases {
                    let cover_art_url = format!("https://coverartarchive.org/release/{}/front", release.id);
                    if let Ok(response) = reqwest::get(&cover_art_url).await {
                        if response.status().is_success() {
                            if let Ok(bytes) = response.bytes().await {
                                if let Ok(img) = image::load_from_memory(&bytes) {
                                    cover_art = Some(img);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (title, artist, cover_art)
}


fn run(
    terminal: &mut DefaultTerminal,
    tx: mpsc::Sender<PlayerCommand>,
    progress_rx: mpsc::Receiver<(Duration, Duration)>,
    sample_rate: u32,
    total_duration: Duration,
    track_title: String,
    artist_name: String,
    cover_art: Option<DynamicImage>,
    picker: &mut Picker,
) -> Result<()> {
    let mut current_progress = (Duration::ZERO, total_duration);

    let mut image_protocol: Option<Box<dyn StatefulProtocol>> = if let Some(img) = &cover_art {
        Some(picker.new_resize_protocol(img.clone()))
    } else {
        None
    };


    loop {
        if let Ok(progress) = progress_rx.try_recv() {
            current_progress = progress;
        }

        terminal.draw(|f| draw(f, sample_rate, current_progress, &track_title, &artist_name, &mut image_protocol))?;

        if event::poll(Duration::from_millis(100)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            tx.send(PlayerCommand::TogglePause)?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}", mins, secs)
}

fn draw(
    frame: &mut Frame,
    sample_rate: u32,
    progress: (Duration, Duration),
    track_title: &str,
    artist_name: &str,
    image_protocol: &mut Option<Box<dyn StatefulProtocol>>,
) {
    let (current_pos, total_duration) = progress;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content (Image + Info)
            Constraint::Length(3)  // Progress Bar
        ])
        .split(frame.area());

    let header_text = format!(
        "Playing audio at {} samples per second. Press 'q' to quit, <SPACE> to pause/resume.",
        sample_rate
    );
    let header = Paragraph::new(header_text).block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, layout[0]);

    // Content area split into Image and Info
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(layout[1]);

    // Render Image
    if let Some(protocol) = image_protocol {
        let image = StatefulImage::new(None).resize(Resize::Fit(None));
        frame.render_stateful_widget(image, content_layout[0], protocol);
    } else {
        let placeholder = Paragraph::new("No Cover Art")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(placeholder, content_layout[0]);
    }

    // Render Info
    let info_text = format!("Title: {}\nArtist: {}", track_title, artist_name);
    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Track Info"));
    frame.render_widget(info, content_layout[1]);


    let ratio = if !total_duration.is_zero() {
        (current_pos.as_secs_f64() / total_duration.as_secs_f64()).min(1.0)
    } else {
        0.0
    };

    let progress_label = format!(
        "{}/{}",
        format_duration(current_pos),
        format_duration(total_duration)
    );

    let progress_bar = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(ratio)
        .label(progress_label);

    frame.render_widget(progress_bar, layout[2]);
}
