use std::io::{stdout, Write};

use crate::Display;
use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal::{self, ClearType}, QueueableCommand, ExecutableCommand
};

impl Display {
    pub fn header(&self) {
        let mut stdout = stdout();

        let hor_char = "─";
        let vert_char = "│";

        let msg = " Namaste from Ludo-The_Game 🙏 ";
        let (columns, rows) = match terminal::size() {
            Ok(size) => (size.0 as usize, size.1 as usize),
            Err(e) => panic!("{:?}", e),
        };

        stdout.queue(terminal::Clear(ClearType::All));

        // START: Header
        let left_spacing = (columns - msg.len()) / 2;
        stdout
            .queue(cursor::MoveTo(1, 0)).unwrap()
            .queue(style::Print(format!("┌{}┐", hor_char.repeat(columns - 3)))).unwrap()
            .queue(cursor::MoveTo(1,1)).unwrap()
            .queue(style::Print(vert_char)).unwrap()
            .queue(cursor::MoveToColumn(left_spacing as u16)).unwrap()
            .queue(style::PrintStyledContent(
                msg.with(Color::Rgb {
                    r: 214,
                    g: 214,
                    b: 214,
                })
                .bold(),
            )).unwrap()
            .queue(cursor::MoveToColumn(columns as u16)).unwrap()
            .queue(style::Print(vert_char)).unwrap()
            .queue(cursor::MoveTo(1,2)).unwrap()
            .queue(style::Print(format!("└{}┘", hor_char.repeat(columns-3)))).unwrap()
            .queue(cursor::MoveToNextLine(1)).unwrap();
        // END: Header
    }

    pub fn board_design(&self, h_scale: u16, v_scale: u16) -> ((u16,u16),(u16,u16)) {
        // START: Board Design
        let mut stdout = stdout();
        let hor_char = "─";
        let vert_char = "│";

        let (columns, rows) = match terminal::size() {
            Ok(size) => (size.0 as usize, size.1 as usize),
            Err(e) => panic!("{:?}", e),
        };

        let left_spacing: u16 = (columns as u16 - (15 * (h_scale) + 1)) / 2;

        // Some gap between Header and Board
        stdout
            .queue(cursor::MoveToColumn(left_spacing as u16 + 1 + (h_scale / 2))).unwrap();

        for i in 0..15 {
            stdout
                .queue(style::Print((i as u8).to_string()))
                .unwrap()
                .queue(style::Print(
                    " ".repeat(h_scale as usize - (if i > 9 { 1 } else { 0 })),
                ))
                .unwrap();
        }

        stdout.queue(cursor::MoveToNextLine(1)).unwrap();

        let board_start_row = cursor::position().unwrap().1;
        let board_start_col = left_spacing as u16;

        stdout
            .queue(cursor::MoveToColumn(left_spacing)).unwrap()
            .queue(style::Print(
                hor_char.repeat(15 * (h_scale + 1) as usize + 1)
            )).unwrap()
            .queue(cursor::MoveToNextLine(1)).unwrap();

        for i in 0..15 {
            // Left spacing of each row
            stdout
                .queue(cursor::MoveToColumn(left_spacing - 3)).unwrap()
                .queue(style::Print(format!(
                    "{} {}",
                    (i as u8).to_string() + if i < 10 { " " } else { "" },
                    vert_char
                ))).unwrap();

            // printing vertical line after each cell (empty)
            for _j in 0..15 {
                stdout
                    .queue(style::Print(format!(
                        "{}{}",
                        " ".repeat(h_scale.into()),
                        vert_char
                    ))).unwrap();
            }

            // Bottom Line of each row
            stdout
                .queue(cursor::MoveToNextLine(1)).unwrap()
                .queue(cursor::MoveToColumn(left_spacing)).unwrap()
                .queue(style::Print(hor_char.repeat(15 * (h_scale + 1) as usize + 1))).unwrap()
                .queue(cursor::MoveToNextLine(1)).unwrap();
        }

        let (board_end_col, board_end_row) = cursor::position().unwrap();

        // START: Colour Boxes
        self.color_boxes(
            board_start_col,
            board_start_row,
            h_scale,
            v_scale,
            9,
            15,
            0,
            6,
            Color::Red,
        );
        self.color_boxes(
            board_start_col,
            board_start_row,
            h_scale,
            v_scale,
            0,
            6,
            0,
            6,
            Color::Green,
        );
        self.color_boxes(
            board_start_col,
            board_start_row,
            h_scale,
            v_scale,
            0,
            6,
            9,
            15,
            Color::Yellow,
        );
        self.color_boxes(
            board_start_col,
            board_start_row,
            h_scale,
            v_scale,
            9,
            15,
            9,
            15,
            Color::Blue,
        );
        self.color_boxes(
            board_start_col,
            board_start_row,
            h_scale,
            v_scale,
            6,
            9,
            6,
            9,
            Color::White,
        );
        // END: Colour Boxes        
        // END: Board Design

        ((board_start_col, board_start_row), (board_end_col, board_end_row))
    }

    pub fn splash_screen(message: &str, colour: Option<Color>) {
        let hor_char = "─";
        let vert_char = "│";

        let (columns, rows) = terminal::size().unwrap();
        let min_side_margin = 8;
        let original_position = cursor::position().unwrap();

        let mut stdout = stdout();

        stdout.execute(terminal::Clear(ClearType::All));

        stdout
            .queue(cursor::MoveTo(1, 0))
            .unwrap()
            .queue(style::Print(format!(
                "┌{}┐",
                hor_char.repeat(columns as usize - 3)
            )))
            .unwrap();

        for i in 1..rows {
            stdout
                .queue(cursor::MoveTo(1, i))
                .unwrap()
                .queue(style::Print(vert_char))
                .unwrap()
                .queue(cursor::MoveToColumn(columns))
                .unwrap()
                .queue(style::Print(vert_char))
                .unwrap()
                .queue(cursor::MoveToNextLine(1))
                .unwrap();
        }

        stdout
            .queue(cursor::MoveTo(1, rows - 1))
            .unwrap()
            .queue(style::Print(format!(
                "└{}┘",
                hor_char.repeat(columns as usize - 3)
            )))
            .unwrap();

        stdout
            .queue(cursor::MoveTo(
                ((columns as usize - message.len()) as u16) / 2,
                rows / 2,
            ))
            .unwrap();
        // Print Message
        match colour {
            Some(colour) => stdout.queue(style::PrintStyledContent(message.bold().with(colour))),
            None => stdout.queue(style::PrintStyledContent(message.bold())),
        }
        .unwrap();

        // Restore Cursor Position
        stdout.queue(cursor::MoveTo(original_position.0, original_position.1));

        stdout.flush();
    }

    pub fn update_according_to_ludo_board(
        &self,
        board_start_col: u16,
        board_start_row: u16,
        h_scale: u16,
        v_scale: u16,
    ) {
        // START: Board Content
        let mut stdout = stdout();

        for (r, row) in self.board.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                // Inner Square
                if r >= 6 && r <= 9 && c >= 6 && c <= 9 {
                    continue;
                }

                // Red Square
                if r >= 9 && c <= 5 {
                    if !(((r,c) == (10,1)) || ((r,c) == (10,4)) || ((r,c) == (13,1)) || ((r,c) == (13,4))) {
                        continue;
                    }
                }

                // Green Square
                if r <= 5 && c <= 5 {
                    if !(((r,c) == (1,1)) || ((r,c) == (1,4)) || ((r,c) == (4,1)) || ((r,c) == (4,4))) {
                        continue;
                    }
                }

                // Yellow Square
                if r <= 5 && c >= 9 {
                    if !(((r,c) == (1,10)) || ((r,c) == (4,10)) || ((r,c) == (1,13)) || ((r,c) == (4,13))) {
                        continue;
                    }
                }

                // Blue Square
                if r >= 9 && c >= 9 {
                    if !(((r,c) == (10,10)) || ((r,c) == (10,13)) || ((r,c) == (13,10)) || ((r,c) == (13,13))) {
                        continue;
                    }
                }
                
                stdout
                    .queue(cursor::MoveTo(
                        (board_start_col + (c as u16) * (h_scale + 1) + (h_scale / 2) + 1).into(),
                        (board_start_row + (r as u16) * (v_scale + 1) + 1) as u16,
                    ))
                    .unwrap()
                    .queue(style::Print(if cell.is_empty() { " " } else { cell }))
                    .unwrap();
            }
        }
        // END: Board Content
    }

    fn color_boxes(
        &self,
        board_start_col: u16,
        board_start_row: u16,
        h_scale: u16,
        v_scale: u16,
        row_start: u16,
        row_end: u16, /* exclusive */
        col_start: u16,
        col_end: u16, /* exclusive */
        colour: Color,
    ) {
        let mut stdout = stdout();
        let width = col_end - col_start;
        let blocks = "▒".repeat((((h_scale + 1) * width) - 1).into());
        let styled_blocks = match colour {
            Color::White => blocks.white(),
            Color::Red => blocks.red(),
            Color::Green => blocks.green(),
            Color::Yellow => blocks.yellow(),
            Color::Blue => blocks.blue(),
            _ => unimplemented!(), // not required
        };

        for r in row_start..row_end {
            stdout
                .queue(cursor::MoveTo(
                    board_start_col + col_start * (h_scale + 1) + 1,
                    board_start_row + r * (v_scale + 1) + 1,
                ))
                .unwrap()
                .queue(style::PrintStyledContent(styled_blocks.clone()))
                .unwrap();

            // For the v_scale*3 - 1 lines in between
            if r != (row_end - 1) {
                for i in 0..v_scale {
                    stdout
                        .queue(cursor::MoveTo(
                            (board_start_col + col_start * (h_scale + 1) + 1).into(),
                            (board_start_row + r * (v_scale + 1) + i + 2).into(),
                        ))
                        .unwrap()
                        .queue(style::PrintStyledContent(styled_blocks.clone()))
                        .unwrap();
                }
            }
        }
    }

    fn safe_spots(&mut self) {
        for coord in [
            (1, 8),
            (2, 6),
            (6, 1),
            (6, 12),
            (8, 2),
            (8, 13),
            (12, 8),
            (13, 6),
        ] {
            self.board[coord.0][coord.1] = String::from("∅");
        }
    }
}
