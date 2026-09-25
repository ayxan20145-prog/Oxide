use oxgfx::{Input, Window};
use sdl3::pixels::Color;

fn main() {
    let mut window = Window::create_window("hi", 800, 600);

    let mut input = Input::new();

    loop {
        window.handle_events(&mut input);

        if window.quit {
            break;
        }

        window.clear(Color::RGB(255, 255, 255));

        window.present();
    }
}
