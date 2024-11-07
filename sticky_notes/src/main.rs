use eframe::egui;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::path::Path;
use substring::Substring;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([240.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Stinky Notes",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Sticky>::default())
        }),
    )
}

struct Sticky {
    content: String,
    filename: String,
}

impl Default for Sticky {
    fn default() -> Self {
        Self {
            content: "Contents".to_string(),
            filename: "Filename".to_string(),
        }
    }
}

impl eframe::App for Sticky {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.content);
            ui.text_edit_singleline(&mut self.filename);
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let _sef = save(self.content.clone(),self.filename.clone());
                };
                if ui.button("Load").clicked() {
                    self.content = load(self.filename.clone());        
                }
                if ui.button("Show Files").clicked() {
                    self.content = show();
                }
            });    
        });
    }
}
/*fn ui_example_system(mut contexts: EguiContexts,) {
    egui::Window::new(ui.text_edit_singleline(&mut Header)).show(contexts.ctx_mut(), |ui| {
        ui.text_edit_multiline(&mut my_string);
    });
}*/
fn save(content:String,filename:String) -> io::Result<()> {
    if !Path::new("./StickyNotes").exists(){
        fs::create_dir("./StickyNotes")?;
    }
    let mut filepath = r".\StickyNotes\".to_owned();
    filepath.push_str(&filename);
    let mut buffer = File::create(filepath)?;
    buffer.write_all(content.as_bytes())?;
    Ok(())
}

fn load(filename:String) -> String {
    let mut filepath = r".\StickyNotes\".to_owned();
    filepath.push_str(&filename);
    if !Path::new(&filepath).exists() {
        let failure:String = "Unable to find file (Error) !".to_string();
        return failure
    }
    let contents:String = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    contents
}
fn show() -> String{
    let paths = fs::read_dir(r".\StickyNotes\").unwrap();
    let mut message:String = "Files stored: \n\n".to_string();
    for path in paths {
        let a = path.unwrap().path().display().to_string();
        message.push_str(a.substring(15,a.chars().count()));
        message.push_str("\n");
    }
    message
}
