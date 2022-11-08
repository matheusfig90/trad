use crossbeam_channel::{select, unbounded, Receiver};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    thread,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

fn setup_ui_events() -> Receiver<Event> {
    let (sender, receiver) = unbounded();
    thread::spawn(move || loop {
        sender.send(crossterm::event::read().unwrap()).unwrap();
    });

    receiver
}

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    ui_events_receiver: Receiver<Event>,
}

impl App {
    pub fn new() -> Self {
        // setup terminal
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        let ui_events_receiver = setup_ui_events();

        App {
            terminal,
            ui_events_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        loop {
            select! { recv(self.ui_events_receiver) -> message => {

                    if let Event::Key(key_event) = message.unwrap() {
                        if key_event.modifiers.is_empty() {
                                if let KeyCode::Char('q') = key_event.code {
                                    break
                                }
                        }
                    }
            }
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
