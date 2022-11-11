use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

pub struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    blocks: Vec<Block<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        // setup terminal
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        App {
            terminal,
            blocks: App::init_blocks(),
        }
    }

    fn init_blocks() -> Vec<Block<'a>> {
        let blocks = vec![Block::default().title("Block").borders(Borders::ALL)];
        blocks
    }

    fn draw_terminal(&mut self) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let size = f.size();
            for block in &self.blocks {
                f.render_widget(block.clone(), size);
            }
        })?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let mut last_tick = Instant::now();
        loop {
            self.draw_terminal()?;
            let tick_rate = Duration::from_millis(200);
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout).is_ok() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if let KeyCode::Esc = key_event.code {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
