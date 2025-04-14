use crate::app::{App, AppTab, ViewMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Status bar
        ].as_ref())
        .split(f.size());

    draw_tabs(f, app, chunks[0]);
    
    // Draw content based on selected tab and view mode
    match app.view_mode {
        ViewMode::List => {
            match app.current_tab {
                AppTab::Scenes => draw_scenes(f, app, chunks[1]),
                AppTab::Performers => draw_performers(f, app, chunks[1]),
                AppTab::Tags => draw_tags(f, app, chunks[1]),
                AppTab::Studios => draw_studios(f, app, chunks[1]),
            }
        },
        ViewMode::Detail => {
            match app.current_tab {
                AppTab::Scenes => draw_scene_detail(f, app, chunks[1]),
                AppTab::Performers => draw_performer_detail(f, app, chunks[1]),
                AppTab::Tags => draw_tag_detail(f, app, chunks[1]),
                AppTab::Studios => draw_studios(f, app, chunks[1]),
            }
        }
    }
    
    draw_status_bar(f, app, chunks[2]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["Scenes", "Performers", "Tags", "Studios"];
    let selected_index = match app.current_tab {
        AppTab::Scenes => 0,
        AppTab::Performers => 1,
        AppTab::Tags => 2,
        AppTab::Studios => 3,
    };
    
    let tabs = Tabs::new(
        titles.iter().map(|t| {
            Line::from(vec![Span::styled(*t, Style::default().fg(Color::White))])
        }).collect(),
    )
    .block(Block::default().title("Stash TUI").borders(Borders::ALL))
    .select(selected_index)
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );
    
    f.render_widget(tabs, area);
}

fn draw_scenes(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .scenes
        .iter()
        .map(|scene| {
            let title = scene.title.clone().unwrap_or_else(|| "Untitled".to_string());
            let duration = scene.files.as_ref()
                .and_then(|files| files.first())
                .and_then(|file| file.duration)
                .map(|d| format!(" ({:.1}s)", d))
                .unwrap_or_else(|| "".to_string());
            
            ListItem::new(Line::from(vec![
                Span::raw(title),
                Span::styled(duration, Style::default().fg(Color::Gray)),
            ]))
        })
        .collect();

    let scenes = List::new(items)
        .block(Block::default().title(format!("Scenes (Page {} of {})", 
            app.page, 
            (app.total_count + app.per_page - 1) / app.per_page
        )).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(scenes, area, &mut state);
}

fn draw_performers(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .performers
        .iter()
        .map(|performer| {
            let name = &performer.name;
            let favorite = if performer.favorite { " ★" } else { "" };
            let scene_count = format!(" ({} scenes)", performer.scene_count);
            
            ListItem::new(Line::from(vec![
                Span::raw(format!("{}{}", name, favorite)),
                Span::styled(scene_count, Style::default().fg(Color::Gray)),
            ]))
        })
        .collect();

    let performers = List::new(items)
        .block(Block::default().title(format!("Performers (Page {} of {})", 
            app.page, 
            (app.total_count + app.per_page - 1) / app.per_page
        )).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(performers, area, &mut state);
}

fn draw_tags(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .tags
        .iter()
        .map(|tag| {
            let name = &tag.name;
            let scene_count = format!(" ({} scenes)", tag.scene_count);
            
            ListItem::new(Line::from(vec![
                Span::raw(name),
                Span::styled(scene_count, Style::default().fg(Color::Gray)),
            ]))
        })
        .collect();

    let tags = List::new(items)
        .block(Block::default().title(format!("Tags (Page {} of {})", 
            app.page, 
            (app.total_count + app.per_page - 1) / app.per_page
        )).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected_index));
    f.render_stateful_widget(tags, area, &mut state);
}

fn draw_studios(f: &mut Frame, _app: &App, area: Rect) {
    let content = Paragraph::new("Studios tab (not implemented yet)")
        .block(Block::default().title("Studios").borders(Borders::ALL));
    f.render_widget(content, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status = if app.loading {
        format!("Loading... {}", app.status_message)
    } else {
        app.status_message.clone()
    };
    
    let help_text = match app.view_mode {
        ViewMode::List => "q: Quit | Tab: Next tab | ↑/↓: Navigate | PgUp/PgDn: Change page | Enter: View details",
        ViewMode::Detail => "q: Quit | Esc: Back to list | ↑/↓: Navigate items",
    };
    
    let status_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(area);
    
    let status_widget = Paragraph::new(status)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    
    let help_widget = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));
    
    f.render_widget(status_widget, status_bar[0]);
    f.render_widget(help_widget, status_bar[1]);
}
fn draw_scene_detail(f: &mut Frame, app: &App, area: Rect) {
    if app.scenes.is_empty() || app.selected_index >= app.scenes.len() {
        return;
    }

    let scene = &app.scenes[app.selected_index];
    
    // Create a layout for the detail view
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(5),     // Details
            Constraint::Length(3),  // File info
        ].as_ref())
        .split(area);
    
    // Title
    let title = scene.title.clone().unwrap_or_else(|| "Untitled".to_string());
    let title_widget = Paragraph::new(title)
        .block(Block::default().title("Scene Title").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(title_widget, chunks[0]);
    
    // Details
    let details = scene.details.clone().unwrap_or_else(|| "No details available".to_string());
    let details_widget = Paragraph::new(details)
        .block(Block::default().title("Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(details_widget, chunks[1]);
    
    // File info
    let file_info = if let Some(files) = &scene.files {
        if let Some(file) = files.first() {
            let mut info = Vec::new();
            
            if let Some(duration) = file.duration {
                info.push(format!("Duration: {:.1}s", duration));
            }
            
            if let Some(width) = file.width {
                if let Some(height) = file.height {
                    info.push(format!("Resolution: {}x{}", width, height));
                }
            }
            
            if let Some(video_codec) = &file.video_codec {
                info.push(format!("Video Codec: {}", video_codec));
            }
            
            if let Some(audio_codec) = &file.audio_codec {
                info.push(format!("Audio Codec: {}", audio_codec));
            }
            
            if let Some(bit_rate) = file.bit_rate {
                info.push(format!("Bitrate: {} kbps", bit_rate / 1000));
            }
            
            info.join(" | ")
        } else {
            "No file information available".to_string()
        }
    } else {
        "No file information available".to_string()
    };
    
    let file_widget = Paragraph::new(file_info)
        .block(Block::default().title("File Information").borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(file_widget, chunks[2]);
}

fn draw_performer_detail(f: &mut Frame, app: &App, area: Rect) {
    if app.performers.is_empty() || app.selected_index >= app.performers.len() {
        return;
    }

    let performer = &app.performers[app.selected_index];
    
    // Create a layout for the detail view
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Name
            Constraint::Min(3),     // Details
        ].as_ref())
        .split(area);
    
    // Name
    let favorite_star = if performer.favorite { " ★" } else { "" };
    let name_widget = Paragraph::new(format!("{}{}", performer.name, favorite_star))
        .block(Block::default().title("Performer").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(name_widget, chunks[0]);
    
    // Details
    let mut details = Vec::new();
    
    if let Some(gender) = &performer.gender {
        details.push(format!("Gender: {}", gender));
    }
    
    details.push(format!("Scene Count: {}", performer.scene_count));
    
    let details_widget = Paragraph::new(details.join("\n"))
        .block(Block::default().title("Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(details_widget, chunks[1]);
}

fn draw_tag_detail(f: &mut Frame, app: &App, area: Rect) {
    if app.tags.is_empty() || app.selected_index >= app.tags.len() {
        return;
    }

    let tag = &app.tags[app.selected_index];
    
    // Create a layout for the detail view
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Name
            Constraint::Min(3),     // Details
        ].as_ref())
        .split(area);
    
    // Name
    let name_widget = Paragraph::new(tag.name.clone())
        .block(Block::default().title("Tag").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(name_widget, chunks[0]);
    
    // Details
    let details = vec![
        format!("Scene Count: {}", tag.scene_count),
        format!("Performer Count: {}", tag.performer_count),
        format!("Image Count: {}", tag.image_count),
    ];
    
    let details_widget = Paragraph::new(details.join("\n"))
        .block(Block::default().title("Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(details_widget, chunks[1]);
}
