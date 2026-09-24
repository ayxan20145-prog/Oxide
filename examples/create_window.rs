use oxide::Window;
use sdl3::pixels::Color;

fn main() {
    let mut window = Window::create_window("hi", 800, 600);

    loop {
        if !window.handle_events() {
            break;
        }

        window.clear(Color::RGB(255, 255, 255));

        window.draw_rect(0, 0, 100, 100, Color::RGB(0, 0, 0));

        window.present();
    }
}
