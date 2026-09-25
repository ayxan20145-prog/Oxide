use oxide::{Input, Window};
use sdl3::{keyboard::Keycode, pixels::Color};

fn main() {
    let mut window = Window::create_window("hi", 800, 600);

    let mut input = Input::new();

    loop {
        window.handle_events(&mut input);

        if window.quit {
            break;
        }

        if input.is_key_down(Keycode::W) {
            println!("w pressed");
        }

        if input.is_key_down(Keycode::Space) {
            println!("space pressed");
        }

        window.clear(Color::RGB(255, 255, 255));

        window.present();
    }
}
