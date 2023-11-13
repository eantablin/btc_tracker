use reqwest;
use serde_json::Value;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;
use std::io::stdout;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Introduce a delay to allow Tokio runtime to start
    sleep(Duration::from_secs(1)).await;

    // Fetch Bitcoin price data
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    let price: Value = serde_json::from_str(&body)?;

    let bitcoin_price = price["bitcoin"]["usd"].as_f64().unwrap_or(0.0);

    // Set up TUI
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Display Bitcoin price in TUI
    let text = Spans::from(vec![
        Span::styled("Bitcoin Price (USD): ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{:.2}", bitcoin_price),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
    ]);

    let paragraph = Paragraph::new(text);

    terminal.clear()?;
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(size);

        let block = Block::default().title("Bitcoin Tracker").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        f.render_widget(paragraph, chunks[1]);
    })?;

    // Wait for a key press before exiting
    tokio::task::block_in_place(|| {
        crossterm::event::read().ok();
    });

    Ok(())
}
