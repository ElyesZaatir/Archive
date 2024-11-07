#![windows_subsystem = "windows"]
use core::time;
use std::thread;
use soloud::*;

fn main(){
    let timer = time::Duration::new(1,0);
    loop{
        let _ = sound();
        thread::sleep(timer);
    }
}
fn sound() -> Result<(), Box<dyn std::error::Error>> {
    println!("works");
    let sl = Soloud::default()?;

    let mut wav = audio::Wav::default();

    wav.load(std::path::Path::new(""))?;
    sl.play(&wav);

    Ok(())
}
