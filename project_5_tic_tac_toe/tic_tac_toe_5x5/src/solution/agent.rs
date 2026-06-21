//use tic_tac_toe_stencil::Outcome::O;
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

// Your solution solution.

const MAX_DEPTH: u64 = 4; 
// looks at a group of exactly 3 cells and scores based on how promising it is.
// positive = good for X, Negative = good for O
fn evaluate_window(window: &[&Cell; 3]) -> i32{
    // count how many of each type are in the 3 cells
    let x_count     = window.iter().filter(|&&c| c == &Cell::X).count();
    let o_count     = window.iter().filter(|&&c| c == &Cell::O).count();
    let empty_count = window.iter().filter(|&&c| c == &Cell::Empty).count();

    //if both players have a piece in the 3 cells its worthless
    if x_count > 0 && o_count > 0 {
        return 0;
    }
    // X has 2 in a row with 1 empty space, 1 move away from completing 3 in a row, 10 points
    if x_count == 2 && o_count == 1 {
        return 10;
    }
    // X has 1 piece and 2 empty spaces, small potential, 2 points
    if x_count == 1 && empty_count == 2 {
        return 2;
    }
    // O has 2 in a row with 1 empty space, big threat to X, -10 points
    if o_count == 2 && empty_count ==1 {
        return -10;
    }
    // O has 1 piece with 2 empty spaces, small threat to X , - 2 points
    if o_count == 1 && empty_count == 2 {
        return - 2;
    }
    0 // window is completely empty or all walls
}
//smart heuristic function
fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells(); //get the 2d grid of cells
    let size = cells.len(); // how wide the board is, 3 for 3*3 , 5 for 5*5
    let mut score: i32 = 0;  // running total of the score

    score = score + board.score() * 100; // reward already completed 3 in a rows heavily, so they always outweigh smaller bonus scores.

    // scan every posibility for a 3 in a row/ column/ diagonal starting from that cell
    for i in 0..size {
        for j in 0..size {

            // horizontal window. 
            if j + 2 < size { // j+2 must be inside the board 
                let window = [&cells[i][j], &cells[i][j+1], &cells[i][j+2]];
                score = score + evaluate_window(&window);
            }
            // vertical window
            if i + 2 < size {
                let window = [&cells[i][j], &cells[i+1][j], &cells[i+2][j]];
                score = score + evaluate_window(&window);
            }
            // Diagonal window going down right
            if i + 2 < size && j +2 < size {
                let window = [&cells[i][j], &cells[i+1][j+1], &cells[i+2][j+2]];
                score = score + evaluate_window(&window);
            }
            // Diagonal window going down left
            if i + 2 < size && j >= 2 { //j >= 2 so that j-2 does not go negative
                let window = [&cells[i][j], &cells[i+1][j-1], &cells[i+2][j-2]];
                score = score + evaluate_window(&window);

            }
        }
    }


// reward control of the center
    let center = size / 2; // middle index of the board, 1 for 3*3 and 2 for 5*5
    for i in 0..size {
        for j in 0..size {
            // calculate the distance from the center
            let distance_from_center = (i as i32 - center as i32).abs() + (j as i32 -center as i32).abs();
            let center_bonus = (size as i32) - distance_from_center; // the closer to the center the higher the bonus and vice versa

            match &cells[i][j] {
                Cell::X => score = score + center_bonus,
                Cell::O => score = score - center_bonus,
                _=> {} //ignore if empty or wall.
        }
    }
   
}
 score // returns the final estimated score for the board position
}




pub struct SolutionAgent {}
// Put your solution here.111111

impl SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn minimax(board: &mut Board, player: Player, depth: u64, max_depth: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)
        // Base case where the game is over and no moves can be played
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        //if depth limit has been reached
        if depth >= MAX_DEPTH {
            return (heuristic(board), 0, 0);
        }
        
        //If the list of available moves is empty  and you cannot make any moves so the terminal board score is returned. checking for draws.
        let moves = board.moves();
        if moves.is_empty() {
            return (board.score(), 0, 0);
        }
        
        let mut best_move = moves[0];

        match player {
            Player::X => {
                //Maximizing player X
                let mut best_score = i32::MIN; //best_score = -238259834.....  the lowest possible integer in rust
                for &m in &moves { // iterates through all the possible moves and applies the moves as a reference 
                    board.apply_move(m, player);
                    let (score, _, _) = SolutionAgent::minimax(board, player.flip(), depth +1, max_depth); // recursively flips the players turns and plays their optimal moves until a result is found.
                    board.undo_move(m, player); // backtracking, when each recursion is finished the board returns to its orginial state so that recursion can take place on the board with the same original state.

                    if score > best_score {     // 1st iteration: if score > -234543645.....
                        best_score = score;     // best_score = score (The best_score becomes the score returned by the recursive call)
                        best_move = m;          // the best move also leads to a better outcome for X, we then save the coordinates of this move
                    }
                }
                (best_score, best_move.0, best_move.1) // after evaluating all possible moves, the highest possible score along with the coordinates of the moves that achieves it is returned.
            }
            Player::O => {
                //Minimizing player 0
                let mut best_score = i32::MAX; //the highest possible number (234567836.....) in rust
                for &m in &moves {
                    board.apply_move(m, player);
                    let (score, _, _)  = SolutionAgent::minimax(board, player.flip(), depth+1, max_depth); //recursively flips the players turns and plays their most optimal moves until a result is found. 
                    board.undo_move(m, player); //undos the moves afte1r recursion to return the board to its orginal state to do recursion again.


                    // finds the best scores and their moves and saves it for player O, looks for the minimum score because player O needs negative scores
                    if score < best_score {
                        best_score = score;
                        best_move = m;    
                        
                    }
                }
                (best_score, best_move.0, best_move.1) //.0 and .1 because it returns a tuple .0 and .1 are the x and y coordinates 
            }
        }
    }
}

// 
impl Agent for SolutionAgent {
    fn solve( board: &mut Board, player: Player, _time_limit: u64, ) -> (i32, usize, usize) {
        let emptycells = board.moves().len(); // count how many empty squares available to play in, i.e 3*3 or 5*5

        let max_depth;  // initialize a variable max_depth 
        if emptycells <=9 {
            max_depth = u64::MAX; // if it is 3*3 or less, recurse as deep as possible
        } else if emptycells >= 20 {
            max_depth = 2;  // early game on an open 5*5 - too many moves to go deep, it will take too long
        } else {
            max_depth = 4; // if it is larger than 3*3 think recurse and think 4 moves ahead
        }
        SolutionAgent::minimax(board, player, 0, max_depth) // run the minimax algorithm starting at depth 0, and stop at max depth
    }
}
