mod github;
mod app;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{List, ListItem, ListState};
use app::{App, Screen};
use github::{get_user, get_repos, get_repos_content};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let token = std::env::var("GITHUB_TOKEN")?;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut app = App::new();

    app.user = Some(get_user(&token).await?);
    app.repos = get_repos(&token).await?;

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|frame| {
            // Turn each repo into a list item
            let items: Vec<ListItem> = app.repos.iter()
                .map(|repo| ListItem::new(repo.name.clone()))
                .collect();

            // Create the list widget
            let list = List::new(items)
                .highlight_symbol(">> ");

            // Draw it
            frame.render_stateful_widget(list, frame.area(), &mut list_state);
        })?;

        // Handle key presses
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    let i = list_state.selected().unwrap_or(0);
                    if i < app.repos.len() - 1 {
                        list_state.select(Some(i + 1));
                    }
                }
                KeyCode::Up => {
                    let i = list_state.selected().unwrap_or(0);
                    if i > 0 {
                        list_state.select(Some(i - 1));
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
