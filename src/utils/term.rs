use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::sync::Mutex;

pub struct TerminalSize {
    screen_x: u16,
    screen_y: u16,
    screen_cols: u16,
    screen_rows: u16,
    cell_width: u16,
    cell_height: u16,
}
