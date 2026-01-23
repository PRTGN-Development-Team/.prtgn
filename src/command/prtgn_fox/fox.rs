use colored::Colorize;
use std::io::{self, Write};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Paragraph},
    layout::{Layout, Constraint, Direction, Alignment},
};
use ratatui_image::{picker::{Picker, ProtocolType}, StatefulImage};
use ratatui_image::thread::{ResizeRequest, ThreadProtocol};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, unbounded_channel, UnboundedSender},
    time::{interval, Duration},
};

pub fn fox(rerun: bool) {
    let mut forced_protocol = None;

    if !rerun {
        println!("{}", "----------------------------------------------".color("red").bold());
        println!("Press '{}' to quit. Press '{}' to get new image.", "q".color("blue"), "r".color("blue"));
        println!("{}", "----------------------------------------------".color("red").bold());
        println!("Select rendering backend:");
        println!("{} {}", "1.".color("cyan").bold(), "Auto-detect".color("magenta"));
        println!("{} {}", "2.".color("cyan").bold(), "Kitty".color("magenta"));
        println!("{} {}", "3.".color("cyan").bold(), "Iterm2".color("magenta"));
        println!("{} {}", "4.".color("cyan").bold(), "Sixel".color("magenta"));
        println!("{} {}", "5.".color("cyan").bold(), "Halfblocks".color("magenta"));
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

    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(async_fox(forced_protocol)) {
        eprintln!("Error: {}", e);
    }
}

async fn async_fox(forced_protocol: Option<ProtocolType>) -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();

    let client = reqwest::Client::new();
    let mut picker = Picker::from_query_stdio().unwrap_or(Picker::halfblocks());
    if let Some(protocol) = forced_protocol {
        picker.set_protocol_type(protocol);
    }

    // Channel for ResizeRequests (std::sync::mpsc for ThreadProtocol compatibility)
    let (resize_tx, resize_rx) = std::sync::mpsc::channel();
    // Channel for main loop to receive ResizeRequests (tokio::sync::mpsc)
    let (tx, rx) = unbounded_channel();

    // Bridge std channel to tokio channel
    let tx_bridge = tx.clone();
    std::thread::spawn(move || {
        while let Ok(req) = resize_rx.recv() {
            if tx_bridge.send(req).is_err() {
                break;
            }
        }
    });

    let (event_tx, event_rx) = unbounded_channel();

    // Spawn a blocking task to read events
    tokio::task::spawn_blocking(move || {
        loop {
            if let Ok(event) = event::read() {
                if event_tx.send(event).is_err() {
                    break;
                }
            }
        }
    });

    // Channel for internal app actions
    let (action_tx, action_rx) = unbounded_channel();

    let thread_protocol = ThreadProtocol::new(resize_tx.clone(), None);

    let mut app = App {
        running: true,
        protocol: thread_protocol,
        event_rx,
        rx,
        action_rx,
        action_tx,
        resize_tx,
        client,
        picker,
        loading: true,
        spinner_index: 0,
    };

    app.reload();

    let res = app.run(&mut terminal).await;

    ratatui::restore();

    res
}

enum Action {
    ImageLoaded(Vec<u8>),
    LoadError(String),
}

struct App {
    running: bool,
    protocol: ThreadProtocol,
    event_rx: UnboundedReceiver<Event>,
    rx: UnboundedReceiver<ResizeRequest>,
    action_rx: UnboundedReceiver<Action>,
    action_tx: UnboundedSender<Action>,
    resize_tx: std::sync::mpsc::Sender<ResizeRequest>,
    client: reqwest::Client,
    picker: Picker,
    loading: bool,
    spinner_index: usize,
}

impl App {
    async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = interval(Duration::from_millis(100));
        while self.running {
            terminal.draw(|f| self.ui(f))?;

            select! {
                Some(event) = self.event_rx.recv() => self.handle_event(event).await?,
                Some(request) = self.rx.recv() => self.handle_request(request)?,
                Some(action) = self.action_rx.recv() => self.handle_action(action)?,
                _ = interval.tick() => {
                    if self.loading {
                        self.spinner_index = (self.spinner_index + 1) % 10;
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.running = false,
                    KeyCode::Char('r') => {
                        if !self.loading {
                            self.reload();
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_request(&mut self, request: ResizeRequest) -> Result<(), Box<dyn std::error::Error>> {
        self.protocol.update_resized_protocol(request.resize_encode()?);
        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            Action::ImageLoaded(data) => {
                let dyn_img = image::load_from_memory(&data)?;
                let protocol = self.picker.new_resize_protocol(dyn_img);
                self.protocol = ThreadProtocol::new(self.resize_tx.clone(), Some(protocol));
                self.loading = false;
            }
            Action::LoadError(_e) => {
                self.loading = false;
                // In a real app, we might want to show this error in the UI
                // For now, we just print it to stderr which might be hidden or mess up TUI
                // But since we are in TUI mode, we should probably not print to stderr.
                // We'll just ignore it for now or maybe log it if we had a logger.
            }
        }
        Ok(())
    }

    fn reload(&mut self) {
        self.loading = true;
        let tx = self.action_tx.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            match fetch(&client).await {
                Ok(data) => { let _ = tx.send(Action::ImageLoaded(data)); }
                Err(e) => { let _ = tx.send(Action::LoadError(e.to_string())); }
            }
        });
    }

    fn ui(&mut self, f: &mut Frame) {
        f.render_widget(Block::default(), f.area());

        if self.loading {
            let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let text = format!("Loading... {}", spinner[self.spinner_index]);
            let area = f.area();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Min(1),
                    Constraint::Percentage(50),
                ])
                .split(area);

            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center);

            f.render_widget(paragraph, layout[1]);
        } else {
            let image = StatefulImage::default();
            f.render_stateful_widget(image, f.area(), &mut self.protocol);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch(client: &reqwest::Client) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = "https://api.fox.pics/v1/get-random-foxes?amount=1";
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    let mut chars = body.chars();
    chars.next();
    chars.next_back();
    chars.next();
    chars.next_back();

    let image_data = download(client, chars.as_str().to_string()).await?;
    Ok(image_data)
}

async fn download(client: &reqwest::Client, url: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let res = client.get(&url).send().await?;
    let bytes = res.bytes().await?;
    Ok(bytes.to_vec())
}
