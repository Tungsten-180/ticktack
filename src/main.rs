use std::io;

////////////////////////////////////////////
const MAP: [[Perm; 3]; 3] = [
    [Perm::C, Perm::S, Perm::C],
    [Perm::S, Perm::O, Perm::S],
    [Perm::C, Perm::S, Perm::C],
];

fn map(cord: [usize; 2]) -> Perm {
    match MAP[cord[0]][cord[1]] {
        Perm::C => Perm::C,
        Perm::O => Perm::O,
        Perm::S => Perm::S,
        _ => Perm::Err,
    }
}

const TRC: [usize; 2] = [0, 0];
const TLC: [usize; 2] = [0, 2];
const BRC: [usize; 2] = [2, 2];
const BLC: [usize; 2] = [2, 0];
const ST: [usize; 2] = [0, 1];
const SB: [usize; 2] = [2, 1];
const SR: [usize; 2] = [1, 0];
const SL: [usize; 2] = [1, 2];
const O: [usize; 2] = [1, 2];

const SS: [[usize; 2]; 4] = [SR, SL, ST, SB];
const CC: [[usize; 2]; 4] = [TRC, TLC, BRC, BLC];

////
fn s_s(moves: &[[usize; 2]; 5]) -> [usize; 2] {
    let mut out = [99, 99usize];
    for k in SS {
        for i in moves {
            if i != &k {
                out = k;
            } else {
            }
        }
    }
    out
}
fn c_c(moves: &[[usize; 2]; 5]) -> [usize; 2] {
    let mut out = [99, 99usize];
    for k in CC {
        for i in moves {
            if i != &k {
                out = k;
            }
        }
    }
    out
}
////
enum Situation {
    Onewin,
    Onelose,
    None,
    Cat,
}
enum Perm {
    CF,
    CA,
    SF,
    SA,
    S,
    C,
    O,
    Err,
    Null,
}

struct State {
    smove1: Perm,
    omove1: Perm,
    boardpermuation: Perm,
}
struct Game {
    cord: [usize; 2],
    epieces: [[usize; 2]; 5],
    board: [[i8; 3]; 3],
    state: State,
    turn: usize,
}
//////////////////////////////////////////////

///////////////////////////////////////////
fn self_move(game: Game) -> Game {
    match game.turn {
        2 => board_2(game),
        _ => game, //4=>board_4(game),
                   //6=>board_6(game),
                   //8=>board_8(game),
    }
}
///////////////////////////////////////
//// STATE OF THE GAME
fn state(game: Game) -> Situation {
    let mut rows = [99usize,101,102,103,104];
    let mut columns = [99usize,101,102,103,104];
    let mut k = 0;
    for turns in game.epieces{
        let [x,y] = turns;
        rows[k] = x;
        columns[k] =y;
        k += 1;
    }
    k = 0;
    todo!();
    Situation::None
}

///////////////////////////////////////
fn board_2(game: Game) -> Game {
    let mut temp = game;
    temp.state.smove1 = match temp.state.omove1 {
        Perm::C | Perm::S => Perm::O,
        Perm::O => Perm::C,
        _ => Perm::Err,
    };
    let crds = match temp.state.smove1 {
        Perm::O => s_s(&temp.epieces),
        Perm::C => c_c(&temp.epieces),
        _ => [99, 99usize],
    };
    let [x, y] = crds;
    temp.board[x][y] = 8;
    temp
}

////////////////////////////
///USER MOVES
fn user_move(game: Game) -> Game {
    let mut temp = game;
    temp.cord = get_move();
    let [n, m] = temp.cord;
    temp.board[n][m] = 1i8;
    match temp.turn {
        1 => temp.state.omove1 = map(temp.cord),
        3 => temp.state.boardpermuation = map(temp.cord),
        _ => {}
    };
    temp.epieces[temp.turn] = temp.cord;
    temp
}
///////////////////////////////////////////////////
// USER INPUT
fn get_num() -> usize {
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed");

    let number: usize = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => get_num(),
    };
    number
}

fn get_move() -> [usize; 2] {
    let out: [usize; 2] = [get_num(), get_num()];
    out
}

/////////////////////////////////////////////////
// PRINT BOARD
fn print_board(game: &Game) {
    println!(
        "
              {}||{}||{}
            -----------
              {}||{}||{}
            -----------
              {}||{}||{}
    ",
        game.board[0][0],
        game.board[0][1],
        game.board[0][2],
        game.board[1][0],
        game.board[1][1],
        game.board[1][2],
        game.board[2][0],
        game.board[2][1],
        game.board[2][2]
    );
}
///////////////////////////////////////
//Game runs below

fn main() {
    println!("Welcome to Tic Tac Toe");
    let row: [i8; 3] = [0; 3];
    let mut game = Game {
        cord: [22; 2],
        board: [row; 3],
        turn: 1,
        epieces: [[22, 22], [22, 22], [22, 22], [22, 22], [22, 22]],
        state: State {
            smove1: Perm::Null,
            omove1: Perm::Null,
            boardpermuation: Perm::Null,
        },
    };
    loop {
        match &game.turn % 2 {
            1 => {
                println!("Your Turn:");
                print_board(&game);
            }
            0 => println!("My Turn"),
            _ => {}
        };
        game = match game.turn % 2 {
            0 => self_move(game),
            1 => user_move(game),
            _ => game,
        };
        game.turn = game.turn + 1;
    }
}
