use super::super::sdl2;

use std::process;
use std::collections::HashMap;
use sdl2::rect::{Rect};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode};
use core::timer::{Timers, TimerType};

pub fn game_loop() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window("hardhead", 640, 480)
        .position_centered()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut rect = Rect::new(10, 10, 10, 10);
    
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = sdl_context.event_pump().unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut x_coord = 0.0;
    let mut y_coord = 0.0;

    let mut counted_frames = 0;
    let mut timers = Timers::new();
    let mut window_is_active = true;
    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::FocusGained => {
                            timers.set_paused(false);
                            window_is_active = true;
                        },
                        WindowEvent::FocusLost => {
                            timers.set_paused(true);
                            window_is_active = false;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        timers.update();

        if window_is_active {
            let main_timer = timers.get_standard_timer(TimerType::Gameplay).unwrap();
            let delta = main_timer.get_delta();

            let fps = ((counted_frames as f32) / main_timer.get_secs_since_boot()).round();
            println!("{}", fps);
            println!("{}, {}", counted_frames, main_timer.get_secs_since_boot());
            let mut keys: HashMap<Keycode, bool> = HashMap::new();
            let sdl_keys_state = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect::<Vec<Keycode>>();
            for key in sdl_keys_state {
                keys.entry(key).or_insert(true);
            }

            let amount_to_move = 100.0 * delta;

            if keys.contains_key(&Keycode::Left) {
                x_coord -= amount_to_move;
            }
            if keys.contains_key(&Keycode::Right) {
                x_coord += amount_to_move;
            }
            if keys.contains_key(&Keycode::Up) {
                y_coord -= amount_to_move;
            }
            if keys.contains_key(&Keycode::Down) {
                y_coord += amount_to_move;
            }
            rect.x = x_coord as i32;
            rect.y = y_coord as i32;

            canvas.set_draw_color(black);
            canvas.clear();
            canvas.set_draw_color(white);
            let _ = canvas.fill_rect(rect);
            canvas.present();
            counted_frames += 1;
        }
    };
}
