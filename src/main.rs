use std::io;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with hardcoded token balances
    let app = App {
        token_balances: vec![
            ("Token1".to_string(), 1000.0),
            ("Token2".to_string(), 2500.0),
            ("Token3".to_string(), 500.0),
        ],
        total_usd_value: 4000.0,
    };

    // Run the app
    terminal.draw(|f| ui(f, &app))?;

    // Wait for user to press 'q'
    crossterm::event::read()?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

struct App {
    token_balances: Vec<(String, f64)>,
    total_usd_value: f64,
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    let token_balance_text = app
        .token_balances
        .iter()
        .map(|(token, balance)| format!("{}: {:.2}", token, balance))
        .collect::<Vec<String>>()
        .join("\n");

    let paragraph = Paragraph::new(token_balance_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Token Balances"));
    f.render_widget(paragraph, chunks[0]);

    let total_value_text = Text::raw(format!("Total USD Value: ${:.2}", app.total_usd_value));
    let total_value_paragraph = Paragraph::new(total_value_text)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Total Value"));
    f.render_widget(total_value_paragraph, chunks[1]);
}