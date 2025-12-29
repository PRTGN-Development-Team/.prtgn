use rodio::{OutputStreamBuilder, Sink, Source};
use rodio::buffer::SamplesBuffer;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    DefaultTerminal, Frame,
};


enum PlayerCommand {
    Play,
    TogglePause,
    Quit,
}



pub fn player(source: SamplesBuffer, _filename: String) -> Result<()> {
    let (command_tx, command_rx) = mpsc::channel();
    let (progress_tx, progress_rx) = mpsc::channel();


    let sample_rate = source.sample_rate();
    let total_duration = source.total_duration().unwrap_or_default();

    // Audio thread
    let audio_thread = thread::spawn(move || -> Result<()> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&stream_handle.mixer());

        sink.pause();
        sink.append(source);

        let mut elapsed_time = Duration::ZERO;
        let mut last_update = Instant::now();
        let mut is_playing = false;

        loop {
            match command_rx.try_recv() {
                Ok(PlayerCommand::Play) => {
                    if sink.is_paused() {
                        sink.play();
                        is_playing = true;
                        last_update = Instant::now();
                    }
                }
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

            thread::sleep(Duration::from_millis(33));
        }
        Ok(())
    });

    color_eyre::install()?;
    let mut terminal = ratatui::init();

    // Start playback
    let _ = command_tx.send(PlayerCommand::Play);

    let app_result = run(
        &mut terminal,
        command_tx.clone(),
        progress_rx,
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
    sample_rate: u32,
    total_duration: Duration,
) -> Result<()> {
    let mut current_progress = (Duration::ZERO, total_duration);

    loop {
        // Drain progress channel to get the latest update
        while let Ok(progress) = progress_rx.try_recv() {
            current_progress = progress;
        }



        terminal.draw(|f| draw(f, sample_rate, current_progress))?;

        // Non-blocking event poll loop
        let mut quit = false;
        while event::poll(Duration::ZERO).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            quit = true;
                            break;
                        },
                        KeyCode::Char(' ') => {
                            tx.send(PlayerCommand::TogglePause)?;
                        }
                        _ => {}
                    }
                }
            }
        }

        if quit {
            break;
        }

        thread::sleep(Duration::from_millis(33));
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

    // Info
    let text = vec![
        format!("Playing audio at {} samples per second.", sample_rate),
        "Press 'q' to quit, <SPACE> to pause/resume.".to_string(),
        String::new(),
    ];

    let info_block = Paragraph::new(text.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .wrap(Wrap { trim: true });
    frame.render_widget(info_block, top_layout[0]);


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