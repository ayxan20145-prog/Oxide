use sdl3::{EventPump, Sdl, event::Event, init, pixels::Color, rect::Rect, render::Canvas, video};

pub struct App {
    pub sdl: Sdl,
    pub canvas: Canvas<video::Window>,
    pub events: EventPump,
}

pub fn create_window(title: &str, width: u32, height: u32) -> App {
    let sdl = init().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window(title, width, height).build().unwrap();

    let events = sdl.event_pump().unwrap();
    let canvas = window.into_canvas();

    App {
        sdl,
        canvas,
        events,
    }
}
pub fn handle_events(app: &mut App) -> bool {
    for event in app.events.poll_iter() {
        match event {
            Event::Quit { .. } => return false,
            _ => {}
        }
    }

    true
}
pub fn clear(app: &mut App, color: Color) {
    app.canvas.set_draw_color(color);
    app.canvas.clear();
}
pub fn draw_rect(app: &mut App, x: i32, y: i32, width: u32, height: u32, color: Color) {
    app.canvas.set_draw_color(color);

    let rect = Rect::new(x, y, width, height);

    app.canvas.fill_rect(rect).unwrap();
}
pub fn present(app: &mut App) {
    app.canvas.present();
}
