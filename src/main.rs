use std::io::{self, stdout, Write};

use crossterm::{
    style::{Color, Print, SetForegroundColor},
    QueueableCommand,
};

fn main() -> io::Result<()> {
    stdout()
        .queue(SetForegroundColor(Color::DarkYellow))?
        .queue(Print("Hello, reovim"))?
        .flush()?;

    Ok(())
}
