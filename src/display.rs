use std::{io, time::{Duration, self}, thread::{self, JoinHandle}, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, borrow::{BorrowMut, Borrow}};
use tui::{Terminal, backend::CrosstermBackend, widgets::{Block, Borders}};
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
            board[coord.0][coord.1] = String::from("⚔");
        }
    }

    pub fn init_display(&self) -> JoinHandle<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();
        let start = time::Instant::now();

        let board = self.board.clone();
        let tab_string = self.tab_string.clone();
        let console_string = self.console_string.clone();
        let should_exit = self.should_exit.clone();

        thread::spawn(move ||{
            while !should_exit.load(Ordering::Relaxed) {
                if terminal.draw(|f| {
                    let size = f.size();
                    let block = Block::default()
                        .title(format!("{}-{} : {}", time::Instant::now().duration_since(start).as_secs(), board.lock().unwrap().borrow()[13][13]," Namaste from Ludo-The_Game 🙏 "))
                        .title_alignment(tui::layout::Alignment::Center)
                        .borders(Borders::ALL);
                    f.render_widget(block, size);
                }).is_err() {
                    panic!("Failed To Display the Board");
                }

                std::thread::sleep(Duration::from_millis(500));
            }
        })
    }

    pub fn end_display(&mut self) {
        self.should_exit.store(true,Ordering::Relaxed);
    }
}

