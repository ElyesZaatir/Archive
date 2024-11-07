use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    task: String,
    time: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            task: "Wash yo face".to_owned(),
            time: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reminder");
            ui.horizontal(|ui| {
                let task_label = ui.label("Your task: ");
                ui.text_edit_singleline(&mut self.task)
                    .labelled_by(task_label.id);
            });
            ui.add(egui::Slider::new(&mut self.time, 0..=120).text("mins"));
            if ui.button("Increment").clicked() {
                self.time += 1;
            }
            ui.label(format!("do '{}' after {}mins", self.task, self.time));

        });
    }
}
