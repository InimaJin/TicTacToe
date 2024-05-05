use std::fs::File;
use std::io::{self, Read};

fn main() {
    //The indices within field where 'X' or 'O' can be placed
    let mut indices: Vec<usize> = Vec::with_capacity(9);
    // The vector holding the game's field. Instead of the 'r' placeholder for the squares where players can insert, it has whitespaces.
    let mut field_vec = create_field("field.txt", &mut indices).unwrap_or_else(|error| {
        panic!("Couldn't open field file:\n{}", error);
    });
    
    // These two vectors hold true at the indices where players inserted their symbol, false for
    // the rest
    let mut player1: Vec<bool> = vec![false; 9];
    let mut player2: Vec<bool> = vec![false; 9];
    let mut player_turn = 1; // player 1 / 2
    let mut winner = 0;

    let mut msg = String::new();
    while winner == 0 {
        draw_game(&field_vec, &mut msg, true, player_turn);

        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Readline operation failed");
        if player_input.len() < 2 {
            println!("Please specify two coordinates. E.g. B2");
            continue;
        }

        let player_move = PlayerMove::new(player_input.trim());
        let player_move = match player_move {
            Ok(instance) => instance,
            Err(e) => {
                msg = e;
                continue;
            }
        };
        if player1[player_move.square_val] || player2[player_move.square_val] {
            msg = String::from("This field is already occupied. Choose another one.");
            continue;
        }
        if player_turn == 1 {
            player1[player_move.square_val] = true;
            field_vec[*indices.get(player_move.square_val).unwrap()] = 'X';
            if tic_tac_toe(&player1) {
                winner = 1;
            }
            player_turn = 2;
        } else {
            player2[player_move.square_val] = true;
            field_vec[*indices.get(player_move.square_val).unwrap()] = 'O';
            if tic_tac_toe(&player2) {
                winner = 2;
            }
            player_turn = 1;
        }

    }
    draw_game(&field_vec, &mut msg, false, 0);
    println!("\u{001b}[33mCongratulations, player {}. You win!\u{001b}[0m", winner);
}

// Reads a tic-tac-toe field template from a file and feeds each of its characters into the field_vec
// while replacing 'r' (a placeholder in the template for where players can place their symbol)
// with a whitespace and pushing the index of that whitespace into the indices vector
fn create_field(filename: &str, indices: &mut Vec<usize>) -> Result<Vec<char>, io::Error> {
    let mut file = File::open(filename)?;

    let mut field_string = String::new();
    file.read_to_string(&mut field_string);

    let mut field_vec: Vec<char> = Vec::with_capacity(field_string.len());
    for (i, c) in field_string.chars().enumerate() {
        if c == 'r' {
            indices.push(i);
            field_vec.push(' ');
            continue;
        }
        field_vec.push(c);
    }
    if indices.len() != 9 {
        panic!("Invalid tic-tac-toe field. Should have excatly 9 insert positions specified");
    }
    Ok(field_vec)
}

// Draws the field in its current state after clearing the screen, prints the prompt to enter
// coordinates and prints the message possibly containing an error
fn draw_game(field_vec: &Vec<char>, msg: &mut String, prompt_user: bool, current_player: i32) {
    let field_as_string: String = field_vec.iter().collect();
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", field_as_string);
    if prompt_user { println!("\u{001b}[35m{}\u{001b}[0m\nEnter coordinate (Player {}):", msg, current_player) }
    msg.clear();
}

// Represents a move the player has made
struct PlayerMove {
    coord_letter: char,
    coord_digit: i32,
    square_val: usize,
}

impl PlayerMove {
    // Creates a new instance of PlayerMove from a coordinates input (e.g. a2, c3, B1 etc...)
    // Calls compute_square_num() before returning the finished instance of PlayerMove.
    fn new(coord: &str) -> Result<Self, String> {
        let mut point = Self {
            coord_letter: ' ',
            coord_digit: 0,
            square_val: 0,
        };
        let mut digit: i32;
        for c in coord.chars() {
            if c != ' ' {
                if c.is_digit(10) && point.coord_digit == 0 {
                    digit = String::from(c).parse().unwrap();
                    if digit > 3 {
                        return Err(format!(
                            "Invalid column: {} is out of bounds.\nShould be between 1 and 3.",
                            digit
                        ));
                    }
                    point.coord_digit = digit;
                } else if c.is_alphabetic() {
                    if point.coord_letter == ' ' {
                        point.coord_letter = match c {
                            'a' => c,
                            'b' => c,
                            'c' => c,
                            _ => return Err(format!("Invalid row: '{}' - Should be a, b or c.", c)),
                        }
                    }
                }
            }
            if point.coord_digit != 0 && point.coord_letter != ' ' {
                break;
            }
        }
        point.compute_square_val()?;
        Ok(point)
    }
    // Computes the number of the field square that this PlayerMove instance was placed in
    // Example: Field at coordinates a1 is number 0. a3 is number 2. b2 is number 4 etc...
    // Sets the square_val value to that number
    fn compute_square_val(&mut self) -> Result<usize, String>  {
        let val = match self.coord_letter {
            'a' => self.coord_digit - 1,
            'b' => self.coord_digit + 2,
            'c' => self.coord_digit + 5,
            _ => return Err("Invalid letter.".to_string()),
        };
        self.square_val = val as usize;
        Ok(self.square_val)
    }
}

// Checks if the current player's vector has three in a row by
// examining all combinations for three 'true' values and returning true
// if one matches
fn tic_tac_toe(player_vec: &Vec<bool>) -> bool {
    return &player_vec[0..3] == &[true, true, true]
        || &player_vec[3..6] == &[true, true, true]
        || &player_vec[6..9] == &[true, true, true]
        || player_vec[0] && player_vec[3] && player_vec[6]
        || player_vec[1] && player_vec[4] && player_vec[7]
        || player_vec[2] && player_vec[5] && player_vec[8]
        || player_vec[0] && player_vec[4] && player_vec[8]
        || player_vec[2] && player_vec[4] && player_vec[6];
    
}
