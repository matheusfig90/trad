use crossbeam_channel::{select, unbounded, Receiver};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::widgets::Paragraph;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
use std::{
    io::{self, Stdout},
    thread,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
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
    input: Input,
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
            input: Input::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            self.terminal.draw(|f| {
                let size = f.size();
                let input = Paragraph::new(self.input.value())
                    .block(Block::default().borders(Borders::ALL).title("Input"));
                f.render_widget(input, size);
                f.set_cursor(self.input.visual_cursor() as u16 + 1, 1);
            })?;

            select! { recv(self.ui_events_receiver) -> message => {

                    if let Event::Key(key_event) = message.unwrap() {
                        if key_event.modifiers.is_empty() {
                            match key_event.code {
                                KeyCode::Esc => break,
                                _ => { self.input.handle_event(&Event::Key(key_event));},
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
