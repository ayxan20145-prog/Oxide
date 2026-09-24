pub fn create_window() {
    let sdl = sdl3::init().unwrap();
    let video = sdl.video().unwrap();

    let _window = video.window("test", 800, 600).build().unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut canvas = _window.into_canvas();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. } => break 'running,
                _ => {}
            }

            canvas.set_draw_color(sdl3::pixels::Color::RGB(255, 255, 255));
            canvas.clear();
            canvas.present();
        }
    }
}
