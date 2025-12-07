use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

enum PlayerCommand {
    TogglePause,
    Quit,
}

pub fn player(filename_wav: String) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let audio_thread = thread::spawn(move || -> Result<()> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&stream_handle.mixer());

        let file = File::open(filename_wav)?;
        let source = Decoder::new(BufReader::new(file))?;
        sink.append(source);

        loop {
            match rx.try_recv() {
                Ok(PlayerCommand::TogglePause) => {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                Ok(PlayerCommand::Quit) => {
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No command, continue.
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }

            if sink.empty() {
                break;
            }

            thread::sleep(Duration::from_millis(200));
        }
        Ok(())
    });

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal, tx.clone());
    ratatui::restore();

    // The TUI loop has finished, we can tell the audio thread to quit.
    // If it hasn't been told to quit already.
    let _ = tx.send(PlayerCommand::Quit);
    audio_thread.join().unwrap()?;

    app_result
}

fn run(terminal: &mut DefaultTerminal, tx: mpsc::Sender<PlayerCommand>) -> Result<()> {
    loop {
        terminal.draw(draw)?;

        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
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

fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Playing audio. Press 'q' to quit, <SPACE> to pause/resume.");
    frame.render_widget(greeting, frame.area());
}
