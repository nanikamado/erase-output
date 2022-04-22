use std::fmt::Display;
use termion::{clear, cursor, terminal_size};
use unicode_width::UnicodeWidthStr;

/// Erase the last output from the terminal.
#[derive(Debug)]
pub struct Erase<'a>(pub &'a str);

impl Display for Erase<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, _) = terminal_size().unwrap_or((u16::MAX, u16::MAX));
        let line = self
            .0
            .split('\n')
            .map(|l| {
                let l = l.width_cjk();
                if l == 0 {
                    1
                } else {
                    l / (x as usize) + if l % x as usize == 0 as usize { 0 } else { 1 }
                }
            })
            .sum::<usize>() as u16
            - 1;
        if line == 0 {
            write!(f, "\r{}", clear::AfterCursor)
        } else {
            write!(f, "\r{}{}", cursor::Up(line), clear::AfterCursor)
        }
    }
}
