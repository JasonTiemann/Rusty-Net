mod brain;
mod constants;
mod display;

use crate::brain::Brain;
use crate::constants::{INPUT_NEURONS, OUTPUT_NEURONS};
use crate::display::draw;
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout, Write};

fn on_output(input: [f32; INPUT_NEURONS], output: [f32; OUTPUT_NEURONS]) {
    print!("\r{:?}", output);
    io::stdout().flush().unwrap();
}

fn main() {
    let mut brain = Brain::new(true);
    let controller = brain.get_controller();

	brain.start();

	
	enable_raw_mode().unwrap();
	execute!(stdout(), EnterAlternateScreen, Hide).unwrap();

	loop{
		draw(&controller).unwrap(); 

		if event::poll(Duration::from_millis(100)).unwrap() {
			if let Event::Key(key) = event::read().unwrap() {
				if key.kind != KeyEventKind::Press {
					continue;
				}
				match key.code {
					KeyCode::Up    		=> controller.dopamine_up(),
					KeyCode::Down 		=> controller.dopamine_down(),
					KeyCode::Char('1') 	=> controller.nudge_input(0,  0.05),
					KeyCode::Char('q') 	=> controller.nudge_input(0, -0.05),
					KeyCode::Char('2') 	=> controller.nudge_input(1,  0.05),
					KeyCode::Char('w') 	=> controller.nudge_input(1, -0.05),
					KeyCode::Esc 		=> break,
					_ => {}
				}
			}
		}
	}

	execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
	disable_raw_mode().unwrap();

	brain.stop();
}
