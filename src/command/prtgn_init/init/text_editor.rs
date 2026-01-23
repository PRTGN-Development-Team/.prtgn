use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph};
use ratatui::Terminal;
use std::borrow::Cow;
use std::{env};
use std::io::{self};
use std::path::PathBuf;
use tui_textarea::{Input, Key, TextArea};
use prtgn_encoding::{read, write};

macro_rules! error {
    ($fmt: expr $(, $args:tt)*) => {{
        Err(io::Error::new(io::ErrorKind::Other, format!($fmt $(, $args)*)))
    }};
}

struct Buffer<'a> {
    textarea: TextArea<'a>,
    path: PathBuf,
    modified: bool,
}

impl Buffer<'_> {
    fn new(path: PathBuf) -> io::Result<Self> {
        let mut textarea = if let Ok(md) = path.metadata() {
            if md.is_file() {
                let decrypted_content = read(path.to_string_lossy().to_string()).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
                let mut textarea: TextArea = decrypted_content.lines().map(String::from).collect();

                if textarea.lines().iter().any(|l| l.starts_with('\t')) {
                    textarea.set_hard_tab_indent(true);
                }
                textarea
            } else {
                return error!("{:?} is not a file", path);
            }
        } else {
            TextArea::default() // File does not exist
        };
        textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
        let path = if path.is_absolute() {
            path
        } else {
            env::current_dir()?.join(path)
        };
        Ok(Self {
            textarea,
            path,
            modified: false,
        })
    }
}

struct Editor<'a> {
    current: usize,
    buffers: Vec<Buffer<'a>>,
    term: Terminal<CrosstermBackend<io::Stdout>>,
    message: Option<Cow<'static, str>>,
}

impl Editor<'_> {
    fn new(filename: String) -> io::Result<Self>
    {
        let buffers = filename
            .split_whitespace()
            .map(|f| Buffer::new(f.into()))
            .collect::<io::Result<Vec<_>>>()?;
        if buffers.is_empty() {
            return error!("USAGE: prtgn  init <filename>");
        }
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        crossterm::execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let term = Terminal::new(backend)?;
        Ok(Self {
            current: 0,
            buffers,
            term,
            message: None,
        })
    }

    fn run(&mut self) -> io::Result<()> {
        loop {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints::<&[Constraint]>(
                    [
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                        .as_ref(),
                );

            self.term.draw(|f| {
                let chunks = layout.split(f.area());

                let buffer = &self.buffers[self.current];
                let textarea = &buffer.textarea;
                f.render_widget(textarea, chunks[0]);

                // Render status line
                let modified = if buffer.modified { " [modified]" } else { "" };
                let slot = format!("[{}/{}]", self.current + 1, self.buffers.len());
                let path = format!(" {}{} ", buffer.path.display(), modified);
                let (row, col) = textarea.cursor();
                let cursor = format!("({},{})", row + 1, col + 1);
                let status_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints::<&[Constraint]>(
                        [
                            Constraint::Length(slot.len() as u16),
                            Constraint::Min(1),
                            Constraint::Length(cursor.len() as u16),
                        ]
                            .as_ref(),
                    )
                    .split(chunks[1]);
                let status_style = Style::default().add_modifier(Modifier::REVERSED);
                f.render_widget(Paragraph::new(slot).style(status_style), status_chunks[0]);
                f.render_widget(Paragraph::new(path).style(status_style), status_chunks[1]);
                f.render_widget(Paragraph::new(cursor).style(status_style), status_chunks[2]);

                // Render message at bottom
                let message = if let Some(message) = &self.message {
                    Line::from(Span::raw(message.as_ref()))
                } else {
                    Line::from(vec![
                        Span::raw("Press "),
                        Span::styled("^Q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to quit, "),
                        Span::styled("^S", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to save, "),
                        Span::styled("^T", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to switch buffer"),
                    ])
                };
                f.render_widget(Paragraph::new(message), chunks[2]);
            })?;

            self.message = None;
            match event::read()? {
                Event::Key(key) => {
                    let input: Input = key.into();
                    match input {
                        Input {
                            key: Key::Char('q'),
                            ctrl: true,
                            ..
                        } => break,
                        Input {
                            key: Key::Char('t'),
                            ctrl: true,
                            ..
                        } => {
                            self.current = (self.current + 1) % self.buffers.len();
                            self.message =
                                Some(format!("Switched to buffer #{}", self.current + 1).into());
                        }
                        Input {
                            key: Key::Char('s'),
                            ctrl: true,
                            ..
                        } => {
                            let buffer = &mut self.buffers[self.current];
                            if buffer.modified {
                                let prtgn_text = buffer.textarea.lines().join("\n");
                                write(buffer.path.to_string_lossy().to_string(), prtgn_text).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
                                buffer.modified = false;
                                self.message = Some("Saved!".into());
                            } else {
                                self.message = Some("No changes to save".into());
                            }
                        }
                        input => {
                            let buffer = &mut self.buffers[self.current];
                            buffer.modified |= buffer.textarea.input(input);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Drop for Editor<'_> {
    fn drop(&mut self) {
        self.term.show_cursor().unwrap();
        disable_raw_mode().unwrap();
        crossterm::execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen
        )
            .unwrap();
    }
}

pub fn editor(filename: String) -> io::Result<()> {
    Editor::new(filename.into())?.run()
}
