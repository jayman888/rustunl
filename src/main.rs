mod app;
mod data;
mod ui;

use std::{error::Error, io, time::Duration};

use clap::Clap;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use app::event::{Config, Events};
use app::{action_event, App};

/// Rustunl - A Rust Pritunl TUI
#[derive(Clap)]
struct Opts {
    /// Time in ms between two ticks.
    #[clap(short, long, default_value = "250")]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(opts.tick_rate),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Rustunl", true);
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        action_event(&mut app, events.next()?);

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
