#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]

extern crate error_chain;
extern crate sdl2;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod lib;

use lib::data::SharedGrid;
use lib::requests;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CommandLineArgs {
    rows: i32,
    columns: i32,
}

fn main() {
    let args = CommandLineArgs::from_args();
    let program_paused = Arc::new(AtomicBool::new(false));
    let program_paused_state = program_paused.clone();

    //init video loop
    let (canvas_width, canvas_height, cell_width) =
        lib::determine_canvas_size(args.columns, args.rows);

    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);

    let shared_grid = lib::grid_init(args.columns, args.rows);
    let sharedgrid_rocket = SharedGrid {
        sharedgrid: shared_grid.sharedgrid.clone(),
    };

    thread::spawn(|| {
        //http requests
        //if no data is comming over http, init color is drawn
        rocket::ignite()
            .mount("/grid", routes![requests::new_grid])
            .mount("/cell", routes![requests::add_cell])
            .mount("/line", routes![requests::add_line])
            .mount("/", routes![requests::intervention])
            .manage(sharedgrid_rocket)
            .manage(program_paused_state)
            .launch();
    });

    //video loop
    'running: loop {
        let mut sharedgrid_loop = SharedGrid {
            sharedgrid: shared_grid.sharedgrid.clone(),
        };
        //
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    lib::toggle_fullscreen(&mut canvas, canvas_width, canvas_height);
                    continue 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    lib::clear_grid(&mut sharedgrid_loop);
                    continue 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    if program_paused.load(Ordering::Relaxed) == false {
                        program_paused.store(true, Ordering::Relaxed);
                        println!("paused");
                    } else {
                        program_paused.store(false, Ordering::Relaxed);
                        println!("unpaused");
                    }
                    continue 'running;
                }

                _ => continue 'running,
            }
        }

        if program_paused.load(Ordering::Relaxed) == false {
            lib::display_frame(
                &mut canvas,
                &sharedgrid_loop,
                &args.columns,
                &args.rows,
                &cell_width,
            );
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}
