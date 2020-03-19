#[allow(dead_code)]
mod op_cli;
#[allow(dead_code)]
mod states;
#[allow(dead_code)]
mod util;

use std::io;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

use crate::util::event::{Event, Events};

fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    let mut controller = states::Screen::new();
    let mut ticks = 0;

    loop {
        match events.next()? {
            Event::Tick => ticks += 1,
            Event::Input(input) => controller = controller.step(input),
        };
        terminal.draw(|f| controller.render(f))?;

        // if ticks == 35 {
        //     break;
        // }
    }

    Ok(())
}
