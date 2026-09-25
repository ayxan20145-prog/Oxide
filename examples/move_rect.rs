use oxgfx::{Input, Window};
use sdl3::{keyboard::Keycode, pixels::Color};

fn main() {
    let mut x = 0;
    let mut y = 0;

    let mut window = Window::create_window("hi", 800, 600);

    let mut input = Input::new();

    loop {
        window.handle_events(&mut input);

        if window.quit {
            break;
        }

        if input.is_key_down(Keycode::Right) {
            if x < 700 {
                x += 1;
            }
        }
        if input.is_key_down(Keycode::Left) {
            if x > 0 {
                x -= 1;
            }
        }
        if input.is_key_down(Keycode::Down) {
            if y < 500 {
                y += 1;
            }
        }
        if input.is_key_down(Keycode::Up) {
            if y > 0 {
                y -= 1;
            }
        }

        window.clear(Color::RGB(255, 255, 255));

        window.draw_rect(x, y, 100, 100, Color::RGB(0, 0, 0));

        window.present();
    }
}
