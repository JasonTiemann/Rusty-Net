use eframe::egui;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};


#[derive(Debug, Clone, Copy)]
enum Dopamine {
    Positive,
    Negative,
}


struct OutputDisplay{
    dopamine: f32,
    shared_dopamine: Arc<AtomicF32>,
}

impl eframe::App for OutputDisplay{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            let slider = ui.add(egui::Slider::new(&mut self.dopamine, -5.0..5.0)).text("Dopamine")
            
            if slider.changed(){
                self.shared_dopamine.store(self.dopamine, Ordering::Relaxed)
            }
        });
    }
}

fn main() -> eframe::Result{
    let shared_dopamine_value = Arc::new();
}