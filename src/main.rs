mod process;
mod ui;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use process::SortKey;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    let mut sort_key = SortKey::Cpu;

    crossterm::terminal::enable_raw_mode().unwrap();

    ui::clear_screen();

    loop {
        ui::display_header(sort_key);
        ui::move_cursor_to_content();

        sys.refresh_all();

        let fetch_size = 30;
        let processes = process::fetch_processes(&sys, fetch_size, sort_key);
        ui::display_rows(&processes);

        let duration = Duration::from_secs(1);
        if event::poll(duration).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => sort_key = SortKey::Cpu,
                    KeyCode::Char('m') => sort_key = SortKey::Memory,
                    _ => {}
                }
            }
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    ui::clear_screen();
}
