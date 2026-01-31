/*
 * .prtgn Copyright (C) 2026 PRTGN Development Team
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.
 */

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};

const WIKI_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub fn selection() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

/// This struct holds the current state of the app. In particular, it has the `wiki_list` field
/// which is a wrapper around `ListState`. Keeping track of the state lets us render the
/// associated widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events. Check
/// the drawing logic for items on how to specify the highlighting style for selected items.
struct App {
    should_exit: bool,
    wiki_list: WikiList,
}

struct WikiList {
    items: Vec<WikiItem>,
    state: ListState,
}

#[derive(Debug)]
struct WikiItem {
    wiki: String,
    info: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Wiki,
    Completed,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            wiki_list: WikiList::from_iter([
                (
                    Status::Wiki,
                    "Rewrite everything with Rust!",
                    "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust",
                ),
                (
                    Status::Completed,
                    "Rewrite all of your tui apps with Ratatui",
                    "Yes, you heard that right. Go and replace your tui with Ratatui.",
                ),
                (
                    Status::Wiki,
                    "Pet your cat",
                    "Minnak loves to be pet by you! Don't forget to pet and give some treats!",
                ),
                (
                    Status::Wiki,
                    "Walk with your dog",
                    "Max is bored, go walk with him!",
                ),
                (
                    Status::Completed,
                    "Pay the bills",
                    "Pay the train subscription!!!",
                ),
                (
                    Status::Completed,
                    "Refactor list example",
                    "If you see this info that means I completed this task!",
                ),
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for WikiList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, wiki, info)| WikiItem::new(status, wiki, info))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl WikiItem {
    fn new(status: Status, wiki: &str, info: &str) -> Self {
        Self {
            status,
            wiki: wiki.to_string(),
            info: info.to_string(),
        }
    }
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.toggle_status();
            }
            _ => {}
        }
    }

    const fn select_none(&mut self) {
        self.wiki_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.wiki_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.wiki_list.state.select_previous();
    }

    const fn select_first(&mut self) {
        self.wiki_list.state.select_first();
    }

    const fn select_last(&mut self) {
        self.wiki_list.state.select_last();
    }

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.wiki_list.state.selected() {
            self.wiki_list.items[i].status = match self.wiki_list.items[i].status {
                Status::Completed => Status::Wiki,
                Status::Wiki => Status::Completed,
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [header_area, content_area, footer_area] = area.layout(&main_layout);

        let content_layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]);
        let [list_area, item_area] = content_area.layout(&content_layout);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Available Wiki Sites")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Wiki List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(WIKI_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .wiki_list
            .items
            .iter()
            .enumerate()
            .map(|(i, wiki_item)| {
                let color = alternate_colors(i);
                ListItem::from(wiki_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.wiki_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.wiki_list.state.selected() {
            match self.wiki_list.items[i].status {
                Status::Completed => format!("✓ DONE: {}", self.wiki_list.items[i].info),
                Status::Wiki => format!("☐ WIKI: {}", self.wiki_list.items[i].info),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Wiki Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(WIKI_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&WikiItem> for ListItem<'_> {
    fn from(value: &WikiItem) -> Self {
        let line = match value.status {
            Status::Wiki => Line::styled(format!(" ☐ {}", value.wiki), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.wiki), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}