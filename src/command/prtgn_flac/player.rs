use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
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

enum PlayerCommand {
    TogglePause,
    Quit,
}

pub fn player(filename: String) -> Result<()> {
    let (command_tx, command_rx) = mpsc::channel();
    let (progress_tx, progress_rx) = mpsc::channel();

    let file = File::open(filename.clone())?;
    let source = Decoder::new(BufReader::new(file))?;
    let sample_rate = source.sample_rate();
    let total_duration = source.total_duration().unwrap_or_default();

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

    color_eyre::install()?;
    let mut terminal = ratatui::init();
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
        if let Ok(progress) = progress_rx.try_recv() {
            current_progress = progress;
        }

        terminal.draw(|f| draw(f, sample_rate, current_progress))?;

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

fn draw(frame: &mut Frame, sample_rate: u32, progress: (Duration, Duration)) {
    let (current_pos, total_duration) = progress;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    let text = format!(
        "Playing audio at {} samples per second. Press 'q' to quit, <SPACE> to pause/resume.",
        sample_rate
    );
    let greeting = Paragraph::new(text);
    frame.render_widget(greeting, layout[0]);

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
