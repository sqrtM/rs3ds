use std::{process::Command, string};

use ctru::prelude::*;

mod map;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();

    let wide_mode = gfx.top_screen.borrow().get_wide_mode();
    gfx.top_screen.borrow_mut().set_wide_mode(!wide_mode);
    let top_screen = Console::init(gfx.top_screen.borrow_mut());
    let bottom_screen = Console::init(gfx.bottom_screen.borrow_mut());

    let mut player_x: f64 = 8.0f64;
    let mut player_y: f64 = 8.0f64;
    let mut player_angle: f64 = 0.0f64;

    const screen_width: i32 = 100;
    const screen_height: i32 = 29;

    let fov: f64 = 3.14159 / 4.0f64;
    let depth:f64 = 16.0;

    let mut screenview: [[&str; screen_width as usize]; screen_height as usize] = [[" "; screen_width as usize]; screen_height as usize];
    
    top_screen.select();

    while apt.main_loop() {
        // set up the buffer
        gfx.flush_buffers();
        //gfx.swap_buffers();
        //gfx.wait_for_vblank();

        bottom_screen.select();
        println!("\x1b[00;00H");
        for i in 0..map::TILE_MAP.len() {
            for j in 0..map::TILE_MAP[i].len() {
                print!("\x1b[5m{}", map::TILE_MAP[i][j].default_char);
            }
            println!("");
        }
        top_screen.select();
        println!("\x1b[00;00H\x1b[0m");
            for x in 0..screen_width {
                let ray_angle: f64 = (player_angle - fov / 2.0) * (x as f64 / screen_width as f64) * fov;
                let mut distance_to_wall: f64 = 0.0f64;
                let mut hit_wall: bool = false;
                let eye_x: f64 = ray_angle.sin();
                let eye_y: f64 = ray_angle.cos();
    
                while !hit_wall && distance_to_wall < depth {
                    distance_to_wall += 0.1f64;
    
                    let test_x: i32 = (player_x + eye_x * distance_to_wall) as i32;
                    let test_y: i32 = (player_y + eye_y * distance_to_wall) as i32;
    
                    if test_x < 0 || test_x >= 16 || test_y < 0 || test_y >= 16 {
                        hit_wall = true;
                        distance_to_wall = depth;
                    } else {
                        if map::TILE_MAP[test_y as usize][test_x as usize].name == map::Tiles::Wall {
                            hit_wall = true;
                        }
                    }
                }
                let ceiling: i32 = ((screen_height / 2) as f64 - (screen_height as f64 / distance_to_wall)) as i32 ;
                let floor: i32 = screen_height - ceiling;
    
                for y in 0..screen_height {
                    let b = true;
                    let shade: &str = match distance_to_wall {
                        i if i <= depth / 4.0 => "\x1b[47m#", // very close
                        i if i <= depth / 3.0 => "\x1b[1m#",
                        i if i <= depth / 2.0 => "#",
                        i if i <= depth => "\x1b[2m#",
                        _ => " ", // very far
                    };  
                    if y < ceiling {
                        screenview[y as usize][x as usize] = " ";
                    } else if y > ceiling && y <= floor {
                        screenview[y as usize][x as usize] = shade;
                    } else {
                        screenview[y as usize][x as usize] = " ";
                    }
                }
            }
            for i in 0..screenview.len() {
                for j in 0..screenview[i].len() {
                    print!("\x1b[5m{}", screenview[i][j]);
                }
            }


        // check inputs
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        if hid.keys_held().contains(KeyPad::KEY_DLEFT) {
            player_angle -= (12.0f64);
        }
        if hid.keys_held().contains(KeyPad::KEY_DRIGHT) {
            player_angle += (12.0f64);
        }
        if hid.keys_held().contains(KeyPad::KEY_DUP) {
            player_x += player_angle.sin();
            player_y += player_angle.cos();
        }
        if hid.keys_held().contains(KeyPad::KEY_DDOWN) {
            player_x -= player_angle.sin();
            player_y -= player_angle.cos();
        }

        //map::TILE_MAP[player_y as i32 as usize][player_x as i32 as usize] = map::tile(map::Tiles::Floor, '@');
    }
}
