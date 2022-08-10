use std::fmt::Display;
use termion::{clear, cursor, terminal_size};
use textwrap::wrap;

/// Erase the last output from the terminal.
#[derive(Debug)]
pub struct Erase<'a>(pub &'a str);

impl Display for Erase<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, _) = terminal_size().unwrap_or((u16::MAX, u16::MAX));
        let line = self
            .0
            .split('\n')
            .map(|l| wrap(l, textwrap::Options::new(x as usize).break_words(true)).len())
            .sum::<usize>() as u16
            - 1;
        write!(f, "\r{}{}", cursor::Up(line), clear::AfterCursor)
    }
}
