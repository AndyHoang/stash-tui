mod api;
mod app;
mod ui;
mod input;

use app::App;
use input::{Action, KeyMap};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
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
    
    // Load key configuration
    let key_map = if let Ok(config) = input::KeyConfig::load_from_file("config.toml") {
        config.to_key_map()
    } else {
        // Use default key bindings if config file not found
        KeyMap::new()
    };
    
    // Initial data fetch
    if let Err(e) = app.fetch_system_status().await {
        eprintln!("Failed to connect to Stash: {}", e);
    }
    
    if let Err(e) = app.fetch_scenes().await {
        eprintln!("Failed to fetch scenes: {}", e);
    }
    
    // Run the app
    let res = run_app(&mut terminal, &mut app, key_map).await;

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
    key_map: KeyMap,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let event = event::read()?;
            let action = key_map.handle_key_event(event);
            
            // Handle the action based on current app state
            match action {
                Action::Quit => return Ok(()),
                Action::NextTab => {
                    if app.view_mode == app::ViewMode::List {
                        app.next_tab();
                        app.fetch_current_data().await?;
                    }
                },
                Action::PreviousTab => {
                    if app.view_mode == app::ViewMode::List {
                        app.previous_tab();
                        app.fetch_current_data().await?;
                    }
                },
                Action::NextItem => app.next_item(),
                Action::PreviousItem => app.previous_item(),
                Action::NextPage => {
                    if app.view_mode == app::ViewMode::List {
                        app.next_page().await?;
                    }
                },
                Action::PreviousPage => {
                    if app.view_mode == app::ViewMode::List {
                        app.previous_page().await?;
                    }
                },
                Action::ToggleDetailView => app.toggle_detail_view(),
                Action::Back => {
                    if app.view_mode == app::ViewMode::Detail {
                        app.toggle_detail_view();
                    }
                },
                Action::Refresh => {
                    if app.view_mode == app::ViewMode::List {
                        app.fetch_current_data().await?;
                    }
                },
                Action::Noop => {},
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
