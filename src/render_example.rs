use ratatui::{
    backend::CrosstermBackend,
    Terminal, Frame
};
use ratatui_image::{picker::{Picker, ProtocolType}, StatefulImage, protocol::StatefulProtocol};
use std::io::{self, Stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct App {
    // We need to hold the render state.
    image: StatefulProtocol,
}

pub fn render() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt for backend selection
    println!("Select rendering backend:");
    println!("1. Auto-detect");
    println!("2. Kitty");
    println!("3. Iterm2");
    println!("4. Sixel");
    println!("5. Halfblocks");
    print!("Enter choice (default 1): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    let font_size = (8, 12);
    let mut picker = Picker::from_fontsize(font_size);

    match choice {
        "2" => picker.set_protocol_type(ProtocolType::Kitty),
        "3" => picker.set_protocol_type(ProtocolType::Iterm2),
        "4" => picker.set_protocol_type(ProtocolType::Sixel),
        "5" => picker.set_protocol_type(ProtocolType::Halfblocks),
        _ => {
             picker = Picker::from_query_stdio().unwrap_or(Picker::from_fontsize(font_size));
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load an image with the image crate.
    let dyn_img = image::ImageReader::open("../prtgn_logo.ico")?.decode()?;

    // Create the Protocol which will be used by the widget.
    let image = picker.new_resize_protocol(dyn_img);

    let mut app = App { image };

    // Run the app loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    // The image widget.
    let image = StatefulImage::default();
    // Render with the protocol state.
    f.render_stateful_widget(image, f.area(), &mut app.image);
}
