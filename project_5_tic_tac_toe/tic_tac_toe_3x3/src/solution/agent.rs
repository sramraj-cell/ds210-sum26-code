use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.111111

impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)
        // Base case where the game is over and no moves can be played
        if board.game_over() {
            return (board.score(), 0, 0);
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
                    let (score, _, _) = SolutionAgent::solve(board, player.flip(), _time_limit); // recursively flips the players turns and plays their optimal moves until a result is found.
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
                    let (score, _, _)  = SolutionAgent::solve(board, player.flip(), _time_limit); //recursively flips the players turns and plays their most optimal moves until a result is found. 
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