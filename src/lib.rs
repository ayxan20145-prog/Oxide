pub fn create_window() {
    let sdl = sdl3::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("test", 800, 600).build().unwrap();

    loop {}
}
