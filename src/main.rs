use std::{io, ptr::null, usize, vec};

fn main() {

    let mut game:Game= Game::new();
    game.play();
    
}

struct Game{
    board: Vec<Vec<char>>,
    current_player: char,
}
impl Game{
    pub fn new() -> Self{
        Game { 
            board: vec![vec![' ';3];3],
            current_player: 'X', 
        }
    }

    pub fn play(&mut self) {
        loop {
            self.display_board(); // Display the board
            
            self.take_turn(); // Let the current player take a turn
    
            if self.check_win() { // Check if the current player has won               
                self.display_board(); // Display final state of the board
                break; // Exit the loop
            }
    
            self.switch_player(); // Switch to the next player
        }
    }
    pub fn display_board(&self){

        for row in self.board.iter(){
            println!("{:?}",row)
        }
    }
    pub fn switch_player(&mut self){
        self.current_player = match self.current_player{
            'X' => 'O',
            'O' => 'X',
            _ => 'P',
        }
    }
    pub fn take_turn(&mut self){
        println!("Its player's {} turn. Make a move (Row, Collumn)",self.current_player);
        let mut line = String::new();
        let cords: Vec<usize> = loop {
            println!("Enter your move as 'row col' (e.g., 1 2):");

            line.clear(); 
            match io::stdin().read_line(&mut line) {
                Ok(_) => {
                    let parsed: Vec<usize> = line
                        .trim()
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();

                    if parsed.len() == 2  && parsed[0] + parsed[1] <= 6{
                        break parsed;
                    } else {
                        println!("Invalid input format. Please enter two numbers.");
                    }
                }
                Err(_) => println!("Failed to read input. Please try again."),
            }
        };
        let (row, column) = (cords[0]-1, cords[1]-1);

        if self.board[row][column] == ' ' {
            self.board[row][column] = self.current_player
        }

    }
    
    pub fn check_win(&self) -> bool {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] == self.current_player
                && self.board[row][1] == self.current_player
                && self.board[row][2] == self.current_player
            {
                println!("Player {} has won!", self.current_player);
                return true;
            }
        }
    
        // Check columns
        for column in 0..3 {
            if self.board[0][column] == self.current_player
                && self.board[1][column] == self.current_player
                && self.board[2][column] == self.current_player
            {
                println!("Player {} has won!", self.current_player);
                return true;
            }
        }
    
        // Check first diagonal
        if self.board[0][0] == self.current_player
            && self.board[1][1] == self.current_player
            && self.board[2][2] == self.current_player
        {
            println!("Player {} has won!", self.current_player);
            return true;
        }
    
        // Check second diagonal
        if self.board[0][2] == self.current_player
            && self.board[1][1] == self.current_player
            && self.board[2][0] == self.current_player
        {
            println!("Player {} has won!", self.current_player);
            return true;
        }
    
        false // No win condition met
    }
    
}


