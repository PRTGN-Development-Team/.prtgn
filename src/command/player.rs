use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::path::Path;

use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use ratatui_image::{picker::Picker, StatefulImage, protocol::StatefulProtocol};
use image::DynamicImage;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::Search;

enum PlayerCommand {
    TogglePause,
    Quit,
}

struct TrackMetadata {
    title: String,
    artist: String,
    album: String,
    cover_art: Option<DynamicImage>,
}

async fn fetch_metadata_async(query: String) -> Option<TrackMetadata> {
    // Search for the recording
    let query_res = Recording::search(query).execute().await;

    if let Ok(response) = query_res {
        if let Some(recording) = response.entities.first() {
            let title = recording.title.clone();
            let artist = recording.artist_credit.as_ref()
                .map(|ac| ac.iter().map(|a| a.name.clone()).collect::<Vec<_>>().join(", "))
                .unwrap_or_else(|| "Unknown Artist".to_string());

            let mut album = "Unknown Album".to_string();
            let mut cover_art = None;

            if let Some(releases) = &recording.releases {
                if let Some(release) = releases.first() {
                    album = release.title.clone();

                    // Try to fetch cover art
                    let url = format!("https://coverartarchive.org/release/{}/front", release.id);
                    if let Ok(resp) = reqwest::get(&url).await {
                        if let Ok(bytes) = resp.bytes().await {
                            if let Ok(img) = image::load_from_memory(&bytes) {
                                cover_art = Some(img);
                            }
                        }
                    }
                }
            }

            return Some(TrackMetadata {
                title,
                artist,
                album,
                cover_art,
            });
        }
    }
    None
}

pub fn player(filename: String) -> Result<()> {
    let (command_tx, command_rx) = mpsc::channel();
    let (progress_tx, progress_rx) = mpsc::channel();
    let (metadata_tx, metadata_rx) = mpsc::channel();

    let file_path = filename.clone();
    let file = File::open(filename.clone())?;
    let source = Decoder::new(BufReader::new(file))?;
    let sample_rate = source.sample_rate();
    let total_duration = source.total_duration().unwrap_or_default();

    // Audio thread
    let audio_thread = thread::spawn(move || -> Result<()> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&stream_handle.mixer());

        let file = File::open(filename)?;
        let source = Decoder::new(BufReader::new(file))?;
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

    // Metadata thread
    let filename_query = Path::new(&file_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let meta = rt.block_on(fetch_metadata_async(filename_query));

        if let Some(m) = meta {
            let _ = metadata_tx.send(m);
        } else {
             let _ = metadata_tx.send(TrackMetadata {
                 title: "Unknown Title".to_string(),
                 artist: "Unknown Artist".to_string(),
                 album: "Unknown Album".to_string(),
                 cover_art: None,
             });
        }
    });

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = run(
        &mut terminal,
        command_tx.clone(),
        progress_rx,
        metadata_rx,
        sample_rate,
        total_duration,
    );
    ratatui::restore();

    let _ = command_tx.send(PlayerCommand::Quit);
    audio_thread.join().unwrap()?;

    app_result
}

fn run(
    terminal: &mut DefaultTerminal,
    tx: mpsc::Sender<PlayerCommand>,
    progress_rx: mpsc::Receiver<(Duration, Duration)>,
    metadata_rx: mpsc::Receiver<TrackMetadata>,
    sample_rate: u32,
    total_duration: Duration,
) -> Result<()> {
    let mut current_progress = (Duration::ZERO, total_duration);
    let mut metadata: Option<TrackMetadata> = None;
    let mut image_protocol: Option<StatefulProtocol> = None;

    // Initialize picker with auto-detection or fallback
    let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)));

    loop {
        if let Ok(progress) = progress_rx.try_recv() {
            current_progress = progress;
        }

        if let Ok(meta) = metadata_rx.try_recv() {
            if let Some(img) = &meta.cover_art {
                image_protocol = Some(picker.new_resize_protocol(img.clone()));
            }
            metadata = Some(meta);
        }

        terminal.draw(|f| draw(f, sample_rate, current_progress, &metadata, &mut image_protocol))?;

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
    metadata: &Option<TrackMetadata>,
    image_protocol: &mut Option<StatefulProtocol>
) {
    let (current_pos, total_duration) = progress;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[0]);

    // Metadata / Info
    let mut text = vec![
        format!("Playing audio at {} samples per second.", sample_rate),
        "Press 'q' to quit, <SPACE> to pause/resume.".to_string(),
        String::new(),
    ];

    if let Some(meta) = metadata {
        text.push(format!("Title:  {}", meta.title));
        text.push(format!("Artist: {}", meta.artist));
        text.push(format!("Album:  {}", meta.album));
    } else {
        text.push("Fetching metadata from MusicBrainz...".to_string());
    }

    let info_block = Paragraph::new(text.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: true });
    frame.render_widget(info_block, top_layout[0]);

    // Cover Art
    let image_block = Block::default().borders(Borders::ALL).title("Cover Art");
    let inner_area = image_block.inner(top_layout[1]);
    frame.render_widget(image_block, top_layout[1]);

    if let Some(protocol) = image_protocol {
        let image = StatefulImage::default();
        frame.render_stateful_widget(image, inner_area, protocol);
    }

    // Progress Bar
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

    frame.render_widget(progress_bar, layout[1]);
}