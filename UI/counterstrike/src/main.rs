use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

// Import your action modules
mod single_file_run;
mod multi_file_run;
mod benchmark;

#[derive(PartialEq, Debug)]
enum AppState {
    Menu,
    Execution(MenuItem),
    CommandOutput { item: MenuItem, output: String },
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MenuItem {
    SingleFileRun,
    MultiFileRun,
    Benchmark,
}

impl MenuItem {
    fn next(&self) -> Self {
        match self {
            MenuItem::SingleFileRun => MenuItem::MultiFileRun,
            MenuItem::MultiFileRun => MenuItem::Benchmark,
            MenuItem::Benchmark => MenuItem::SingleFileRun,
        }
    }

    fn prev(&self) -> Self {
        match self {
            MenuItem::SingleFileRun => MenuItem::Benchmark,
            MenuItem::Benchmark => MenuItem::MultiFileRun,
            MenuItem::MultiFileRun => MenuItem::SingleFileRun,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            MenuItem::SingleFileRun => "Make single-file run",
            MenuItem::MultiFileRun => "Make multi-file run",
            MenuItem::Benchmark => "See benchmark performance",
        }
    }
}

// Draw the UI based on the current state
fn draw_ui<B: Backend>(terminal: &mut Terminal<B>, state: &AppState, selected_item: MenuItem) -> io::Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ].as_ref())
        .split(terminal.size()?);

    match state {
        AppState::Menu | AppState::Execution(_) => {
            let title = Paragraph::new(Line::from(vec![Span::styled(
                "CounterStrike",
                Style::default().fg(Color::Yellow),
            )]))
            .alignment(Alignment::Center);

            let single_file_run = Paragraph::new(Line::from(vec![Span::styled(
                "Make single-file run",
                if selected_item == MenuItem::SingleFileRun {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                },
            )]))
            .block(Block::default().borders(Borders::ALL).title("1"));

            let multi_file_run = Paragraph::new(Line::from(vec![Span::styled(
                "Make multi-file run",
                if selected_item == MenuItem::MultiFileRun {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                },
            )]))
            .block(Block::default().borders(Borders::ALL).title("2"));

            let benchmark = Paragraph::new(Line::from(vec![Span::styled(
                "See benchmark performance",
                if selected_item == MenuItem::Benchmark {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                },
            )]))
            .block(Block::default().borders(Borders::ALL).title("3"));

            terminal.draw(|f| {
                f.render_widget(title, chunks[0]);
                f.render_widget(single_file_run, chunks[1]);
                f.render_widget(multi_file_run, chunks[2]);
                f.render_widget(benchmark, chunks[3]);
            })?;
        }

        AppState::CommandOutput { item, output } => {
            terminal.draw(|f| {
                let command = Paragraph::new(Line::from(Span::styled(
                    format!("> Running {}", item.label()),
                    Style::default(),
                )))
                .block(Block::default().borders(Borders::ALL).title("Command"));
                f.render_widget(command, chunks[0]);

                let output_paragraph = Paragraph::new(Line::from(output.as_str()))
                    .block(Block::default().borders(Borders::ALL).title("Output"));
                f.render_widget(output_paragraph, chunks[1]);
            })?;
        }
    }

    Ok(())
}

// Execute a function based on the selected menu item
fn execute_script(item: &MenuItem) -> String {
    match item {
        MenuItem::SingleFileRun => single_file_run::run(),
        MenuItem::MultiFileRun => multi_file_run::run(),
        MenuItem::Benchmark => benchmark::run(),
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial state is explicitly the menu with no automatic execution
    let mut state = AppState::Menu;
    let mut selected_item = MenuItem::SingleFileRun;
    let mut should_exit = false;

    while !should_exit {
        draw_ui(&mut terminal, &state, selected_item)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    if let AppState::Menu = state {
                        state = AppState::CommandOutput {
                            item: selected_item,
                            output: execute_script(&selected_item),
                        };
                    }
                }
                KeyCode::Up => selected_item = selected_item.prev(),
                KeyCode::Down => selected_item = selected_item.next(),
                KeyCode::Esc | KeyCode::Backspace => state = AppState::Menu,
                KeyCode::Char('q') => should_exit = true,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
