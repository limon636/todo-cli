use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use crate::task::{load_tasks, save_tasks, Task};

// TUI State
pub struct App {
    pub tasks: Vec<Task>,
    pub list_state: ListState,
    pub mode: AppMode,
    pub input: String,
    pub message: Option<String>,
}

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Adding,
    Editing(u32),
    SetDueDate(u32),
}

impl App {
    pub fn new() -> App {
        let tasks = load_tasks();
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }
        App {
            tasks,
            list_state,
            mode: AppMode::Normal,
            input: String::new(),
            message: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn toggle_current(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.tasks.len() {
                self.tasks[i].done = !self.tasks[i].done;
                save_tasks(&self.tasks);
                self.message = Some(format!("Task {} completed!", self.tasks[i].id));
            }
        }
    }

    pub fn delete_current(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.tasks.len() {
                let id = self.tasks[i].id;
                self.tasks.remove(i);
                save_tasks(&self.tasks);
                self.message = Some(format!("Task {} deleted!", id));
                
                // Adjust selection
                if self.tasks.is_empty() {
                    self.list_state.select(None);
                } else if i >= self.tasks.len() {
                    self.list_state.select(Some(self.tasks.len() - 1));
                }
            }
        }
    }

    pub fn add_task(&mut self) {
        if !self.input.is_empty() {
            let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            self.tasks.push(Task {
                id,
                text: self.input.clone(),
                done: false,
                due_date: None,
            });
            save_tasks(&self.tasks);
            self.message = Some(format!("Task {} added!", id));
            self.input.clear();
            self.mode = AppMode::Normal;
            self.list_state.select(Some(self.tasks.len() - 1));
        }
    }

    pub fn edit_task(&mut self, id: u32) {
        if !self.input.is_empty() {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.text = self.input.clone();
                save_tasks(&self.tasks);
                self.message = Some(format!("Task {} updated!", id));
            }
            self.input.clear();
            self.mode = AppMode::Normal;
        }
    }

    pub fn set_due_date(&mut self, id: u32) {
        if !self.input.is_empty() {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.due_date = Some(self.input.clone());
                save_tasks(&self.tasks);
                self.message = Some(format!("Due date {} set!", self.input));
            }
            self.input.clear();
            self.mode = AppMode::Normal;
        }
    }
}

// Launch TUI
pub fn run_tui() -> io::Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Terminal restore
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match &app.mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Char(' ') | KeyCode::Enter => app.toggle_current(),
                        KeyCode::Char('d') => app.delete_current(),
                        KeyCode::Char('a') => {
                            app.mode = AppMode::Adding;
                            app.input.clear();
                            app.message = Some("Add new task (Enter: save, Esc: cancel)".to_string());
                        }
                        KeyCode::Char('e') => {
                            if let Some(i) = app.list_state.selected() {
                                if i < app.tasks.len() {
                                    let id = app.tasks[i].id;
                                    app.input = app.tasks[i].text.clone();
                                    app.mode = AppMode::Editing(id);
                                    app.message = Some("Edit task (Enter: save, Esc: cancel)".to_string());
                                }
                            }
                        }
                        KeyCode::Char('t') => {
                            if let Some(i) = app.list_state.selected() {
                                if i < app.tasks.len() {
                                    let id = app.tasks[i].id;
                                    app.input.clear();
                                    app.mode = AppMode::SetDueDate(id);
                                    app.message = Some("Set due date (YYYY-MM-DD) (Enter: save, Esc: cancel)".to_string());
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            app.input.clear();
                            app.message = None;
                        }
                        _ => {}
                    },
                    AppMode::Adding => match key.code {
                        KeyCode::Enter => app.add_task(),
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            app.input.clear();
                            app.message = None;
                        }
                        KeyCode::Char(c) => app.input.push(c),
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        _ => {}
                    },
                    AppMode::Editing(id) => {
                        let id = *id;
                        match key.code {
                            KeyCode::Enter => app.edit_task(id),
                            KeyCode::Esc => {
                                app.mode = AppMode::Normal;
                                app.input.clear();
                                app.message = None;
                            }
                            KeyCode::Char(c) => app.input.push(c),
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            _ => {}
                        }
                    }
                    AppMode::SetDueDate(id) => {
                        let id = *id;
                        match key.code {
                            KeyCode::Enter => app.set_due_date(id),
                            KeyCode::Esc => {
                                app.mode = AppMode::Normal;
                                app.input.clear();
                                app.message = None;
                            }
                            KeyCode::Char(c) => app.input.push(c),
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(5),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🦀 Todo TUI - Your Super Fast Todo Tool")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Task list
    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let status = if task.done { "✅" } else { "⬜" };
            let due_info = match &task.due_date {
                Some(date) => format!(" 📅 {}", date),
                None => String::new(),
            };
            let content = format!("{} [{}] {}{}", task.id, status, task.text, due_info);
            let style = if task.done {
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(content)).style(style)
        })
        .collect();

    let tasks_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("📋 Task List"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(tasks_list, chunks[1], &mut app.list_state);

    // Input box
    let input_text = if app.mode != AppMode::Normal {
        &app.input
    } else {
        ""
    };

    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);

    // Help/Messages
    let help_text = if let Some(msg) = &app.message {
        msg.clone()
    } else if app.mode == AppMode::Normal {
        "q: quit | j/k: up/down | Space/Enter: toggle | a: add | e: edit | t: due date | d: delete".to_string()
    } else {
        "Enter: save | Esc: cancel".to_string()
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("💡 Help"))
        .wrap(Wrap { trim: true });
    f.render_widget(help, chunks[3]);
}