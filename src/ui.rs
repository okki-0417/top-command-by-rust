use crate::process::{ProcessInfo, SortKey};

const RED: &str = "\x1B[31m";
const YELLOW: &str = "\x1B[33m";
const GREEN: &str = "\x1B[32m";
const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";

const BAR_WIDTH: usize = 20;
const NAME_WIDTH: usize = 30;

fn make_bar(percent: f32, max: f32) -> String {
    let ratio = (percent / max).min(1.0);
    let filled = (ratio * BAR_WIDTH as f32) as usize;
    let empty = BAR_WIDTH - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn color_for_percent(percent: f32) -> &'static str {
    if percent > 50.0 {
        RED
    } else if percent > 20.0 {
        YELLOW
    } else {
        GREEN
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn display_header(sort_key: SortKey) {
    print!("\x1B[1;1H"); // カーソルを先頭に
    let sort_label = match sort_key {
        SortKey::Cpu => "CPU",
        SortKey::Memory => "MEM",
    };
    print!(
        "top-command-by-rust | Sort: {}{}{} | q:終了 c:CPU順 m:MEM順\r\n\r\n",
        BOLD, sort_label, RESET
    );
    print!(
        "{:>7} {:<width$} {:>22} {:>22}\r\n",
        "PID", "NAME", "CPU", "MEM", width = NAME_WIDTH
    );
    print!("{}\r\n", "-".repeat(80));
}

pub fn move_cursor_to_content() {
    print!("\x1B[5;1H");
    print!("\x1B[J");
}

pub fn display_rows(processes: &[ProcessInfo]) {
    use std::io::{self, Write};

    for p in processes {
        let cpu_bar = make_bar(p.cpu_percent, 100.0);
        let mem_percent = (p.memory_mb as f32 / 1000.0) * 100.0;
        let mem_bar = make_bar(mem_percent, 100.0);

        let cpu_color = color_for_percent(p.cpu_percent);
        let mem_color = color_for_percent(mem_percent);

        let name: String = p.name.chars().take(NAME_WIDTH).collect();

        print!(
            "{:>7} {:<width$} {}{} {:>5.1}%{} {}{} {:>5}M{}\r\n",
            p.pid,
            name,
            cpu_color, cpu_bar, p.cpu_percent, RESET,
            mem_color, mem_bar, p.memory_mb, RESET,
            width = NAME_WIDTH
        );
    }
    io::stdout().flush().unwrap();
}
