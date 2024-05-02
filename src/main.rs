use std::fs::File;
use std::io::{self, Read};

fn main() {
    //The indices within field where 'X' or 'O' can be placed
    let mut indices: Vec<usize> = Vec::with_capacity(9);
    let field_vec = create_field("field.txt", &mut indices)
        .unwrap_or_else(|error|{ 
            panic!("Failed to open field file:\n{}", error);
        });
   
    
    let player1: Vec<i32> = Vec::with_capacity(5);
    let player1_turn = true;
    let player2: Vec<i32> = Vec::with_capacity(5);
    
    loop {
        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Readline operation failed");
        let location = PlayerMove::new(player_input.trim());
        let location = match location {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        println!("{}", location.square_num);
    }
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
    Ok(field_vec)
}

#[derive(Debug)]
struct PlayerMove {
    coord_letter: char,
    coord_digit: i32,
    square_num: i32
}

impl PlayerMove {
    // Creates a new instance of PlayerMove from a coordinates input (e.g. a2, c3, B1 etc...)
    // Calls compute_square_num() before returning the finished instance of PlayerMove.
    fn new(coord: &str) -> Result<PlayerMove, String> {
        let mut point = Self { coord_letter: ' ', coord_digit: 0, square_num: 0 };
        let mut digit: i32;
        for c in coord.chars() {
            if c != ' ' {
                if c.is_digit(10) && point.coord_digit == 0 {
                    digit = String::from(c).parse().unwrap();         
                    if digit > 3 {
                        return Err(format!("Invalid input: {} is out of bounds.\nShould be between 1 and 3.", digit));
                    }
                    point.coord_digit = digit;
                }
                else {
                    if point.coord_letter == ' ' { 
                        point.coord_letter = match c {
                            'a' => c,
                            'b' => c,
                            'c' => c,
                            _ => return Err(format!("Invalid input: {}", c))
                        }
                    }
                }
            }
            if point.coord_digit != 0 && point.coord_letter != ' ' {
                break;
            }
        }
        point.square_num = point.compute_square_num()?;
        Ok(point)
    }
    // Computes the number of the field square that this PlayerMove instance was placed in
    // Example: Field at coordinates a1 is number 1. a3 is number 3. b2 is number 5 etc...  
    fn compute_square_num(&mut self) -> Result<i32, String> {
        let num = match self.coord_letter {         
            'a' => self.coord_digit,
            'b' => self.coord_digit + 3,
            'c' => self.coord_digit + 6,
            _ => panic!("error")
        };
        Ok(num)
    }
}
