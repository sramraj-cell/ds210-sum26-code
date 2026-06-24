use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;
use std::time::{Instant, Duration};

// How many slots in the transposition table (memory cache of board positions we already evaluated)
const TABLE_SIZE: usize = 1 << 18;

// A very large number to represent a win or loss
const WIN_SCORE: i32 = 1_000_000;

// Only use 85% of the time limit so we don't accidentally go over
const TIME_USAGE: f64 = 0.85;



// TRANSPOSITION TABLE
// Stores results of board positions we already searched so we
// don't have to search the same position twice.


// Tells us what kind of result we stored for a position
#[derive(Clone, Copy)]
enum EntryType {
    Exact,      // the score is exactly right
    UpperBound, // the real score is at most this value
    LowerBound, // the real score is at least this value
}

// One entry in the transposition table
#[derive(Clone, Copy)]
struct TableEntry {
    key:       u64,        // which board position this is for
    depth:     u8,         // how deep we searched when we stored this
    score:     i32,        // the score we found
    kind:      EntryType,  // what kind of result this is
    best_move: (u8, u8),   // the best move we found for this position
}



// ZOBRIST HASHING
// A way to turn a board position into a unique number (hash)
// so we can quickly look it up in the transposition table.
// Each piece on each square gets a random number, and we XOR
// them all together to get the board's hash.

struct ZobristTable {
    x_hashes:    [[u64; 5]; 5], // random numbers for X pieces
    o_hashes:    [[u64; 5]; 5], // random numbers for O pieces
    wall_hashes: [[u64; 5]; 5], // random numbers for walls
}

impl ZobristTable {
    // Build the table using a deterministic random number generator
    // so the hashes are always the same every run
    fn new() -> Self {
        let mut seed = 0x123456789ABCDEF0u64;

        // Simple LCG random number generator
        let mut next_random = || {
            seed = seed.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
            seed
        };

        let mut x_hashes    = [[0u64; 5]; 5];
        let mut o_hashes    = [[0u64; 5]; 5];
        let mut wall_hashes = [[0u64; 5]; 5];

        for i in 0..5 {
            for j in 0..5 {
                x_hashes[i][j]    = next_random();
                o_hashes[i][j]    = next_random();
                wall_hashes[i][j] = next_random();
            }
        }

        Self { x_hashes, o_hashes, wall_hashes }
    }

    // Compute the hash for the current board by XORing together
    // the random numbers for each piece on each square
    fn hash_board(&self, board: &Board) -> u64 {
        let cells = board.get_cells();
        let size  = cells.len();
        let mut hash = 0u64;

        for i in 0..size {
            for j in 0..size {
                match cells[i][j] {
                    Cell::X    => hash ^= self.x_hashes[i][j],
                    Cell::O    => hash ^= self.o_hashes[i][j],
                    Cell::Wall => hash ^= self.wall_hashes[i][j],
                    Cell::Empty => {}
                }
            }
        }

        hash
    }
}



// SOLUTION AGENT

pub struct SolutionAgent {}

impl SolutionAgent {

    // Find all valid windows of 3 cells (rows, cols, diagonals)
    // that don't contain any walls, since walls block 3-in-a-rows.
    // We compute this once at the start of each turn and reuse it.
    fn find_all_windows(board: &Board) -> Vec<[(usize, usize); 3]> {
        let cells = board.get_cells();
        let size  = cells.len();
        let mut windows = Vec::with_capacity(48);

        for i in 0..size {
            for j in 0..size {

                // Horizontal: go right
                if j + 2 < size {
                    if cells[i][j] != Cell::Wall
                    && cells[i][j+1] != Cell::Wall
                    && cells[i][j+2] != Cell::Wall {
                        windows.push([(i, j), (i, j+1), (i, j+2)]);
                    }
                }

                // Vertical: go down
                if i + 2 < size {
                    if cells[i][j] != Cell::Wall
                    && cells[i+1][j] != Cell::Wall
                    && cells[i+2][j] != Cell::Wall {
                        windows.push([(i, j), (i+1, j), (i+2, j)]);
                    }
                }

                // Diagonal: go down-right
                if i + 2 < size && j + 2 < size {
                    if cells[i][j] != Cell::Wall
                    && cells[i+1][j+1] != Cell::Wall
                    && cells[i+2][j+2] != Cell::Wall {
                        windows.push([(i, j), (i+1, j+1), (i+2, j+2)]);
                    }
                }

                // Diagonal: go down-left
                if i + 2 < size && j >= 2 {
                    if cells[i][j] != Cell::Wall
                    && cells[i+1][j-1] != Cell::Wall
                    && cells[i+2][j-2] != Cell::Wall {
                        windows.push([(i, j), (i+1, j-1), (i+2, j-2)]);
                    }
                }
            }
        }

        windows
    }

    // Estimate how good the board is without searching further.
    // Positive = good for X, Negative = good for O.
    fn heuristic(board: &Board, windows: &Vec<[(usize, usize); 3]>) -> i32 {
        let cells  = board.get_cells();
        let mut score = 0;

        // Score every window of 3 cells
        for window in windows {
            let mut number_of_x = 0;
            let mut number_of_o = 0;

            for &(row, col) in window {
                match cells[row][col] {
                    Cell::X => number_of_x += 1,
                    Cell::O => number_of_o += 1,
                    _       => {}
                }
            }

            // If both players are in this window, neither can win here — skip it
            if number_of_x > 0 && number_of_o > 0 {
                continue;
            }

            // Score the window based on how many pieces one player has in it
            if number_of_x > 0 {
                score += match number_of_x {
                    3 => 1000, // completed 3-in-a-row
                    2 => 50,   // 2 in a row with space
                    1 => 5,    // foothold
                    _ => 0,
                };
            } else if number_of_o > 0 {
                score -= match number_of_o {
                    3 => 1000,
                    2 => 50,
                    1 => 5,
                    _ => 0,
                };
            }
        }

        // Small bonus for controlling cells near the center of the board
        let center = cells.len() as i32 / 2;
        for i in 0..cells.len() {
            for j in 0..cells.len() {
                let distance_from_center =
                    (i as i32 - center).abs() + (j as i32 - center).abs();
                let center_bonus = cells.len() as i32 - distance_from_center;

                match cells[i][j] {
                    Cell::X => score += center_bonus,
                    Cell::O => score -= center_bonus,
                    _       => {}
                }
            }
        }

        score
    }

    // Alpha-beta pruning — an improved version of minimax that skips
    // branches that can't possibly affect the final result.
    //
    // alpha = the best score X has guaranteed so far
    // beta  = the best score O has guaranteed so far
    // If beta <= alpha, we can stop searching this branch (pruning).
    fn alphabeta(
        board:        &mut Board,
        player:       Player,
        depth:        u8,
        mut alpha:    i32,
        mut beta:     i32,
        windows:      &Vec<[(usize, usize); 3]>,
        zobrist:      &ZobristTable,
        hash:         u64,
        table:        &mut Vec<Option<TableEntry>>,
        start_time:   Instant,
        time_limit:   Duration,
        nodes_visited: &mut u64,
        out_of_time:  &mut bool,
    ) -> i32 {

        // Every 1024 nodes, check if we've run out of time
        *nodes_visited += 1;
        if *nodes_visited & 1023 == 0 && start_time.elapsed() >= time_limit {
            *out_of_time = true;
            return 0;
        }

        // Game is over — return the real score weighted by WIN_SCORE
        if board.game_over() {
            return board.score() * WIN_SCORE;
        }

        // Hit the depth limit — use the heuristic to estimate
        if depth == 0 {
            return Self::heuristic(board, windows);
        }

        // Check the transposition table — have we seen this position before?
        let table_index = (hash as usize) % table.len();
        let saved_best_move = if let Some(entry) = table[table_index] {
            if entry.key == hash && entry.depth >= depth {
                match entry.kind {
                    EntryType::Exact      => return entry.score,
                    EntryType::UpperBound if entry.score <= alpha => return entry.score,
                    EntryType::LowerBound if entry.score >= beta  => return entry.score,
                    _ => {}
                }
            }
            Some(entry.best_move)
        } else {
            None
        };

        let mut available_moves = board.moves();
        if available_moves.is_empty() {
            return board.score() * WIN_SCORE;
        }

        // Move ordering: try the best move from the table first,
        // then try moves closer to the center before edge moves.
        // This helps alpha-beta pruning cut off more branches early.
        let center = board.get_cells().len() as i32 / 2;
        available_moves.sort_by_key(|&(row, col)| {
            if let Some((best_row, best_col)) = saved_best_move {
                if row == best_row as usize && col == best_col as usize {
                    return -100; // put the table's best move first
                }
            }
            (row as i32 - center).abs() + (col as i32 - center).abs()
        });

        let mut best_score = if let Player::X = player { i32::MIN } else { i32::MAX };
        let mut best_move  = (available_moves[0].0 as u8, available_moves[0].1 as u8);

        match player {
            Player::X => {
                // X is maximizing
                for m in available_moves {
                    let new_hash = hash ^ zobrist.x_hashes[m.0][m.1];

                    board.apply_move(m, player);
                    let score = Self::alphabeta(
                        board, player.flip(), depth - 1, alpha, beta,
                        windows, zobrist, new_hash, table,
                        start_time, time_limit, nodes_visited, out_of_time,
                    );
                    board.undo_move(m, player);

                    if *out_of_time { return 0; }

                    if score > best_score {
                        best_score = score;
                        best_move  = (m.0 as u8, m.1 as u8);
                    }

                    alpha = alpha.max(best_score);
                    if beta <= alpha { break; } // prune — O would never allow this
                }
            }
            Player::O => {
                // O is minimizing
                for m in available_moves {
                    let new_hash = hash ^ zobrist.o_hashes[m.0][m.1];

                    board.apply_move(m, player);
                    let score = Self::alphabeta(
                        board, player.flip(), depth - 1, alpha, beta,
                        windows, zobrist, new_hash, table,
                        start_time, time_limit, nodes_visited, out_of_time,
                    );
                    board.undo_move(m, player);

                    if *out_of_time { return 0; }

                    if score < best_score {
                        best_score = score;
                        best_move  = (m.0 as u8, m.1 as u8);
                    }

                    beta = beta.min(best_score);
                    if beta <= alpha { break; } // prune — X would never allow this
                }
            }
        }

        // Save the result in the transposition table for future lookups
        let entry_kind = if best_score <= alpha {
            EntryType::UpperBound
        } else if best_score >= beta {
            EntryType::LowerBound
        } else {
            EntryType::Exact
        };

        table[table_index] = Some(TableEntry {
            key:       hash,
            depth,
            score:     best_score,
            kind:      entry_kind,
            best_move: best_move,
        });

        best_score
    }
}


impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {

        // Set up the timer — stop searching when 85% of time is used
        let start_time   = Instant::now();
        let max_duration = Duration::from_millis((time_limit as f64 * TIME_USAGE) as u64);

        // Precompute all valid windows and the zobrist hash for this position
        let windows = Self::find_all_windows(board);
        let zobrist = ZobristTable::new();
        let hash    = zobrist.hash_board(board);

        // Transposition table — starts empty
        let mut table: Vec<Option<TableEntry>> = vec![None; TABLE_SIZE];

        // Track the best move found so far across all depths
        let mut best_move_found = (0, 0);
        let mut best_score_found = 0;

        // Iterative deepening — search depth 1, then 2, then 3, etc.
        // This way if we run out of time, we always have a result from
        // the last fully completed depth to fall back on.
        let max_depth = board.moves().len() as u8;

        for current_depth in 1..=max_depth {

            let mut out_of_time  = false;
            let mut nodes_visited = 0u64;

            let mut available_moves = board.moves();
            if available_moves.is_empty() { break; }

            // Order moves for the root: table's best move first, then center moves
            let table_index    = (hash as usize) % table.len();
            let saved_best_move = if let Some(entry) = table[table_index] {
                if entry.key == hash { Some(entry.best_move) } else { None }
            } else {
                None
            };

            let center = board.get_cells().len() as i32 / 2;
            available_moves.sort_by_key(|&(row, col)| {
                if let Some((best_row, best_col)) = saved_best_move {
                    if row == best_row as usize && col == best_col as usize {
                        return -100;
                    }
                }
                (row as i32 - center).abs() + (col as i32 - center).abs()
            });

            let mut best_score_this_depth = if let Player::X = player { i32::MIN } else { i32::MAX };
            let mut best_move_this_depth  = available_moves[0];
            let mut alpha = i32::MIN;
            let mut beta  = i32::MAX;

            for m in available_moves {
                let new_hash = if let Player::X = player {
                    hash ^ zobrist.x_hashes[m.0][m.1]
                } else {
                    hash ^ zobrist.o_hashes[m.0][m.1]
                };

                board.apply_move(m, player);
                let score = Self::alphabeta(
                    board, player.flip(), current_depth - 1, alpha, beta,
                    &windows, &zobrist, new_hash, &mut table,
                    start_time, max_duration, &mut nodes_visited, &mut out_of_time,
                );
                board.undo_move(m, player);

                if out_of_time { break; }

                match player {
                    Player::X => {
                        if score > best_score_this_depth {
                            best_score_this_depth = score;
                            best_move_this_depth  = m;
                        }
                        alpha = alpha.max(best_score_this_depth);
                    }
                    Player::O => {
                        if score < best_score_this_depth {
                            best_score_this_depth = score;
                            best_move_this_depth  = m;
                        }
                        beta = beta.min(best_score_this_depth);
                    }
                }
            }

            // Only update our best answer if this depth completed without running out of time
            if !out_of_time {
                best_move_found  = best_move_this_depth;
                best_score_found = best_score_this_depth;

                // If we found a forced win or loss, no need to search deeper
                if best_score_found.abs() >= WIN_SCORE { break; }
            } else {
                break;
            }
        }

        // Safety net: if we somehow have no move yet, just pick the first available one
        if best_move_found == (0, 0) {
            if let Some(first_move) = board.moves().first() {
                best_move_found = *first_move;
            }
        }

        (best_score_found, best_move_found.0, best_move_found.1)
    }
}