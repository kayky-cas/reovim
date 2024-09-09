use std::io::{self, stdout};

use crossterm::{
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
};

fn main() -> io::Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::DarkYellow))?
        .execute(Print("Hello, reovim"))?;

    Ok(())
}
