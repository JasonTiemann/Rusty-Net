use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout, Write};
use crate::brain::BrainControl;

fn bar(v: f32, width: usize) -> String {
    let filled = (v.clamp(0.0, 1.0) * width as f32).round() as usize;
    format!("[{}{}]", "#".repeat(filled), "-".repeat(width - filled))
}

pub fn draw(ctl: &BrainControl) -> io::Result<()> {
    let mut out = stdout();
    let mut lines = vec![
        "Up/Down: dopamine   1/q, 2/w: inputs   Esc: quit".to_string(),
        String::new(),
        format!("Dopamine: {:+7.3}", ctl.dopamine_get()),
        String::new(),
        "Inputs".to_string(),
    ];
    for (i, v) in ctl.get_inputs().iter().enumerate() {
        lines.push(format!("  in[{i}]  {} {:.2}", bar(*v, 30), v));
    }
    lines.push(String::new());
    lines.push("Outputs".to_string());
    for (i, v) in ctl.get_outputs().iter().enumerate() {
        lines.push(format!("  out[{i}] {} {:.2}", bar(*v, 30), v));
    }

    for (row, line) in lines.iter().enumerate() {
        queue!(out, MoveTo(0, row as u16), Print(line), Clear(ClearType::UntilNewLine))?;
    }
    queue!(out, Clear(ClearType::FromCursorDown))?;
    out.flush()
}