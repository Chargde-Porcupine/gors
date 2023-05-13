use crate::Direction::{DOWN, LEFT, NONE, RIGHT, UP};
use crate::IntersectionState::BLACK;
use crate::IntersectionState::EMPTY;
use crate::IntersectionState::WHITE;
use std::fmt;

//hello github!

#[derive(Clone, PartialEq, Copy)]
enum IntersectionState {
    EMPTY,
    BLACK,
    WHITE,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

impl fmt::Debug for IntersectionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EMPTY => write!(f, " "),
            BLACK => write!(f, "●"),
            WHITE => write!(f, "○"),
        }
    }
}

/*
functions needed:
 -make a move given(done)
 -check liberties(done)
 -clear dead stones
 (should happen after each turn on the stone placed and those around it
 */

fn get_stone(
    x: usize,
    y: usize,
    board: &mut Vec<Vec<IntersectionState>>,
) -> Option<IntersectionState> {
    match board.get(y) {
        None => None,
        Some(val) => match val.get(x) {
            None => None,
            Some(val) => Some(val.to_owned()),
        },
    }
}

fn add_stone(
    x: usize,
    y: usize,
    color: IntersectionState,
    board: &mut Vec<Vec<IntersectionState>>,
) -> Result<Vec<Vec<IntersectionState>>, &'static str> {
    match get_stone(x, y, board) {
        None => Err("Index out of bounds"),
        Some(val) => {
            if val.to_owned() == EMPTY {
                board[y][x] = color;
                Ok(board.to_vec())
            } else {
                Err("Cannot place at non-empty space.")
            }
        }
    }
}

fn check_liberties(
    x: usize,
    y: usize,
    board: &mut Vec<Vec<IntersectionState>>,
    direction: Direction,
    color: IntersectionState,
) -> bool {
    match get_stone(x, y, board) {
        None => false,
        Some(val) => {
            if val == EMPTY {
                true
            }else if val == color{
                match direction {
                    NONE => {
                        check_liberties(x + 1, y, board, RIGHT, color)
                            || (if x == 0 {
                            false
                        } else {
                            check_liberties(x - 1, y, board, LEFT, color)
                        })
                            || (if y == 0 {
                            false
                        } else {
                            check_liberties(x, y - 1, board, UP, color)
                        })
                            || check_liberties(x, y + 1, board, DOWN, color)
                    }
                    UP => {
                        check_liberties(x + 1, y, board, RIGHT, color)
                            || (if x == 0 {
                            false
                        } else {
                            check_liberties(x - 1, y, board, LEFT, color)
                        })
                            || (if y == 0 {
                            false
                        } else {
                            check_liberties(x, y - 1, board, UP, color)
                        })
                    }
                    DOWN => {
                        check_liberties(x + 1, y, board, RIGHT, color)
                            || (if x == 0 {
                            false
                        } else {
                            check_liberties(x - 1, y, board, LEFT, color)
                        })
                            || check_liberties(x, y + 1, board, DOWN, color)
                    }
                    RIGHT => {
                        check_liberties(x + 1, y, board, RIGHT, color)
                            || check_liberties(x, y + 1, board, DOWN, color)
                            || (if y == 0 {
                            false
                        } else {
                            check_liberties(x, y - 1, board, UP, color)
                        })
                    }
                    LEFT => {
                        (if x == 0 {
                            false
                        } else {
                            check_liberties(x - 1, y, board, LEFT, color)
                        }) || if y == 0 {
                            false
                        } else {
                            check_liberties(x, y - 1, board, UP, color)
                        } || check_liberties(x, y + 1, board, DOWN, color)
                    }
                }
            } else {
                false
            }

        }
    }
}

fn main() {
    let mut board: Vec<Vec<IntersectionState>> = vec![vec![EMPTY; 9]; 9];
    println!("Hello, world!");
    board = match add_stone(1, 1, WHITE, &mut board) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            board
        }
    };
    println!("{:?}", check_liberties(1, 1, &mut board, NONE, WHITE));
    for i in board {
        println!("{:?}", i);
    }
}
