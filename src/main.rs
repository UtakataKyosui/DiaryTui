mod app;
mod calendar;
mod editor;
mod storage;
mod ui;

use app::{App, Mode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Calendar => match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        app.calendar.move_selection(-1);
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        app.calendar.move_selection(1);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.calendar.move_selection(-7);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.calendar.move_selection(7);
                    }
                    KeyCode::Char('H') => {
                        app.calendar.prev_month();
                    }
                    KeyCode::Char('L') => {
                        app.calendar.next_month();
                    }
                    KeyCode::Enter => {
                        app.switch_to_editor();
                    }
                    _ => {}
                },
                Mode::Editor => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        match key.code {
                            KeyCode::Char('s') => {
                                app.save_and_return_to_calendar();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char(c) => {
                                app.editor.insert_char(c);
                            }
                            KeyCode::Backspace => {
                                app.editor.delete_char();
                            }
                            KeyCode::Enter => {
                                app.editor.insert_newline();
                            }
                            KeyCode::Left => {
                                app.editor.move_cursor_left();
                            }
                            KeyCode::Right => {
                                app.editor.move_cursor_right();
                            }
                            KeyCode::Up => {
                                app.editor.move_cursor_up();
                            }
                            KeyCode::Down => {
                                app.editor.move_cursor_down();
                            }
                            KeyCode::Tab => {
                                app.save_and_return_to_calendar();
                            }
                            KeyCode::Esc => {
                                app.cancel_edit();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
