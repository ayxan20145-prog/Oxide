pub fn create_window() {
    let sdl = sdl3::init().unwrap();
    let video = sdl.video().unwrap();

    let _window = video.window("test", 800, 600).build().unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
