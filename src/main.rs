use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Chart, Dataset, GraphType, Borders, Axis, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::collections::VecDeque;

use std::time::Duration;
use ratatui::symbols::Marker;
mod sens;
use sens::Sens;
mod bat;
use bat::Battery;
mod approx;
use approx::sort_graph;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    ratatui::run(run)?;
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut sens = Sens::new()?;
    let mut battery = Battery::new()?;
    let mut vec: Vec<(f64, f64)> = Vec::new();
    let mut vecbat: Vec<(f64,f64)> = Vec::new();
    loop {
        let _ = battery.refresh();
        terminal.draw(|frame| render(frame, &sens, &battery, &mut vec, &mut vecbat))?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, sens: &Sens, battery: &Battery, vec: &mut Vec<(f64,f64)>) {
    // refactor some point
    let pressure = sens.get_pressure();
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35)])
        .split(area);

    let top = chunks[0];
    let top_chunks = Layout::default()
          .direction(Direction::Horizontal) // or Vertical
          .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
          .split(top);

    let wei = top_chunks[0];
    let bat = top_chunks[1];

    let middle = chunks[1];
    let bottom = chunks[2];






    let top_block = Block::default()
        .title("weight")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let top_inner = top_block.inner(wei);
    frame.render_widget(top_block, wei);

    let top_block2 = Block::default()
        .title("bat")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let top_inner2 = top_block2.inner(bat);
    frame.render_widget(top_block2, bat);

    let middle_block = Block::default()
        .title("graph weight view ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let middle_inner = middle_block.inner(middle);
    frame.render_widget(middle_block, middle);

   // top
    let weight = sens.calc_weight();
    let text = format!(
        "Device: {}  \n Pressure: {}  \n Status: Runing \n Folder: {} \n
        Weight: {} kg
        Press 'q' to quit.",

        sens.name, pressure, sens.path, weight,
    );
    let text2 = format!(
        "Status: {}\nCapacity: {:.0}%\nHealth: {:.0}%\nPower: {:+.2} W\nRemaining: {:.2} Wh\nCharge: {:.3} Ah",
        battery.status,
        battery.capacity,
        battery.health,
        battery.watt,
        battery.rm_wh,
        battery.charge_ah,
    );
    let para2 = Paragraph::new(text2).wrap(Wrap { trim: true});
    frame.render_widget(para2, top_inner2);
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, top_inner);

    let bottom_block = Block::default()
        .title("Battery")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let bottom_inner = bottom_block.inner(bottom);
    frame.render_widget(bottom_block, bottom);

    let para_two = format!(
        "Battery: {} | {:.0}% | {:+.2} W | {:.2} Wh | health {:.0}%",
        battery.status,
        battery.capacity,
        battery.watt,
        battery.rm_wh,
        battery.health
    );


    let footer = Paragraph::new(para_two).wrap(Wrap { trim: true });
    frame.render_widget(footer, bottom_inner);
    render_chart(frame, middle_inner, vec, pressure);
    render_chart2(frame, middle_inner, vec2, battery.watt);

}


pub fn render_chart2(frame: &mut Frame, area: Rect, vec2: &mut Vec<(f64, f64)>, pres: f64) {
    let data = sort_graph(vec, pres);
    let dataset = Dataset::default()
        .name("bat")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Green)
        .data(data);

    let x_axis = Axis::default()
        .title("time (obviously)".green())
        .bounds([0.0, 100.0])
        .labels(["0%", "100"]);

    let y_axis = Axis::default()
        .title("".green())
        .bounds([0.0, 1000.0])
        .labels(["0", "1000"]);

    let chart = Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
    frame.render_widget(chart, area);
}




pub fn render_chart(frame: &mut Frame, area: Rect, vec: &mut Vec<(f64, f64)>, pres: i32) {
    let data = sort_graph(vec, pres);
    let dataset = Dataset::default()
        .name("WEIGHT!!!")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Red)
        .data(data);

    let x_axis = Axis::default()
        .title("time (obviously)".red())
        .bounds([0.0, 100.0])
        .labels(["0%", "100"]);

    let y_axis = Axis::default()
        .title("".red())
        .bounds([0.0, 1000.0])
        .labels(["0", "1000"]);

    let chart = Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
    frame.render_widget(chart, area);
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
