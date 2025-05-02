use crate::board::{Board, SIZE};
use crate::player::GomokuPlayer;
use std::cmp::{max, min};

pub struct AI {
    pub icon: char,
    max_depth: i32, // ミニマックスの最大探索深度
    transposition_table: std::collections::HashMap<u64, (i32, i32)>, // 状態のハッシュ値 -> (評価値, 探索深度)
}

impl AI {
    pub fn new(icon: char) -> Self {
        AI {
            icon,
            max_depth: 2, // デフォルト深度は3
            transposition_table: std::collections::HashMap::new(),
        }
    }

    pub fn _with_depth(icon: char, depth: i32) -> Self {
        AI {
            icon,
            max_depth: depth,
            transposition_table: std::collections::HashMap::new(),
        }
    }

    // 未実装
    pub fn _adjust_evaluation_params(&self) {
        println!("AIの評価関数パラメータを調整しました");
    }

    // 探索半径
    const SEARCH_RADIUS: isize = 2;

    fn find_valid_moves_around(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        let mut visited = vec![vec![false; SIZE]; SIZE];

        for i in 0..SIZE {
            for j in 0..SIZE {
                if board.get_board()[i][j] != '#' {
                    for di in -Self::SEARCH_RADIUS..=Self::SEARCH_RADIUS {
                        for dj in -Self::SEARCH_RADIUS..=Self::SEARCH_RADIUS {
                            let ni = i as isize + di;
                            let nj = j as isize + dj;

                            if ni < 0 || ni >= SIZE as isize || nj < 0 || nj >= SIZE as isize {
                                continue;
                            }

                            let ni = ni as usize;
                            let nj = nj as usize;

                            if visited[ni][nj] {
                                continue;
                            }

                            if board.settable(ni, nj) {
                                valid_moves.push((ni, nj));
                                visited[ni][nj] = true;
                            }
                        }
                    }
                }
            }
        }

        // もし有効な手がない場合（例：初手）は、盤面の中央付近から効率的に探す
        if valid_moves.is_empty() {
            let center = SIZE / 2;
            let search_range = SIZE / 3;

            let start = center.saturating_sub(search_range);
            let end = (center + search_range).min(SIZE);

            for i in start..end {
                for j in start..end {
                    if board.settable(i, j) {
                        valid_moves.push((i, j));
                    }
                }
            }

            // それでも手が見つからない場合は全体から探す
            if valid_moves.is_empty() {
                for i in 0..SIZE {
                    for j in 0..SIZE {
                        if board.settable(i, j) {
                            valid_moves.push((i, j));
                        }
                    }
                }
            }
        }

        valid_moves
    }

    fn compute_board_hash(&self, board: &Board) -> u64 {
        let mut hash: u64 = 0;
        let zobrist_table = self.get_zobrist_table();

        (0..SIZE).for_each(|i| {
            for j in 0..SIZE {
                let cell = board.get_cell(i, j);
                if cell != '#' {
                    // 空白以外のセル（石）がある場合、そのハッシュ値を計算
                    let cell_index = if cell == 'O' { 0 } else { 1 };
                    hash ^= zobrist_table[i][j][cell_index];
                }
            }
        });

        hash
    }

    fn get_zobrist_table(&self) -> &[[[u64; 2]; SIZE]; SIZE] {
        use std::sync::{Once, OnceLock};

        static ZOBRIST_TABLE: OnceLock<[[[u64; 2]; SIZE]; SIZE]> = OnceLock::new();
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            // テーブル初期化
            let mut table = [[[0; 2]; SIZE]; SIZE];
            let mut seed: u64 = 0x123456789ABCDEF0;

            (0..SIZE).for_each(|i| {
                for j in 0..SIZE {
                    for k in 0..2 {
                        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        table[i][j][k] = seed;
                    }
                }
            });

            // OnceLockにテーブルを設定
            let _ = ZOBRIST_TABLE.set(table);
        });

        ZOBRIST_TABLE.get().unwrap()
    }
}

// 自分のパターン（O側）
const FIVE: [&str; 1] = ["OOOOO"]; // 5連（勝利）
const LIVE4: [&str; 4] = ["#OOOO#", "#OO#OO#", "#O#OOO#", "#OOO#O#"]; // 活4
const DEAD4: [&str; 6] = [
    "#OOOOX", "XOOOO#", "XOO#OO#", "#OO#OOX", "XO#OOO#", "#O#OOOX",
]; // 眠4（相手が防がないと勝てる）
const LIVE3: [&str; 6] = ["##OOO##", "#OO#O#", "#O#OO#", "#O##OO#", "##OOO#", "#OOO##"];
const DEAD3: [&str; 4] = ["#OOOX", "XOOO#", "XOO#O#", "#OO#OX"]; // 眠3
const LIVE2: [&str; 3] = ["##OO##", "##O#O##", "##O##O##"]; // 活2

// 敵のパターン（X側）
const FIVE_E: [&str; 1] = ["XXXXX"]; // 敵の5連（敵の勝利）
const LIVE4_E: [&str; 4] = ["#XXXX#", "#XX#XX#", "#X#XXX#", "#XXX#X#"]; // 敵の活4
const DEAD4_E: [&str; 6] = [
    "#XXXXO", "OXXXX#", "OXX#XX#", "#XX#XXO", "OX#XXX#", "#X#XXXO",
]; // 敵の眠4
const LIVE3_E: [&str; 6] = ["##XXX##", "#XX#X#", "#X#XX#", "#X##XX#", "##XXX#", "#XXX##"]; // 敵の活3
const DEAD3_E: [&str; 4] = ["#XXXO", "OXXX#", "OXX#X#", "#XX#XO"]; // 敵の眠3
const LIVE2_E: [&str; 3] = ["##XX##", "##X#X##", "##X##X##"]; // 敵の活2

impl AI {
    fn evaluate(&self, board: &Board) -> i32 {
        let mut score = 0;
        let board_lines = extract_lines(board.get_board());

        // 各パターンの得点設定
        const FIVE_SCORE: i32 = 1000000; // 5連（勝利）
        const LIVE4_SCORE: i32 = 50000; // 活4
        const DEAD4_SCORE: i32 = 10000; // 眠4
        const LIVE3_SCORE: i32 = 5000; // 活3
        const DEAD3_SCORE: i32 = 1000; // 眠3
        const LIVE2_SCORE: i32 = 100; // 活2

        // 相手の活3の数をカウント
        let mut enemy_live3_count = 0;

        for line in &board_lines {
            for pattern in &LIVE3_E {
                if line.contains(pattern) {
                    enemy_live3_count += 1;
                }
            }
        }

        if enemy_live3_count > 0 {
            score -= LIVE3_SCORE * enemy_live3_count * 3;
        }

        // 通常の評価
        for line in board_lines {
            for pattern in &FIVE {
                if line.contains(pattern) {
                    score += FIVE_SCORE;
                }
            }
            for pattern in &LIVE4 {
                if line.contains(pattern) {
                    score += LIVE4_SCORE;
                }
            }
            for pattern in &DEAD4 {
                if line.contains(pattern) {
                    score += DEAD4_SCORE;
                }
            }
            for pattern in &LIVE3 {
                if line.contains(pattern) {
                    score += LIVE3_SCORE;
                }
            }
            for pattern in &DEAD3 {
                if line.contains(pattern) {
                    score += DEAD3_SCORE;
                }
            }
            for pattern in &LIVE2 {
                if line.contains(pattern) {
                    score += LIVE2_SCORE;
                }
            }

            for pattern in &FIVE_E {
                if line.contains(pattern) {
                    score -= FIVE_SCORE * 2;
                }
            }
            for pattern in &LIVE4_E {
                if line.contains(pattern) {
                    score -= LIVE4_SCORE * 2;
                }
            }
            for pattern in &DEAD4_E {
                if line.contains(pattern) {
                    score -= DEAD4_SCORE;
                }
            }

            for pattern in &DEAD3_E {
                if line.contains(pattern) {
                    score -= DEAD3_SCORE;
                }
            }
            for pattern in &LIVE2_E {
                if line.contains(pattern) {
                    score -= LIVE2_SCORE;
                }
            }
        }

        score
    }

    fn alpha_beta(
        &mut self,
        board: &Board,
        depth: i32,
        alpha: i32,
        beta: i32,
        is_maximizing: bool,
    ) -> i32 {
        if depth == 0 {
            return self.evaluate(board);
        }

        let board_hash = self.compute_board_hash(board);

        // キャッシュがあればそれを使用
        if let Some(&(cached_score, cached_depth)) = self.transposition_table.get(&board_hash) {
            if cached_depth >= depth {
                return cached_score;
            }
        }

        let valid_moves = self.find_valid_moves_around(board);

        if valid_moves.is_empty() {
            let score = self.evaluate(board);
            self.transposition_table.insert(board_hash, (score, depth));
            return score;
        }

        let mut best_score;

        if is_maximizing {
            best_score = i32::MIN;
            let mut alpha = alpha;

            for (i, j) in valid_moves {
                let mut new_board = board.clone();
                new_board.set(i, j, self.icon);

                // 再帰的に評価
                let score = self.alpha_beta(&new_board, depth - 1, alpha, beta, false);
                best_score = max(best_score, score);

                alpha = max(alpha, best_score);

                // ベータカット
                if beta <= alpha {
                    break;
                }
            }
        } else {
            best_score = i32::MAX;
            let mut beta = beta;

            let enemy_icon = if self.icon == 'O' { 'X' } else { 'O' };

            for (i, j) in valid_moves {
                let mut new_board = board.clone();
                new_board.set(i, j, enemy_icon);

                let score = self.alpha_beta(&new_board, depth - 1, alpha, beta, true);
                best_score = min(best_score, score);

                beta = min(beta, best_score);

                if beta <= alpha {
                    break;
                }
            }
        }

        self.transposition_table
            .insert(board_hash, (best_score, depth));

        best_score
    }

    // 最適な手を探す
    fn find_best_move(&mut self, board: &Board) -> (usize, usize, i32) {
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);

        let valid_moves = self.find_valid_moves_around(board);

        self.transposition_table.clear();

        let center = SIZE / 2;
        let mut scored_moves: Vec<((usize, usize), i32)> = valid_moves
            .iter()
            .map(|&(i, j)| {
                let distance =
                    (i as isize - center as isize).abs() + (j as isize - center as isize).abs();
                ((i, j), -(distance as i32))
            })
            .collect();

        scored_moves.sort_by_key(|&(_, score)| -score);

        for ((i, j), _) in scored_moves {
            let mut new_board = board.clone();
            new_board.set(i, j, self.icon);

            let score = self.alpha_beta(&new_board, self.max_depth - 1, i32::MIN, i32::MAX, false);

            if score > best_score {
                best_score = score;
                best_move = (i, j);
            }
        }

        (best_move.0, best_move.1, best_score)
    }
}

impl GomokuPlayer for AI {
    fn get(&mut self, board: &Board) -> (usize, usize) {
        println!(
            "{}のターン (AI): ミニマックス法で最適な位置を探しています...",
            self.icon
        );

        let is_first_move = board.get_board().iter().flatten().all(|&cell| cell == '#');
        if is_first_move {
            let center = SIZE / 2;
            println!("AIは初手で中央 ({}, {}) を選択しました", center, center);
            return (center, center);
        }

        let (best_x, best_y, best_score) = self.find_best_move(board);

        println!(
            "AIは ({}, {}) を選択しました (スコア: {})",
            best_x, best_y, best_score
        );
        (best_x, best_y)
    }

    fn get_icon(&self) -> char {
        self.icon
    }
}

fn extract_lines(cells: &[[char; SIZE]; SIZE]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    (0..SIZE).for_each(|r| {
        let row_str: String = cells[r].iter().collect();
        result.push(row_str);
    });

    for c in 0..SIZE {
        let col_str: String = (0..SIZE).map(|r| cells[r][c]).collect();
        result.push(col_str);
    }

    for k_signed in -(SIZE as isize - 1)..=(SIZE as isize - 1) {
        let mut diag_chars: Vec<char> = Vec::new();
        (0..SIZE).for_each(|r| {
            // 対応する c を計算: c = r - k
            let c_signed = r as isize - k_signed;
            // c が配列の範囲内かチェック
            if c_signed >= 0 && c_signed < SIZE as isize {
                let c = c_signed as usize;
                diag_chars.push(cells[r][c]);
            }
        });
        if !diag_chars.is_empty() {
            result.push(diag_chars.iter().collect());
        }
    }

    for k in 0..=(2 * (SIZE - 1)) {
        let mut diag_chars: Vec<char> = Vec::new();
        (0..SIZE).for_each(|r| {
            if k >= r {
                let c = k - r;
                if c < SIZE {
                    diag_chars.push(cells[r][c]);
                }
            }
        });
        if !diag_chars.is_empty() {
            result.push(diag_chars.iter().collect());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_evaluate_naname1() {
        let mut board = Board::new('#');
        board.set(6, 1, 'X');
        board.set(5, 2, 'X');
        board.set(4, 3, 'X');

        let ai = AI::new('O');
        let score = ai.evaluate(&board);
        // board.display();
        let mut board2 = Board::new('#');
        board2.set(6, 1, 'O');
        board2.set(5, 2, 'O');
        board2.set(4, 3, 'O');
        let score2 = ai.evaluate(&board2);

        assert!(score < 0); // 敵の石があるのでマイナス評価
        assert!(score2 > 0); // 自分の石があるのでプラス評価
    }
    #[test]
    fn test_evaluate_naname2() {
        let mut board = Board::new('#');
        board.set(1, 10, 'X');
        board.set(2, 11, 'X');
        board.set(3, 12, 'X');

        let ai = AI::new('O');
        let score = ai.evaluate(&board);
        // board.display();

        let mut board2 = Board::new('#');
        board2.set(1, 10, 'O');
        board2.set(2, 11, 'O');
        board2.set(3, 12, 'O');
        let score2 = ai.evaluate(&board2);

        assert!(score < 0); // 敵の石があるのでマイナス評価
        assert!(score2 > 0); // 自分の石があるのでプラス評価
    }

    #[test]
    fn test_evaluate_horizontal() {
        let mut board = Board::new('#');
        board.set(1, 1, 'X');
        board.set(1, 2, 'X');
        board.set(1, 3, 'X');

        let ai = AI::new('O');
        let score = ai.evaluate(&board);
        // board.display();
        let mut board2 = Board::new('#');
        board2.set(1, 1, 'O');
        board2.set(1, 2, 'O');
        board2.set(1, 3, 'O');
        let score2 = ai.evaluate(&board2);

        assert!(score < 0); // 敵の石があるのでマイナス評価
        assert!(score2 > 0); // 自分の石があるのでプラス評価
    }
}
