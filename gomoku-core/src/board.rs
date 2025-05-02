pub const SIZE: usize = 15;

pub struct Board {
    cells: [[char; SIZE]; SIZE],
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board { cells: self.cells }
    }
}

impl Board {
    pub fn new(value: char) -> Self {
        Board {
            cells: [[value; SIZE]; SIZE],
        }
    }

    pub fn display(&self) {
        print!("  ");
        for i in 0..SIZE {
            print!("{:X} ", i);
        }
        println!();
        for row in self.cells.iter().enumerate() {
            print!("{:X} ", row.0); // 行番号を表示
            for cell in row.1 {
                print!("{} ", cell);
            }
            println!();
        }
    }
    pub fn settable(&self, x: usize, y: usize) -> bool {
        if x < SIZE && y < SIZE {
            self.cells[x][y] == '#'
        } else {
            false
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        if self.settable(x, y) {
            self.cells[x][y] = value;
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> char {
        if x < SIZE && y < SIZE {
            self.cells[x][y]
        } else {
            '#'
        }
    }

    pub fn get_board(&self) -> &[[char; SIZE]; SIZE] {
        &self.cells
    }

    pub fn check(&self, x: usize, y: usize, value: char) -> bool {
        // 横方向 (左右)
        if self.check_direction(x, y, value, 1, 0) {
            return true;
        }
        // 縦方向 (上下)
        if self.check_direction(x, y, value, 0, 1) {
            return true;
        }
        // 斜め方向 (左上から右下)
        if self.check_direction(x, y, value, 1, 1) {
            return true;
        }
        // 斜め方向 (右上から左下)
        if self.check_direction(x, y, value, 1, -1) {
            return true;
        }

        false
    }

    fn check_direction(&self, x: usize, y: usize, value: char, dx: isize, dy: isize) -> bool {
        let mut count = 1; // 中心の石を含める

        // 正方向にチェック
        let mut nx = x as isize + dx;
        let mut ny = y as isize + dy;
        while nx >= 0
            && nx < SIZE as isize
            && ny >= 0
            && ny < SIZE as isize
            && self.cells[nx as usize][ny as usize] == value
        {
            count += 1;
            nx += dx;
            ny += dy;
        }

        // 負方向にチェック
        nx = x as isize - dx;
        ny = y as isize - dy;
        while nx >= 0
            && nx < SIZE as isize
            && ny >= 0
            && ny < SIZE as isize
            && self.cells[nx as usize][ny as usize] == value
        {
            count += 1;
            nx -= dx;
            ny -= dy;
        }

        // 5個以上並んでいればtrue
        count >= 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new('#');

        // 全てのセルが'#'で初期化されていることを確認
        for i in 0..SIZE {
            for j in 0..SIZE {
                assert_eq!(board.cells[i][j], '#');
            }
        }
    }

    #[test]
    fn test_settable() {
        let board = Board::new('#');

        // 範囲内の空いているセルは設置可能
        assert!(board.settable(0, 0));
        assert!(board.settable(1, 1));
        assert!(board.settable(SIZE - 1, SIZE - 1));

        // 範囲外のセルは設置不可
        assert!(!board.settable(SIZE, SIZE)); // 範囲外

        // すでに石が置かれている場所は設置不可のテスト
        let mut board = Board::new('#');
        board.set(5, 5, 'O');
        assert!(!board.settable(5, 5));
    }

    #[test]
    fn test_set() {
        let mut board = Board::new('#');

        // 有効な位置に石を置く
        board.set(5, 5, 'O');
        assert_eq!(board.cells[5][5], 'O');

        // すでに石がある位置に別の石を置こうとしても変化しない
        board.set(5, 5, 'X');
        assert_eq!(board.cells[5][5], 'O'); // 上書きされない

        // 範囲外の位置に石を置こうとしても変化しない
        board.set(SIZE, SIZE, 'X');
        assert_eq!(board.cells[SIZE - 1][SIZE - 1], '#'); // 変更されない
    }

    #[test]
    fn test_check_win_horizontal() {
        let mut board = Board::new('#');

        // 横方向に5つ連続で石を置く
        for i in 1..6 {
            board.set(i, 5, 'O');
        }

        // 勝利条件を満たすかチェック
        assert!(board.check(3, 5, 'O')); // 連続した中の一つの石からチェック
    }

    #[test]
    fn test_check_win_vertical() {
        let mut board = Board::new('#');

        // 縦方向に5つ連続で石を置く
        for i in 1..6 {
            board.set(5, i, 'O');
        }

        // 勝利条件を満たすかチェック
        assert!(board.check(5, 3, 'O')); // 連続した中の一つの石からチェック
    }

    #[test]
    fn test_check_win_diagonal1() {
        let mut board = Board::new('#');

        // 左上から右下への対角線に5つ連続で石を置く
        for i in 1..6 {
            board.set(i, i, 'O');
        }

        // 勝利条件を満たすかチェック
        assert!(board.check(3, 3, 'O')); // 連続した中の一つの石からチェック
    }

    #[test]
    fn test_check_win_diagonal2() {
        let mut board = Board::new('#');

        // 右上から左下への対角線に5つ連続で石を置く
        for i in 1..6 {
            board.set(i, 6 - i, 'O');
        }

        // 勝利条件を満たすかチェック
        assert!(board.check(3, 3, 'O')); // 連続した中の一つの石からチェック
    }

    #[test]
    fn test_check_no_win() {
        let mut board = Board::new('#');

        // 横方向に4つだけ石を置く（勝利条件に満たない）
        for i in 1..5 {
            board.set(i, 5, 'O');
        }

        // 勝利条件を満たさない
        assert!(!board.check(3, 5, 'O'));

        // 異なる色の石が混ざっている場合
        let mut board = Board::new('#');
        board.set(1, 1, 'O');
        board.set(2, 2, 'O');
        board.set(3, 3, 'X'); // 異なる色
        board.set(4, 4, 'O');
        board.set(5, 5, 'O');

        // 勝利条件を満たさない
        assert!(!board.check(2, 2, 'O'));
    }
}
