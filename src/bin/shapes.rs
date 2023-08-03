use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct App {
    show_popup: bool,
}

impl App {
    fn new() -> App {
        App { show_popup: false }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('p') => app.show_popup = !app.show_popup,
                _ => {}
            }
        }
    }
}
// use tui::{
//     backend::Backend,
//     layout::{Constraint, Direction, Layout},
//     widgets::{Block, Borders},
//     Frame,
// };
// fn ui<B: Backend>(f: &mut Frame<B>) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(1)
//         .constraints(
//             [
//                 Constraint::Percentage(10),
//                 Constraint::Percentage(40),
//                 Constraint::Percentage(40),
//                 Constraint::Percentage(10),
//             ]
//             .as_ref(),
//         )
//         .split(f.size());
//     let block = Block::default().title("Block").borders(Borders::ALL);
//     f.render_widget(block, chunks[0]);
//     let block = Block::default().title("Block 2").borders(Borders::ALL);
//     f.render_widget(block, chunks[1]);
//     let block = Block::default().title("Block 3").borders(Borders::ALL);
//     f.render_widget(block, chunks[3]);
// }
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let block = Block::default()
        .title("Enter a todo item")
        .borders(Borders::ALL);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(size);

    let text = "The Todoodler";
    let paragraph = Paragraph::new(Span::styled(
        text,
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    f.render_widget(block, chunks[1]);
    f.render_widget(paragraph, chunks[0]);
}
