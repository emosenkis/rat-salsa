//!
//! Reset the terminal after a crash.
//! Just some small utility ...
//!

use crossterm::ExecutableCommand;
use crossterm::cursor::{DisableBlinking, SetCursorStyle};
use crossterm::event::DisableBracketedPaste;
use crossterm::terminal::{
  LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::stdout;

fn main() -> Result<(), anyhow::Error> {
  enable_raw_mode().expect("");
  disable_raw_mode().expect("");
  stdout().execute(DisableBracketedPaste).expect("");
  stdout()
    .execute(SetCursorStyle::DefaultUserShape)
    .expect("");
  stdout().execute(DisableBlinking).expect("");
  // stdout().execute(DisableMouseCapture).expect("");
  stdout().execute(LeaveAlternateScreen).expect("");

  let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).expect("");
  terminal.clear().expect("");

  Ok(())
}
