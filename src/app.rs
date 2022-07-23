use std::fs;
use std::process::Command;

use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

use crate::{
    shows::{new_show::NewShow, ShowInterface},
    util, VERSION,
};

#[derive(PartialEq, Eq)]
pub enum SelectedElement {
    MainList,
}

pub struct App {
    shows: Vec<Box<dyn ShowInterface>>,
    select_index: usize,
    selected_element: SelectedElement,
}

impl App {
    pub fn new() -> Self {
        let new_shows = fs::read_dir("D:\\Movies")
            .unwrap()
            .map(|x| x.unwrap())
            .map(|x| NewShow::new(x.file_name().to_string_lossy().to_string(), x.path()))
            .map(|x| Box::new(x) as Box<dyn ShowInterface>)
            .collect::<Vec<_>>();

        App {
            shows: new_shows,
            select_index: 0,
            selected_element: SelectedElement::MainList,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        loop {
            terminal.draw(|frame| self.ui(frame)).unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => return,
                    KeyCode::Char('w') => {
                        if let Some(file) =
                            util::find_movie_file(self.shows[self.select_index].path())
                        {
                            Command::new("mpv")
                                .args([
                                    "--ontop",
                                    &file.canonicalize().unwrap().to_string_lossy().to_string(),
                                ])
                                .spawn()
                                .unwrap();
                        }
                    }
                    _ => {}
                }

                if self.selected_element == SelectedElement::MainList {
                    match key.code {
                        KeyCode::Down => {
                            self.select_index = (self.select_index + 1).min(self.shows.len() - 1)
                        }
                        KeyCode::Up => self.select_index = self.select_index.saturating_sub(1),
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();

        let block = Block::default();
        f.render_widget(block, size);

        let inner_blocks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(size);

        let upper_block = Block::default()
            .borders(Borders::ALL)
            .title(vec![
                Span::from("━ "),
                Span::styled(
                    "Movie Manager",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" [V{}] ", VERSION),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Magenta),
                ),
                Span::styled(
                    "Connor Slade ",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Blue),
                ),
            ])
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Thick);

        let lower_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        f.render_widget(lower_block, inner_blocks[1]);

        let main_items = self
            .shows
            .iter()
            .map(|x| ListItem::new(x.name()))
            .collect::<Vec<_>>();
        let mut main_list_state = ListState::default();
        main_list_state.select(Some(self.select_index));
        let main_list = List::new(main_items)
            .block(upper_block)
            .style(Style::default().fg(Color::Gray))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("❯ ");
        f.render_stateful_widget(main_list, inner_blocks[0], &mut main_list_state);
    }
}
