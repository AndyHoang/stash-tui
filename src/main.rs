mod api;
mod app;
mod ui;

use app::{App, ViewMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    env,
    error::Error,
    io,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get Stash URL from environment variable or use default
    let stash_url = env::var("STASH_URL")
        .unwrap_or_else(|_| {
            eprintln!("STASH_URL environment variable not set, using default URL");
            "http://localhost:9999".to_string()
        });
    
    println!("Connecting to Stash at: {}", stash_url);
    
    // Create app and run it
    let mut app = App::new(&stash_url);
    
    // Initial data fetch
    if let Err(e) = app.fetch_system_status().await {
        eprintln!("Failed to connect to Stash: {}", e);
    }
    
    if let Err(e) = app.fetch_scenes().await {
        eprintln!("Failed to fetch scenes: {}", e);
    }
    
    // Run the app
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Tab => {
                        if app.view_mode == app::ViewMode::List {
                            app.next_tab();
                            app.fetch_current_data().await?;
                        }
                    },
                    KeyCode::BackTab => {
                        if app.view_mode == app::ViewMode::List {
                            app.previous_tab();
                            app.fetch_current_data().await?;
                        }
                    },
                    KeyCode::Down => app.next_item(),
                    KeyCode::Up => app.previous_item(),
                    KeyCode::PageDown => {
                        if app.view_mode == app::ViewMode::List {
                            app.next_page().await?;
                        }
                    },
                    KeyCode::PageUp => {
                        if app.view_mode == app::ViewMode::List {
                            app.previous_page().await?;
                        }
                    },
                    KeyCode::Enter => app.toggle_detail_view(),
                    KeyCode::Esc => {
                        if app.view_mode == app::ViewMode::Detail {
                            app.toggle_detail_view();
                        }
                    },
                    KeyCode::Char('r') => {
                        if app.view_mode == app::ViewMode::List {
                            app.fetch_current_data().await?;
                        }
                    },
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(())
                    },
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
