use ggez::event;
use ggez::glam::*;
use ggez::graphics;
use ggez::graphics::PxScale;
use ggez::graphics::TextFragment;
use ggez::{Context, GameResult};
use hermanha_chess::PieceType;
use hermanha_chess::Position;

mod network;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

struct MainState {
    board: hermanha_chess::Board,
    selected_piece: Position,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let board = hermanha_chess::Board::start_pos();

        Ok(MainState {
            board,
            selected_piece: Position { row: 4, col: 4 },
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let cerise = graphics::Color::from([0.87, 0.19, 0.39, 1.0]); // cerise by ChatGPT

        let mut canvas = graphics::Canvas::from_frame(ctx, cerise);

        for row in 0..8 {
            for col in 0..8 {
                // dbg!(col);
                // dbg!(75 * col);
                // dbg!(53 + (75 * col));
                let x: f32 = 100.0 + (75.0 * col as f32);
                let y: f32 = 75.0 * (row as f32);

                let square = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {
                        x: 0.0,
                        y: 0.0,
                        w: 75.0,
                        h: 75.0,
                    },
                    if (row + col) % 2 == 0 {
                        graphics::Color::WHITE
                    } else {
                        graphics::Color::BLACK
                    },
                )?;

                canvas.draw(&square, Vec2::new(x, y));

                let pos = Position {
                    row: 7 as i8 - row,
                    col,
                };
                match self.board.get(pos) {
                    Some(piece) => {
                        let draw_color = match piece.color {
                            hermanha_chess::Color::White => "w",
                            hermanha_chess::Color::Black => "b",
                        };

                        match piece.piece_type {
                            hermanha_chess::PieceType::Pawn => {
                                let draw_piece_outer = graphics::Text::new(
                                    TextFragment::new("p")
                                        .scale(PxScale::from(65.0))
                                        .color(cerise),
                                );

                                canvas.draw(&draw_piece_outer, Vec2::new(x + 5.0, y + 5.0));

                                let draw_piece = graphics::Text::new(
                                    TextFragment::new("p").scale(PxScale::from(57.0)).color(
                                        if draw_color == "w" {
                                            graphics::Color::WHITE
                                        } else {
                                            graphics::Color::BLACK
                                        },
                                    ),
                                );

                                canvas.draw(&draw_piece, Vec2::new(x + 7.0, y + 9.0));
                            }
                            hermanha_chess::PieceType::Bishop => {
                                let draw_piece_outer = graphics::Text::new(
                                    TextFragment::new("B")
                                        .scale(PxScale::from(65.0))
                                        .color(cerise),
                                );

                                canvas.draw(&draw_piece_outer, Vec2::new(x + 5.0, y + 5.0));

                                let draw_piece = graphics::Text::new(
                                    TextFragment::new("B").scale(PxScale::from(57.0)).color(
                                        if draw_color == "w" {
                                            graphics::Color::WHITE
                                        } else {
                                            graphics::Color::BLACK
                                        },
                                    ),
                                );

                                canvas.draw(&draw_piece, Vec2::new(x + 7.0, y + 9.0));
                            }
                            hermanha_chess::PieceType::Rook => {
                                let draw_piece_outer = graphics::Text::new(
                                    TextFragment::new("R")
                                        .scale(PxScale::from(65.0))
                                        .color(cerise),
                                );

                                canvas.draw(&draw_piece_outer, Vec2::new(x + 5.0, y + 5.0));

                                let draw_piece = graphics::Text::new(
                                    TextFragment::new("R").scale(PxScale::from(57.0)).color(
                                        if draw_color == "w" {
                                            graphics::Color::WHITE
                                        } else {
                                            graphics::Color::BLACK
                                        },
                                    ),
                                );

                                canvas.draw(&draw_piece, Vec2::new(x + 7.0, y + 9.0));
                            }
                            hermanha_chess::PieceType::Knight => {
                                let draw_piece_outer = graphics::Text::new(
                                    TextFragment::new("k")
                                        .scale(PxScale::from(65.0))
                                        .color(cerise),
                                );

                                canvas.draw(&draw_piece_outer, Vec2::new(x + 5.0, y + 5.0));

                                let draw_piece = graphics::Text::new(
                                    TextFragment::new("k").scale(PxScale::from(57.0)).color(
                                        if draw_color == "w" {
                                            graphics::Color::WHITE
                                        } else {
                                            graphics::Color::BLACK
                                        },
                                    ),
                                );

                                canvas.draw(&draw_piece, Vec2::new(x + 7.0, y + 9.0));
                            }
                            hermanha_chess::PieceType::Queen => {
                                let draw_piece_outer = graphics::Text::new(
                                    TextFragment::new("Q")
                                        .scale(PxScale::from(65.0))
                                        .color(cerise),
                                );

                                canvas.draw(&draw_piece_outer, Vec2::new(x + 5.0, y + 5.0));

                                let draw_piece = graphics::Text::new(
                                    TextFragment::new("Q").scale(PxScale::from(57.0)).color(
                                        if draw_color == "w" {
                                            graphics::Color::WHITE
                                        } else {
                                            graphics::Color::BLACK
                                        },
                                    ),
                                );

                                canvas.draw(&draw_piece, Vec2::new(x + 7.0, y + 9.0));
                            }
                            hermanha_chess::PieceType::King => {
                                let draw_piece_outer = graphics::Text::new(
                                    TextFragment::new("K")
                                        .scale(PxScale::from(65.0))
                                        .color(cerise),
                                );

                                canvas.draw(&draw_piece_outer, Vec2::new(x + 5.0, y + 5.0));

                                let draw_piece = graphics::Text::new(
                                    TextFragment::new("K").scale(PxScale::from(57.0)).color(
                                        if draw_color == "w" {
                                            graphics::Color::WHITE
                                        } else {
                                            graphics::Color::BLACK
                                        },
                                    ),
                                );

                                canvas.draw(&draw_piece, Vec2::new(x + 7.0, y + 9.0));
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let row: f32 = y / 75.0;
        let col: f32 = (x - 100.0) / 75.0;
        let clicked_pos = Position {
            row: 7 as i8 - row as i8,
            col: col as i8,
        };

        println!(
            "selected ? : {} {}",
            self.selected_piece.row, self.selected_piece.col
        );
        println!("pos: {} {}", clicked_pos.row, clicked_pos.col);

        match self.board.get(self.selected_piece) {
            Some(_found_piece) => {
                println!("if");
                if let Err(e) =
                    self.board
                        .move_piece(self.selected_piece, clicked_pos, Some(PieceType::Queen))
                {
                    println!("Failed to move piece: {:?}", e);
                }
            }
            None => (),
        }

        self.selected_piece = clicked_pos;

        Ok(())
    }
}

pub fn main() -> GameResult {
    // code below copied from example on ggez website

    let cb = ggez::ContextBuilder::new("eahla_chess_game_gui", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;

    std::thread::spawn(|| {
        network::start_server("127.0.0.1:6969").unwrap();
    });
    std::thread::sleep(Duration::from_millis(200));

    // dummy client
    std::thread::spawn(|| {
        let mut stream = TcpStream::connect("127.0.0.1:6969").unwrap();
        stream.write_all(b"hey server!").unwrap();

        let mut buf = [0u8; 128];
        if let Ok(n) = stream.read(&mut buf) {
            if n > 0 {
                println!("Client received: {:?}", String::from_utf8_lossy(&buf[..n]));
            }
            std::thread::sleep(Duration::from_millis(1000));
            stream.shutdown(Shutdown::Both).unwrap(); 
        }
    });

    event::run(ctx, event_loop, state)
}
