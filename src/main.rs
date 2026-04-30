use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::time::Duration;

mod sens;
use sens::Sens;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    ratatui::run(run)?;
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let sens = Sens::new()?;
    loop {
        terminal.draw(|frame| render(frame, &sens))?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, sens: &Sens) {
    let pressure = sens.get_pressure();
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30)])
        .split(area);

    let top = chunks[0];
    let middle = chunks[1];
    let bottom = chunks[2];

    let top_block = Block::default()
        .title("Sense")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let top_inner = top_block.inner(top);
    frame.render_widget(top_block, top);

    let middle_block = Block::default()
        .title("graph weight view ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let middle_inner = middle_block.inner(middle);
    frame.render_widget(middle_block, middle);




    let weight = sens.calc_weight();
    let text = format!(
        "Device: {}\nPressure: {}\nFolder: /dev/input/event2
        Weight: {} kg
        \n\nPress 'q' to quit.",

        sens.name, pressure, weight
    );
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, top_inner);

    let bottom_block = Block::default()
        .title("Info")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let bottom_inner = bottom_block.inner(bottom);
    frame.render_widget(bottom_block, bottom);

    let para_two = format!("Status: running \n State: {}", get_status(sens));


    let footer = Paragraph::new(para_two).wrap(Wrap { trim: true });
    frame.render_widget(footer, bottom_inner);
}
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(125))? {
        let ev = event::read()?;
        if let Event::Key(key) = ev {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn get_status(sens: &Sens) -> &str {
    let str: i32 = sens.get_pressure();
    match str {
        0..=10 => "put finger on haptic pad",
        11..=30 => "place object: try to minimize force",
        31..=9999 => "Okkei",
        _ => "no force exerted",

    }
}
