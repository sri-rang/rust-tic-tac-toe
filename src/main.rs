use text_io::read;

const BLANK_CELL: &str = " ";
const PLAYER_01: &str = "X";
const PLAYER_02: &str = "Y";

const GAME_ONGOING: &str = "GAME_ONGOING";
const GAME_DRAW: &str = "GAME_DRAW";

const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
    // rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    // cols
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    // diags
    [0, 4, 8],
    [2, 4, 6]
];

fn main() {
    let mut board: [&str; 9] = [
        BLANK_CELL, BLANK_CELL, BLANK_CELL,
        BLANK_CELL, BLANK_CELL, BLANK_CELL,
        BLANK_CELL, BLANK_CELL, BLANK_CELL,
    ];

    let mut current_player = PLAYER_01;
    let mut result = get_result(board);

    while result == GAME_ONGOING {
        render_board(board);
        if current_player == PLAYER_01 {
            board[get_human_move(PLAYER_01, board)] = PLAYER_01;
            current_player = PLAYER_02;
        } else if current_player == PLAYER_02 {
            board[get_human_move(PLAYER_02, board)] = PLAYER_02;
            current_player = PLAYER_01;
        }
        result = get_result(board);
    }

    println!("   ");
    render_board(board);
    println!("   ");
    println!(" Result: {0}", result);
    println!("   ");
}

fn get_result(board: [&str; 9]) -> &str {
    for combination in &WINNING_COMBINATIONS {
        let value_1 = board[combination[0]];
        let value_2 = board[combination[1]];
        let value_3 = board[combination[2]];
        if value_1 == value_2 && value_1 == value_3 && value_1 != BLANK_CELL {
            return value_1;
        }
    }
    if board.contains(&BLANK_CELL) {
        return GAME_ONGOING;
    }
    return GAME_DRAW;
}

fn render_board(board: [&str; 9]) {
    clear_screen();
    println!(" ");
    println!("     |     |     ");
    println!("  {0}  |  {1}  |  {2}  ", board[0], board[1], board[2]);
    println!("     |     |     ");
    println!("-----------------");
    println!("     |     |     ");
    println!("  {0}  |  {1}  |  {2} ", board[3], board[4], board[5]);
    println!("     |     |     ");
    println!("-----------------");
    println!("     |     |     ");
    println!("  {0}  |  {1}  |  {2} ", board[6], board[7], board[8]);
    println!("     |     |     ");
    println!(" ");
}

fn get_human_move(player: &str, board: [&str; 9]) -> usize {
    println!("{0}'s Turn:", player);
    let mut human_move = read_stdin();
    while !is_valid_move(human_move, board) {
        human_move = read_stdin();
        println!("{0}", human_move);
    }
    return human_move;
}

fn read_stdin() -> usize {
    let human_move: usize = read!();
    return human_move;
}

fn is_valid_move(position: usize, board: [&str; 9]) -> bool {
    if position > 8 {
        println!("Enter value between 0 and 8");
        return false;
    } else if board[position] != BLANK_CELL {
        println!("Position not EMPTY");
        return false;
    }
    return true;
}

fn clear_screen() {
    std::process::Command::new("clear").status().unwrap().success();
}