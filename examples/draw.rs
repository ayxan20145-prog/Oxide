use oxide::{Input, Window};
use sdl3::{pixels::Color, ttf};

fn main() {
    let mut window = Window::create_window("hi", 800, 600);
    let ttf = ttf::init().unwrap();

    let font = ttf
        .load_font("assets/JetBrainsMonoNerdFont-Regular.ttf", 32.0)
        .unwrap();

    let mut input = Input::new();

    loop {
        window.handle_events(&mut input);

        if window.quit {
            break;
        }

        window.clear(Color::RGB(255, 255, 255));

        window.draw_rect(100, 100, 100, 100, Color::RGB(0, 0, 0));
        window.draw_text(&font, "hello", 0, 0, Color::RGB(0, 0, 0));

        window.present();
    }
}
