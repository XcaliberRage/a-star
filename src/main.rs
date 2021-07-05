use glam::*;

use ggez;
use mint;
use ggez::{Context, ContextBuilder, GameResult, conf};
use ggez::event;
use ggez::timer;
use ggez::graphics;
use std::collections::HashMap;
use ggez::graphics::DrawMode::Stroke;
use ggez::graphics::{DrawMode, Color, Rect, Drawable, BLACK};
use ggez::event::EventHandler;
use std::env;
use std::path;
use nalgebra::Vector2;
use ggez::conf::{WindowSetup, WindowMode, FullscreenType, NumSamples};

type Point2 = glam::Vec2;

const CELL_SIZE: f32 = 87.5; // 700 / 8, the dimensions of the chessboard image

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
    resize_projection: bool,
    win_mo: WindowMode,
}

struct MainState {
    board: ChessBoard,
    window_settings: WindowSettings,
    canvas: graphics::Canvas,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {


        let board = ChessBoard::new(ctx)?;

        let win_mo = WindowMode {
            width: board.board_dimensions.w + 50.0,
            height: board.board_dimensions.h + 50.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: board.board_dimensions.w + 50.0,
            min_height: board.board_dimensions.h + 50.0,
            max_width: 0.0,
            max_height: 0.0,
            resizable: true
        };
        graphics::set_mode(ctx, win_mo)?;
        graphics::set_drawable_size(ctx, board.board_dimensions.w, board.board_dimensions.h)?;
        let canvas = graphics::Canvas::with_window_size(ctx)?;

        let s = MainState {
            board,
            window_settings: WindowSettings {
                toggle_fullscreen: false,
                is_fullscreen: false,
                resize_projection: false,
                win_mo,
            },
            canvas,
        };




        Ok(s)
    }

}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        const DESIRED_FPS: u32 = 120;
        while timer::check_update_time(_ctx, DESIRED_FPS) {

            if self.window_settings.toggle_fullscreen {
                let fullscreen_type = if self.window_settings.is_fullscreen {
                    conf::FullscreenType::Desktop
                } else {
                    conf::FullscreenType::Windowed
                };
                ggez::graphics::set_fullscreen(_ctx, fullscreen_type)?;
                self.window_settings.toggle_fullscreen = false;
            }

        }

        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from((64, 0, 0, 0)));

        graphics::set_canvas(ctx, Option::Some(&self.canvas));
        graphics::clear(ctx, graphics::Color::from((255, 255, 255, 128)));

        let dst = nalgebra::Point2::new(25.0, 25.0);
        let scale = mint::Vector2::from([1.0, 1.0]);

        graphics::draw(
            ctx,
            &self.board.board_image,
            graphics::DrawParam::new()
                .dest(dst)
                .scale(scale)
                .rotation(0.0)
                .src(self.board.board_dimensions)
        )?;
        graphics::set_canvas(ctx, None);
        graphics::draw(
            ctx,
            &self.canvas,
            graphics::DrawParam::new().color(Color::from((255, 255, 255,128)))
        )?;

        graphics::present(ctx)
    }
}

struct ChessBoard {
    board_image: graphics::Image,
    board_dimensions: Rect,
    board_data: Vec<Cell>,
}

impl ChessBoard {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        let mut board_image = graphics::Image::new(ctx, "/chess_board.jpg")?;
        let board_dimensions = board_image.dimensions();

        let mut m = ChessBoard {
            board_image,
            board_dimensions,
            board_data: Vec::new(),
        };

        for rank in 1..=8 {
            for file in 1..=8 {
                m.board_data.push(Cell::new(rank, file, ctx)?)
            }
        }

        Ok(m)

    }
}

struct Cell {
    rank: u32,
    file: u32,
    mesh: graphics::Mesh
}

impl Cell {
    fn new(rank: u32, file: u32, ctx: &mut Context) -> GameResult<Cell> {

        let mesh = build_cell_mesh(ctx, Cell::vec_as_coord(rank), Cell::vec_as_coord(file))?;

        Ok(Cell {
            rank,
            file,
            mesh
        })
    }

    pub fn get_file_as_char(&self) -> char {

        match self.rank {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => panic!("Impossible rank! received {}", self.rank)
        }

    }

    pub fn vec_as_coord(vec: u32) -> f32 {
        (vec - 1) as f32 * CELL_SIZE
    }
}

fn build_cell_mesh(ctx: &mut Context, y: f32, x: f32) -> GameResult<graphics::Mesh>{

    let mb = &mut graphics::MeshBuilder::new();

    mb.rectangle(
        DrawMode::stroke(0.1),
        Rect::new(x, y, CELL_SIZE, CELL_SIZE),
        Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    );
    
    mb.build(ctx)

}

fn main() -> GameResult {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Make a Context and an EventLoop.
    let mut c =
       ContextBuilder::new("game_name", "XcaliberRage")
           .add_resource_path(resource_dir);

    let (mut ctx, mut event_loop) = c.build()?;

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut game = MainState::new(&mut ctx)?;

    dbg!(game.board.board_dimensions);
    dbg!(game.canvas.dimensions(&mut ctx));


    // Run!
    event::run(&mut ctx, &mut event_loop,&mut game)
}