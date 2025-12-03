use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

pub fn player_tui() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

pub fn play(filename_wav: String) -> Result<(), Box<dyn std::error::Error>> {
    
    let filename_wav = filename_wav;
    
    // Get a handle to the default output stream and a stream handle
    let (stream_handle) = OutputStreamBuilder::open_default_stream()?;

    // Create a new Sink, which manages playback
    let sink = Sink::connect_new(&stream_handle.mixer());

    // Open the audio file
    let file = File::open(filename_wav)?;
    let file_reader = BufReader::new(file);

    // Decode the audio file
    let source = Decoder::new(file_reader)?;

    // Add the decoded audio to the sink
    sink.append(source);

    // Wait for the audio to finish playing
    sink.sleep_until_end();

    Ok(())
}