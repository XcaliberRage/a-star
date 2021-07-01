use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use std::collections::HashMap;
use ggez::graphics::DrawMode::Stroke;
use ggez::graphics::{DrawMode, Color, Rect};
use glam::*;

const CELL_SIZE: f32 = 87.5; // 700 / 8, the dimensions of the chessboard image

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("game_name", "author_name")
           .build()
           .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    chess_board: ChessBoard,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        let chess_board = ChessBoard::new(_ctx).unwrap();

        MyGame {
            chess_board
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())

    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...
        let dst = glam::Vec2::new(20.0, 20.0);
        let scale = glam::Vec2::new(1.0, 1.0);
        graphics::draw(ctx,
                       &self.chess_board.board_image,
                       graphics::DrawParam::new()
                           .dest(dst)
                           .rotation(0.0)
                           .scale(scale))?;

        graphics::present(ctx)
    }
}

struct ChessBoard {
    board_image: graphics::Image,
    board_data: Vec<Cell>,
}

impl ChessBoard {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        let board_image = graphics::Image::new(ctx, "/data/assets/chess_board.jpg")?;

        let mut m = ChessBoard {
            board_image,
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
