use std::sync::{Arc, Mutex};
use crate::constants::{DOPAMINE_REGRESS_RATE, DOPAMINE_MULTI};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feedback {
    Good,
    Bad,
}

/// Cloneable handle. The value is private; the only way to read it is `step()`.
#[derive(Clone)]
pub struct Dopamine {
    value: Arc<Mutex<f32>>
}

impl Dopamine {
    pub fn new() -> Self {
        Self { value: Arc::new(Mutex::new(0.0))}
    }

    /// Decays the value toward zero, then returns it.
    /// Calling this more often makes the value regress to zero faster.
    pub fn step(&self) -> f32 {
        let mut v = self.value.lock().unwrap();
        *v *= 1.0 - DOPAMINE_REGRESS_RATE;
        *v
    }

	pub fn get(&self) -> f32 {
		*self.value.lock().unwrap()
	}

    pub fn up(&self) {
        *self.value.lock().unwrap() = DOPAMINE_MULTI;
    }

    pub fn down(&self){
        *self.value.lock().unwrap() = -1.0 * DOPAMINE_MULTI
    }
}