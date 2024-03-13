use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use crate::command::login::ConfigOptions;

pub fn run(option: ConfigOptions) {
    println!("login: {:?}", option);

    if option.interactive {
        interactive().unwrap();
        return;
    } else if option.id.is_empty() || option.password.is_empty() {
        println!("id or password is empty");
        return;
    }

    let mut credentials = String::new();
    credentials.push_str(format!("id: {}\n", option.id).as_str());
    credentials.push_str(format!("pwd: {}\n", option.password).as_str());

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./credentials.txt")
        .unwrap();

    file.write(credentials.as_bytes()).unwrap();
}

fn interactive() -> io::Result<()> {
    use std::io::stdout;

    use crossterm::event::{self, KeyCode, KeyEventKind};
    use crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    };
    use crossterm::ExecutableCommand;
    use ratatui::backend::CrosstermBackend;
    use ratatui::Terminal;
    use ratatui::{
        style::{Color, Style},
        widgets::{Block, BorderType, Borders, Paragraph},
    };

    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();

    let backend = CrosstermBackend::new(stdout());

    let mut terminal = Terminal::new(backend).unwrap();

    enum Step {
        Id,
        Password,
        Done,
    }

    let mut step = Step::Id;
    let mut id = String::new();
    let mut password = String::new();

    let mut render_text = String::new();
    loop {
        render_text.clear();

        match step {
            Step::Id => {
                render_text.push_str("Enter your id: ");
                render_text.push_str(&id);
            }
            Step::Password => {
                render_text.push_str("Enter your password: ");
                render_text.push_str(&"*".repeat(password.len()));
            }
            Step::Done => {
                run(ConfigOptions {
                    id: id.clone(),
                    password: password.clone(),
                    interactive: false,
                });
                break;
            }
        }

        let block = Block::default()
            .title("Login Page")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .border_type(BorderType::Rounded);

        let paragraph = Paragraph::new(render_text.clone()).block(block);

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(paragraph, area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => match step {
                            Step::Id => {
                                id.push(c);
                            }
                            Step::Password => {
                                password.push(c);
                            }
                            Step::Done => {}
                        },
                        KeyCode::Backspace => match step {
                            Step::Id => {
                                id.pop();
                            }
                            Step::Password => {
                                password.pop();
                            }
                            Step::Done => {}
                        },
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Enter => match step {
                            Step::Id => {
                                step = Step::Password;
                            }
                            Step::Password => {
                                step = Step::Done;
                            }
                            Step::Done => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();

    println!("Goodbye!");

    Ok(())
}
