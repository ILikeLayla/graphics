use screen;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

fn main() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
    }
    // let mut window = screen::sample::WindowSample::new().unwrap();
    let mut window = screen::Window::new().unwrap();
    let _ = window.run();
}