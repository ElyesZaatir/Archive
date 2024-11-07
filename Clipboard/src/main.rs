use std::thread::sleep;
use core::time::Duration;
use arboard::Clipboard;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use screenshots::Screen;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    let screen = Screen::from_point(100, 100).unwrap();
    let list_names = read_lines("File.txt");
    let time = Duration::from_millis(500);
    println!("Starting...");
    sleep(Duration::from_millis(0));
    for name in list_names{
        clipboard.set_text(name.clone()).unwrap();
        paste();
        sleep(time);
        println!("{}",name);
        let image = screen.capture_area(1099, 378, 26, 25).unwrap();
        image.save(format!("{}.png",name)).unwrap();
        delete_all();
    }

}

fn paste(){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::Control, Press);
    let _ = enigo.key(Key::Unicode('v'), Click);
    let _ = enigo.key(Key::Control, Release);
}

/*fn alt_tab(){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::Tab, Press);
    let _ = enigo.key(Key::Tab, Release);
}*/

fn delete_all(){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::Control, Press);
    let _ = enigo.key(Key::Unicode('a'), Click);
    let _ = enigo.key(Key::Control, Release);
    let _ = enigo.key(Key::Backspace, Press);
    let _ = enigo.key(Key::Backspace, Release);
}
