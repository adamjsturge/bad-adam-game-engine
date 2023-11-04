extern crate sdl2;

mod object;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use std::collections::HashSet;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_width = 800;
    let window_height = 600;

    let window = video_subsystem.window("adam-engine demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut keys_pressed: HashSet<Keycode> = HashSet::new();

    let mut player = object::Object::new(0, 0, 60, 120);
    let mut player_sprit;//= Rect::new(player.get_x_position(), player.get_y_position(), player.get_width(), player.get_height());

    'running: loop {

        i = (i + 1) % 255;
        
        canvas.set_draw_color(Color::RGB(0, 255, 255)); // Black background
        canvas.clear();

        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    keys_pressed.insert(key);
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    keys_pressed.remove(&key);
                },
                _ => {}
            }            
        }

        // if squareY < window_height as i32 - squareH as i32 {
        //     squareY += 5;
        //     if keys_pressed.contains(&Keycode::S) {
        //         squareY += 10;
        //     }
        // }

        if keys_pressed.contains(&Keycode::S) {
            player.change_y_speed(10);
        }


        if keys_pressed.contains(&Keycode::Space) && player.get_y_position() == window_height as i32 - player.get_height() as i32 {
            // println!("jump");
            player.change_y_speed(-20);
        }

        // if jump_value > 0 {
        //     squareY -= 10;
        //     jump_value -= 10;
        // }

        if keys_pressed.contains(&Keycode::W) {
            player.anti_gravity();
        }
        if keys_pressed.contains(&Keycode::A) {
            player.change_x_speed(-50);
        }
        
        if keys_pressed.contains(&Keycode::D) {
            player.change_x_speed(50);
        }
        
 
        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Red square
        player.calculate_new_position();
        player_sprit = Rect::new(player.get_x_position(), player.get_y_position(), player.get_width(), player.get_height());

        canvas.fill_rect(player_sprit).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}