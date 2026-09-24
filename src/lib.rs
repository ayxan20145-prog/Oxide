use sdl3::{event::Event, init, pixels::Color, rect::Rect, ttf};

pub fn create_window() {
    let sdl = init().unwrap();
    let video = sdl.video().unwrap();
    let ttf = ttf::init().unwrap();

    let window = video.window("test", 800, 600).build().unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut canvas = window.into_canvas();

    let font = ttf
        .load_font("assets/JetBrainsMonoNerdFont-Regular.ttf", 32.0)
        .unwrap();

    let surface = font.render("hello").blended(Color::RGB(0, 0, 0)).unwrap();

    let idk = canvas.texture_creator();
    let texture = idk.create_texture_from_surface(&surface).unwrap();

    let text_rect = Rect::new(0, 0, surface.width(), surface.height());

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas.copy(&texture, None, text_rect).unwrap();

        canvas.present();
    }
}
