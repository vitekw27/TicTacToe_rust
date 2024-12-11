fn main() {

    let mut game:Game= Game::new();
    game.display_board();
    
}

struct Game{
    board: Vec<Vec<char>>,
    current_player: char,
}
impl Game{
    pub fn new() -> Self{
        Game { 
            board: vec![vec!['T';3];3],
            current_player: 'X', 
        }
    }

    pub fn play(&mut self){
        loop{
            self.display_board();
            self.switch_player();



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
        
    }
}


