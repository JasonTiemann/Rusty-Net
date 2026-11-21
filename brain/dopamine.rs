use crate::constants::{DOPAMINE_MULTI, DOPAMINE_TICK};
use eframe::egui;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dopamine {
    Positive,
    Negative,
}

struct DopamineDisplay {
    shared_dopamine: Arc<AtomicF32>,
    send: Sender<Dopamine>,
    get: Reciever<Dopamine>,
}

impl Default for DopamineDisplay {
    fn default() -> Self {
        let (send, get) = mspc::channel();

        Self {
            shared_dopamine: Arc::new(AtomicF32::new(0.0)),
            send,
            get,
        }
    }
}

impl eframe::App for DopamineDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let up_button = ui.add(egui::Button::new("Good"));
            let down_button = ui.add(egui::Button::new("Bad"));

            if up_button.clicked() {
                self.shared_dopamine.store();
            } else if down_button.clicked() {
                self.shared_dopamine.store();
            }
        });
    }
}

fn main() -> eframe::Result {
    let shared_dopamine_value = Arc::new();
}
