use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

mod docker;

struct Internal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = Internal {
        terminal: setup_terminal().context("setup failed")?,
    };
    if terminal.run().await.is_err() {
        dbg!("terminal.run err");
    };
    terminal
        .restore_terminal()
        .context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

impl Internal {
    fn restore_terminal(&mut self) -> Result<()> {
        disable_raw_mode().context("failed to disable raw mode")?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .context("unable to switch to main screen")?;
        self.terminal.show_cursor().context("unable to show cursor")
    }

    async fn run(&mut self) -> Result<()> {
        let mut buttons = vec![];

        loop {
            let quit = should_quit()?;
            if !quit.2.is_empty() {
                buttons.push(quit.2);
            }
            if quit.0 {
                break;
            }
            if quit.1 {
                self.terminal.clear().ok();
                self.restore_terminal().context("restore terminal error")?;
                docker::do_stuff().await.ok();
                self.terminal.clear().ok();
                self.terminal = setup_terminal().context("setup failed")?;
                continue;
            }
            self.terminal.draw(|i| crate::render_app(i, &buttons))?;
        }
        Ok(())
    }
}

fn render_app(frame: &mut Frame, buttons: &[String]) {
    let greeting = Paragraph::new(format!(
        "Hello World! (press 'q' to quit, 'e' to docker execute) - just pressed: {buttons:?}"
    ));

    frame.render_widget(greeting, frame.size());
}

fn should_quit() -> Result<(bool, bool, String)> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            if key.code == KeyCode::Char('e') {
                return Ok((false, true, String::from("e")));
            }

            if key.code == KeyCode::Char('q') {
                return Ok((true, false, String::from("q")));
            }
            return Ok((false, false, format!("{:?}", key.code)));
        }
    }
    Ok((false, false, String::new()))
}
