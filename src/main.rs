mod chess;
mod ui;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use chess::{GameState, Position};
use ui::{board_text, prefers_unicode_symbols};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug)]
pub struct App {
    game: GameState,
    cursor: Position,
    selected: Option<Position>,
    use_unicode_symbols: bool,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: GameState::default(),
            cursor: Position::zero(),
            selected: None,
            use_unicode_symbols: prefers_unicode_symbols(),
            exit: false,
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('h') => self.move_cursor(0, -1),
            KeyCode::Right | KeyCode::Char('l') => self.move_cursor(0, 1),
            KeyCode::Up | KeyCode::Char('k') => self.move_cursor(-1, 0),
            KeyCode::Down | KeyCode::Char('j') => self.move_cursor(1, 0),
            KeyCode::Enter | KeyCode::Char(' ') => self.confirm_selection(),
            KeyCode::Esc => self.clear_selection(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_cursor(&mut self, row_delta: isize, col_delta: isize) {
        self.cursor.0 = self.cursor.0.saturating_add_signed(row_delta).min(7);
        self.cursor.1 = self.cursor.1.saturating_add_signed(col_delta).min(7);
    }

    fn confirm_selection(&mut self) {
        match self.selected {
            Some(from) if from == self.cursor => self.selected = None,
            Some(from) => {
                self.game.move_piece(from, self.cursor);
                self.selected = None;
            }
            None => {
                self.selected = Some(self.cursor);
            }
        }
    }

    fn clear_selection(&mut self) {
        self.selected = None;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Chess ".bold());
        let instructions = Line::from(vec![
            " Move ".into(),
            "<Arrows/HJKL>".blue().bold(),
            " Select/Drop ".into(),
            "<Enter/Space>".blue().bold(),
            " Clear ".into(),
            "<Esc>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = board_text(
            &self.game.board,
            self.cursor,
            self.selected,
            self.use_unicode_symbols,
        );

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
