use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
    Frame,
};
use std::io::{stdout, Result};
use grid::Grid;
use game::render_game;
mod grid;
mod game;

fn main() -> Result<()> {
    //let mut stdout = stdout();
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut grid = Grid::new();

    // TODO main loop
    loop {
        // TODO draw the UI
        terminal.draw(|frame: &mut Frame| {
            render_game(frame, &mut grid)
        })?;
        // TODO handle events
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press{
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => grid.change_col(-1),
                    KeyCode::Right => grid.change_col(1),
                    KeyCode::Enter | KeyCode::Char(' ')
                        => if !grid.is_game_end(){
                        grid.drop_piece(grid.current_cursor)
                        },
                    KeyCode::Char('r') => grid.reload_grid(),
                    _ => {},
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
