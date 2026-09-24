use oxide::{clear, create_window, draw_rect, handle_events, present};
use sdl3::pixels::Color;

fn main() {
    let mut app = create_window("hi", 800, 600);

    loop {
        if !handle_events(&mut app) {
            break;
        }

        clear(&mut app, Color::RGB(255, 255, 255));

        draw_rect(&mut app, 0, 0, 100, 100, Color::RGB(0, 0, 0));

        present(&mut app);
    }
}
