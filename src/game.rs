use crate::Grid;
use crate::grid::{ROWS, COLS, Tile};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier,Style},
    text::{Span, Line},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};


pub fn render_game(frame: &mut Frame,grid:&mut Grid){
    let size = frame.size();

    let main_block = Block::default()
        .title("Connect 4")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .border_style(Style::default().fg(Color::White));
    
    let area = render_main_terminal(size, frame,grid);

    grid_area_render(frame, main_block.inner(area),grid);

    frame.render_widget(main_block, area);
}

// Render all the widget for the game of connect 4
pub fn render_main_terminal(
        rect: Rect,
        frame: &mut Frame,
        grid: &mut Grid,
    ) -> Rect{
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                Constraint::Percentage(3),
                Constraint::Percentage(87),
                Constraint::Percentage(10)
        ].as_ref())
        .split(rect);

    let upper_text = if grid.current_winner == Tile::NONE {
        if grid.is_game_end() {
            format!("It is a draw")
        }else {
            format!("Player Turn {}", if grid.current_player == Tile::YELLOW{
                "Yellow"
                    
            }else {
                "Red"
            }) 
        }
    } else {
        format!("Winner is {}", if grid.current_winner == Tile::YELLOW{
                "Yellow"
                    
            }else {
                "Red"
            })
    };

    let bottom_text = if grid.is_game_end(){
        vec![
        Line::from(Span::raw(" Play again: r ")),
        Line::from(Span::raw(" Quit: q ")),
        ]
    } else {
        vec![
        Line::from(Span::raw(" Move piece column: ← → ")),
        Line::from(Span::raw(" Put a piece: ENTER/SPACE ")),
        Line::from(Span::raw(" quit: q ")),
        ]
    };

    let player_turn =
        Paragraph::new(upper_text)
            .alignment(Alignment::Center);

    frame.render_widget(player_turn, layout[0]);

    let command = 
        Paragraph::new(bottom_text)
            .alignment(Alignment::Center);

    frame.render_widget(command, layout[2]);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                //Constraint::Percentage(1),
                Constraint::Percentage(100),
                //Constraint::Percentage(1)
        ].as_ref())
        .split(layout[1])[0]
}

// Render the Grid of the connect 4
pub fn grid_area_render(
    frame: &mut Frame,
    rect: Rect,
    grid: &mut Grid,
    ){
    
    let layout_grid_vertical =
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(rect.height/ROWS); ROWS.into()])
            .split(rect);

    let mut is_chosen = true;
    for (row_id, row) in layout_grid_vertical.iter().enumerate(){
        let layout_grid_horinzontal = 
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(rect.width/ COLS);COLS.into()])
                .split(*row);

    
        for (col_id, col) in layout_grid_horinzontal.iter().enumerate().rev(){
            let case = grid.grid[row_id][col_id].clone();
            let mut case_color = Color::LightBlue;
            // Look at the last tile we can put
            if is_chosen 
                && case == Tile::NONE 
                && col_id == grid.current_cursor.into()
                && (
                    row_id == 0 ||
                    grid.grid[row_id-1][col_id].clone() != Tile::NONE
                )
            {
                case_color = Color::Yellow;
                is_chosen = false;
            }

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(case_color));

            let text_case = grid.tile_to_ascii(case.clone());

            let widget = Paragraph::new(text_case)
                .block(block.clone())
                .alignment(Alignment::Center)
                .style(Style::default().fg( if case == Tile::YELLOW {
                        Color::Yellow
                    }else{
                        Color::Red
                    }));

            frame.render_widget(widget, *col);
            frame.render_widget(block, *col);
        }
    }
}


