use sdl2::video::Window;

use std::sync::{Arc, Mutex};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::FullscreenType::{self, Desktop, Off};
use sdl2::EventPump;

pub mod api;
pub mod data;
pub mod err;
pub mod requests;

use data::{Grid, SharedGrid, RGB, ScreenResolution};

//creates a grid with ncells*ncells initialized with cell in a color
pub fn grid_init(nx_cells: i32, ny_cells: i32) -> SharedGrid {
    let mut grid_vector = Vec::new();

    for row in 0..ny_cells {
        grid_vector.push(Vec::new());
        for _column in 0..nx_cells {
            grid_vector[row as usize].push(RGB {
                red: 35_u8,
                green: 15_u8,
                blue: 13_u8,
            });
        }
    }
    let grid = Grid { grid: grid_vector };

    let output_grid = SharedGrid {
        sharedgrid: Arc::new(Mutex::new(grid)),
    };

    output_grid
}

//converts row column values into xy pixels and draws rectangle in the specified color
pub fn display_cell(
    renderer: &mut Canvas<Window>,
    row: i32,
    col: i32,
    grid_data: &Grid,
    cell_width: &i32,
) {
    let cell_height = cell_width;

    let grid = &grid_data.grid;

    let x = cell_width * col;
    let y = cell_width * row;

    let cell_color = &grid[row as usize][col as usize];
    let drawing_color = Color::RGB(cell_color.red, cell_color.green, cell_color.blue);

    renderer.set_draw_color(drawing_color);
    let square = renderer.fill_rect(Rect::new(x, y, *cell_width as u32, *cell_height as u32));
    match square {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }
}

//displays the whole grid by repeatedly calling display_cell on every cell
pub fn display_frame(
    renderer: &mut Canvas<Window>,
    shared_grid: &SharedGrid,
    nx_cells: &i32,
    ny_cells: &i32,
    cell_width: &i32,
) {
    let sharedgrid_data = &shared_grid.sharedgrid;
    let grid_data = sharedgrid_data.lock().expect("grid lock failed");

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    for row in 0..*ny_cells {
        for column in 0..*nx_cells {
            display_cell(renderer, row, column, &grid_data, &cell_width)
        }
    }
    renderer.present();
}

pub fn toggle_fullscreen(canvas: &mut Canvas<Window>, canvas_width: i32, canvas_height: i32) {
    if canvas.window_mut().fullscreen_state() == FullscreenType::Off {
        canvas.window_mut().set_fullscreen(Desktop).unwrap();

        //change viewport for fullscreen
        let screen_resolution = get_screen_resolution(canvas);
        let center_rect = center_rect(
            screen_resolution.w,
            screen_resolution.h,
            canvas_width,
            canvas_height,
        );

        canvas.set_viewport(center_rect);
    } else {
        canvas.window_mut().set_fullscreen(Off).unwrap();
    };
}

pub fn init<'a>(x: i32, y: i32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Squares", x as u32 + 1, y as u32 + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

pub fn get_screen_resolution(canvas: &mut Canvas<Window>) -> ScreenResolution {
    let window = canvas.window_mut();
    let video_subsystem = window.subsystem();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let width = display_mode.w;
    let height = display_mode.h;

    let screen_resolution = ScreenResolution {
        w: width,
        h: height,
    };
    screen_resolution
}

pub fn clear_grid(shared_grid: &SharedGrid) {
    println!("clearing grid");

    let mut sharedgrid_data = shared_grid.sharedgrid.lock().expect("grid lock failed");
    let max_rows = &sharedgrid_data.grid.len();
    let max_columns = &sharedgrid_data.grid[0].len();

    for row in 0..*max_rows as i32 {
        for column in 0..*max_columns as i32 {
            sharedgrid_data.grid[row as usize][column as usize] = RGB {
                red: 35_u8,
                green: 15_u8,
                blue: 13_u8,
            };
        }
    }
}

pub fn make_checker_board(shared_grid: &SharedGrid) {
    println!("clearing grid");

    let mut sharedgrid_data = shared_grid.sharedgrid.lock().expect("grid lock failed");
    let max_rows = &sharedgrid_data.grid.len();
    let max_columns = &sharedgrid_data.grid[0].len();

    for row in 0..*max_rows as i32 {
        if row % 2 == 0 {
            for column in 0..*max_columns as i32 {
                if column % 2 == 0 {
                    sharedgrid_data.grid[row as usize][column as usize] = RGB {
                        red: 255,
                        green: 255,
                        blue: 255,
                    };
                }
            }
        } else {
            for column in 0..*max_columns as i32 {
                if column % 2 == 1 {
                    sharedgrid_data.grid[row as usize][column as usize] = RGB {
                        red: 255,
                        green: 255,
                        blue: 255,
                    };
                }
            }
        }
    }
}

pub fn center_rect(res_width: i32, res_height: i32, canvas_width: i32, canvas_height: i32) -> Rect {
    let x = (res_width - canvas_width) / 2;
    let y = (res_height - canvas_height) / 2;
    let center_rect = Rect::new(x, y, res_width as u32, res_height as u32);

    center_rect
}

pub fn determine_canvas_size(nx_cells: i32, ny_cells: i32) -> (i32, i32, i32) {
    let (mut canvas, mut _events) = init(100, 100);
    let screen_resolution = get_screen_resolution(&mut canvas);

    if nx_cells == ny_cells {
        let canvas_height = screen_resolution.h - 200;
        let canvas_width = canvas_height;
        let cell_width = canvas_height / ny_cells;
        (canvas_width, canvas_height, cell_width)
    } else {
        let canvas_height = screen_resolution.h - 200;
        let cell_width = canvas_height / ny_cells;
        let canvas_width = cell_width * nx_cells;
        (canvas_width, canvas_height, cell_width)
    }
}
