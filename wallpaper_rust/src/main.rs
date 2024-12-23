#![windows_subsystem = "windows"]
extern crate winapi;
use std::{thread, time};
use std::ffi::CString;
use winapi::ctypes::c_void;
use soloud::*;
use winapi::um::winuser::{SystemParametersInfoA, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER};

fn main(){
    let timer = time::Duration::new(1,0);
    loop {
        _ = wallpaper();
        thread::sleep(timer);
    }
}

/*fn sound() -> Result<(), Box<dyn std::error::Error>> {
    let sl = Soloud::default()?;

    let mut wav = audio::Wav::default();

    wav.load(std::path::Path::new("Song.wav"))?;
    sl.play(&wav);
    wallpaper();
    Ok(())
}*/ // including sound from my SoundRust


fn wallpaper() {
    let mut image_path: String = "frame".to_owned();
    let mut image_suffix = 1;
    let image_type: &str = ".jpg";
    let twenty_millis = time::Duration::from_millis(49);
    while image_suffix<781 {
        thread::sleep(twenty_millis);
        unsafe {
            image_path.push_str(&image_suffix.to_string());
            image_path.push_str(image_type);
            let final_image = CString::new(image_path.clone()).unwrap();
            SystemParametersInfoA(
                SPI_SETDESKWALLPAPER,
                0,
                final_image.as_ptr() as *mut c_void,
                SPIF_UPDATEINIFILE,
            );
        }
        image_suffix += 1;
        image_path = "frame".to_string();
    }
}
