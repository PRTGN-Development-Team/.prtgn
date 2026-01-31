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

use crate::command::prtgn_wiki::search;

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
    status: Status,
    wiki: String,
    info: String,
    url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Unselected,
    Selected,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            wiki_list: WikiList::from_iter([
                (
                    Status::Unselected,
                    "Ivycomb Wiki | MediaWiki | wiki.ivy.cm",
                    "This wiki serves as the de-facto source for proven information (and occasional theories) on ivycomb's various projects, ranging from music, to Cosmic Critters and Antihuman, to general character and meta information. \n\nThis wiki is officially endorsed by and owned by ivycomb, but pages are edited and maintained by the community with limited input from the original creators.",
                    "https://wiki.ivy.cm/w/api.php?",
                ),
                (
                    Status::Unselected,
                    "Alterhumanity Wiki | Fandom | alterhumanity.fandom.com",
                    "Alterhumanity is a fandom wiki created by Humanwingz, This fandom wiki was created for the purpose of explaining Alterhuman terms, creating new Alterhuman terms, and created to shed light on Alterhumanity as a whole to the public.",
                    "https://alterhumanity.fandom.com/api.php?",
                ),
                (
                    Status::Unselected,
                    "Alterhuman Wiki | MediaWiki | alterhuman.miraheze.org",
                    "This is a wiki for all things alterhuman, or alternative to the common societal idea of humanity. Alterhuman includes, but is not limited to, therianthropy, otherkin, fictionkin, otherhearted, otherlink, plurality, and more. \n\nThis wik is dedicated to storing information on various alterhuman labels and the history of the alterhuman community as well as alterhuman symbols, flags, and other imagery. Alterhuman-adjacent topics are also welcome.",
                    "https://alterhuman.miraheze.org/w/api.php?",
                ),
                (
                    Status::Unselected,
                    "LGBTQIA+ Wiki | MediaWiki | lgbtqia.wiki",
                    "The LGBTQIA+ Wiki is a resource of LGBTQIA+ terminology and labels used by various queer communities, as well as the questioning and/or curious. The wiki is designed to be a helpful resource for explaining identities that are often unknown, unheard of, or difficult to find information for.",
                    "https://lgbtqia.wiki/w/api.php",
                ),
                (
                    Status::Unselected,
                    "New LGBTQIA+ Wiki | MediaWiki | new.lgbtqia.wiki",
                    "The LGBTQIA+ Wiki is a resource of LGBTQIA+ terminology and labels used by various queer communities, as well as the questioning and/or curious. The wiki is designed to be a helpful resource for explaining identities both known and unknown.",
                    "https://new.lgbtqia.wiki/w139/api.php?",
                ),
                (
                    Status::Unselected,
                    "Wikipedia | MediaWiki | wikipedia.org",
                    "The free encyclopedia that anyone can edit.",
                    "https://en.wikipedia.org/w/api.php",
                ),
                (
                    Status::Unselected,
                    "Wiktionary | MediaWiki | wiktionary.org",
                    "Welcome to the English-language Wiktionary, a collaborative project to produce a free-content multilingual dictionary. It aims to describe all words of all languages using definitions and descriptions in English.",
                    "https://en.wiktionary.org/w/api.php",
                ),
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str, &'static str)> for WikiList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str, &'static str)>>(iter: I) -> Self {
        let mut items: Vec<WikiItem> = iter
            .into_iter()
            .map(|(status, wiki, info, url)| WikiItem::new(status, wiki, info, url))
            .collect();
        items.sort_by(|a, b| a.wiki.cmp(&b.wiki));
        let state = ListState::default();
        Self { items, state }
    }
}

impl WikiItem {
    fn new(status: Status, wiki: &str, info: &str, url: &str) -> Self {
        Self {
            status,
            wiki: wiki.to_string(),
            info: info.to_string(),
            url:  url.to_string(),
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
            KeyCode::Char('a') | KeyCode::Left => self.select_none(),
            KeyCode::Char('s') | KeyCode::Down => self.select_next(),
            KeyCode::Char('w') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('d') | KeyCode::Right | KeyCode::Enter => {
                self.select();
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
    fn select(&mut self) {
        // if let Some(i) = self.wiki_list.state.selected() {
        //     self.wiki_list.items[i].status = match self.wiki_list.items[i].status {
        //         Status::Selected => Status::Unselected,
        //         Status::Unselected => Status::Selected,
        //     }
        // }

        // fetch(self.wiki_list.items[0].url.clone());

        search().unwrap();
    }
}

























// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------








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
                Status::Selected => format!("{}", self.wiki_list.items[i].info),
                Status::Unselected => format!("{}", self.wiki_list.items[i].info),
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
            Status::Unselected => Line::styled(format!(" ☐ {}", value.wiki), TEXT_FG_COLOR),
            Status::Selected => {
                Line::styled(format!(" ✓ {}", value.wiki), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}