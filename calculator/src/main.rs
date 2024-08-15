mod calculator;

use calculator::{Calculator, Inputs};
use eframe::{
    egui::{self},
    run_native, App, NativeOptions,
};

fn main() {
    let app = Calculator::new();
    let win_option = NativeOptions::default();
    run_native("Calculator", win_option, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}

impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let ui = egui::CentralPanel::default();
        ui.show(ctx, |ui| {
            ui.label(&self.display_value);
            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    self.add_input(Inputs::Number(vec![1]))
                }
                if ui.button("2").clicked() {
                    self.add_input(Inputs::Number(vec![2]))
                }
                if ui.button("3").clicked() {
                    self.add_input(Inputs::Number(vec![3]))
                }
                if ui.button("+").clicked() {
                    self.add_input(Inputs::Add)
                }
            });
            dbg!(&self.inputs);
        });
    }
}
