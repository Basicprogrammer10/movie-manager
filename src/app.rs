use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

use crate::VERSION;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run<B: Backend>(&self, terminal: &mut Terminal<B>) {
        loop {
            terminal.draw(|frame| self.ui(frame)).unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => return,
                    _ => {}
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

        let block = Block::default()
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
        f.render_widget(block, inner_blocks[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        f.render_widget(block, inner_blocks[1]);
    }
}
