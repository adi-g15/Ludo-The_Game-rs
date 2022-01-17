use std::{io::{self, stdout, Write}, time::{Duration, self}, thread::{self, JoinHandle}, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, borrow::{BorrowMut, Borrow}};
use crossterm::{self, QueueableCommand, terminal, cursor, style::{self, Stylize, style, Color}, ExecutableCommand};
use arr_macro::arr;

pub struct Display {
    board: Arc<Mutex<[[String; 15]; 15]>>,
    tab_string: Arc<Mutex<String>>,
    console_string: Arc<Mutex<String>>,
    should_exit: Arc<AtomicBool>,  // atomic bool doesn't have .clone()
}

impl Display {
    pub fn new() -> Self {
        const row: [String; 15] = arr![String::new(); 15];

        Display {
            board: Arc::new(Mutex::new(arr![row; 15])),
            tab_string: Arc::new(Mutex::new(String::new())),
            console_string: Arc::new(Mutex::new(String::new())),
            should_exit: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn custom_diplay(&mut self) {
        // Additional 'let' required, since .lock().unwrap() creates a temporary
        let mut board_ref = self.board.lock().unwrap();
        let board = board_ref.borrow_mut();
        board[1][1] = String::from("🟢");
        board[1][4] = String::from("🟢");
        board[4][1] = String::from("🟢");
        board[4][4] = String::from("🟢");

        board[10][1] = String::from("🔴");
        board[10][4] = String::from("🔴");
        board[13][1] = String::from("🔴");
        board[13][4] = String::from("🔴");

        board[1][10] = String::from("🟡");
        board[1][13] = String::from("🟡");
        board[4][10] = String::from("🟡");
        board[4][13] = String::from("🟡");

        board[10][10] = String::from("🔵");
        board[10][13] = String::from("🔵");
        board[13][10] = String::from("🔵");
        board[13][13] = String::from("🔵");

        for coord in [(1,8),(2,6),(6,1),(6,12),(8,2),(8,13),(12,8),(13,6)] {
            board[coord.0][coord.1] = String::from("∅");
        }
    }

    pub fn init_display(&self) -> JoinHandle<()> {
        let start = time::Instant::now();

        let board = self.board.clone();
        let tab_string = self.tab_string.clone();
        let console_string = self.console_string.clone();
        let should_exit = self.should_exit.clone();

        // terminal::enable_raw_mode();
        stdout().execute(terminal::SetTitle("Ludo-The_Game"));

        thread::spawn(move ||{
            while !should_exit.load(Ordering::Relaxed) {
                let mut stdout = stdout();

                stdout
                    .queue(terminal::Clear(terminal::ClearType::All)).unwrap()
                    .queue(cursor::Hide).unwrap();

                let msg = format!(
                    "{} : {}",
                    time::Instant::now().duration_since(start).as_secs(),
                    " Namaste from Ludo-The_Game 🙏 "
                );
                let size = match terminal::size() {
                    Ok(size) => size,
                    Err(e) => panic!("{:?}", e)
                };
                let columns = size.0 as usize;
                let rows = size.1 as usize;

                let hor_char = "─";
                let vert_char = "│";

                // START: Header
                let mut left_spacing = (columns-msg.len())/2;
                stdout
                    .queue(cursor::MoveTo(1,0)).unwrap()
                    .queue(style::PrintStyledContent(hor_char.repeat(columns-1).white())).unwrap()
                    .queue(cursor::MoveToNextLine(1)).unwrap()
                    .queue(style::PrintStyledContent(format!(" {}",vert_char).white())).unwrap()
                    .queue(style::PrintStyledContent(
                            format!("{}{}", " ".repeat(left_spacing), msg)
                    .with(Color::Rgb{r:214,g:214,b:214}).bold())).unwrap()
                    .queue(style::Print(format!("{}{}", " ".repeat(columns-1-msg.len()-left_spacing), vert_char))).unwrap()
                    .queue(cursor::MoveToNextLine(1)).unwrap()
                    .queue(style::PrintStyledContent(format!(" {}", hor_char.repeat(columns-1)).white())).unwrap()
                    ;
               // END: Header

                // Some gap between Header and Board
                stdout
                    .queue(cursor::MoveToNextLine(2)).unwrap()
                    ;

                // START: Board Design
                let board_ref = board.lock().unwrap();
                let h_scale = 3;
                let v_scale = 1;

                left_spacing = (columns - (15*(h_scale+1)+1))/2;
 
                let board_start_row = cursor::position().unwrap().1;
                let board_start_col = left_spacing as u16;

                stdout
                    .queue(style::Print(format!("{}{}", " ".repeat(left_spacing), hor_char.repeat(15*(h_scale+1)+1)))).unwrap()
                    .queue(cursor::MoveToNextLine(1)).unwrap()
                    ;

                for _i in 0..15 {
                    // Left spacing of each row
                    stdout
                        .queue(style::Print(format!("{}{}", " ".repeat(left_spacing), vert_char))).unwrap()
                        ;

                    for _j in 0..15 {
                        stdout
                            .queue(style::Print(format!("{}{}", " ".repeat(h_scale),vert_char))).unwrap();
                    }
 
                    // Bottom Line of each row
                   stdout
                        .queue(cursor::MoveToNextLine(1)).unwrap()
                        .queue(style::Print(format!("{}{}", " ".repeat(left_spacing), hor_char.repeat(15*(h_scale+1)+1)))).unwrap()
                        .queue(cursor::MoveToNextLine(1)).unwrap()
                        ;
 
                }
                // END: Board Design
                
                // START: Board Content
                let mut r=0;
                for row in board_ref.iter() {
                    let mut c=0;
                    for cell in row.iter() {
                        stdout
                            .queue(cursor::MoveTo(
                                    (board_start_col as usize + c*(h_scale+1) + (h_scale/2)+1) as u16,
                                    (board_start_row + r*(v_scale+1) + 1) as u16
                                )).unwrap()
                            .queue(style::Print(if cell.is_empty() { " " } else { cell })).unwrap()
                            ;
                        c+=1;
                    }

                    r+=1;
                }
                // END: Board Content
 
                if stdout.flush().is_err() {
                    terminal::disable_raw_mode().unwrap();
                    panic!("Couldn't print board");
                }

                std::thread::sleep(Duration::from_millis(100));
            }

            terminal::disable_raw_mode();
        })
    }

    pub fn end_display(&mut self) {
        self.should_exit.store(true,Ordering::Relaxed);
    }
}

