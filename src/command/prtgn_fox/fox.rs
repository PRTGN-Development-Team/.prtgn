pub fn fox(rerun: bool) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = reqwest::Client::new();

    let font_size = (8, 12);
    let mut forced_protocol = None;

    if !rerun {
        println!("Select rendering backend:");
        println!("1. Auto-detect");
        println!("2. Kitty");
        println!("3. Iterm2");
        println!("4. Sixel");
        println!("5. Halfblocks");
        print!("Enter choice (default 1): ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let choice = input.trim();
            match choice {
                "2" => forced_protocol = Some(ProtocolType::Kitty),
                "3" => forced_protocol = Some(ProtocolType::Iterm2),
                "4" => forced_protocol = Some(ProtocolType::Sixel),
                "5" => forced_protocol = Some(ProtocolType::Halfblocks),
                _ => {}
            }
        }
    }

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut picker = Picker::from_query_stdio().unwrap_or(Picker::from_fontsize(font_size));
    if let Some(protocol) = forced_protocol {
        picker.set_protocol_type(protocol);
    }

    let mut error_msg: Option<String> = None;

    loop {
        let image_data = match rt.block_on(fetch(&client)) {
            Ok(data) => data,
            Err(e) => {
                error_msg = Some(format!("Error: {}", e));
                break;
            }
        };

        terminal.clear().unwrap();

        match render(&mut terminal, &image_data, &mut picker) {
            Ok(should_reload) => {
                if !should_reload {
                    break;
                }
            },
            Err(e) => {
                error_msg = Some(format!("Render error: {}", e));
                break;
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();

    if let Some(msg) = error_msg {
        eprintln!("{}", msg);
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch(client: &reqwest::Client) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let url = "https://api.fox.pics/v1/get-random-foxes?amount=1";

    //eprintln!("Fetching {url:?}...");

    let res = client.get(url).send().await?;

    // eprintln!("Response: {:?} {}", res.version(), res.status());
    // eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    let mut chars = body.chars();
    chars.next();
    chars.next_back();
    chars.next();
    chars.next_back();


    // println!("{}", chars.as_str().to_string());

    let image_data = download(client, chars.as_str().to_string()).await?;

    Ok(image_data)


}

async fn download(client: &reqwest::Client, url: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let res = client.get(&url).send().await?;
    let bytes = res.bytes().await?;
    Ok(bytes.to_vec())
}


// -------------------------------------------------------
// -------------------------------------------------------
// -------------------------------------------------------
// -------------------------------------------------------



use ratatui::{
    backend::CrosstermBackend,
    Terminal, Frame,
    widgets::Block
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

pub fn render(terminal: &mut Terminal<CrosstermBackend<Stdout>>, image_data: &[u8], picker: &mut Picker) -> Result<bool, Box<dyn std::error::Error>> {

    // Load an image with the image crate.
    let dyn_img = image::load_from_memory(image_data)?;

    // Create the Protocol which will be used by the widget.
    let image = picker.new_resize_protocol(dyn_img);

    let mut app = App { image };

    // Run the app loop
    let res = run_app(terminal, &mut app);

    match res {
        Ok(reload) => Ok(reload),
        Err(err) => {
            println!("{:?}", err);
            Ok(false)
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(false);
                }
                if key.code == KeyCode::Char('r') {
                    return Ok(true);
                }
            }
        }
    }
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    f.render_widget(Block::default(), f.area());
    // The image widget.
    let image = StatefulImage::default();
    // Render with the protocol state.
    f.render_stateful_widget(image, f.area(), &mut app.image);
}
