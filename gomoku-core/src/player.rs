use crate::board::Board;

// 共通トレイトの定義
pub trait GomokuPlayer {
    fn get(&mut self, board: &Board) -> (usize, usize);
    fn get_icon(&self) -> char;
}

pub struct Player {
    pub icon: char,
}

impl Player {
    pub fn new(icon: char) -> Self {
        Player { icon }
    }
}

impl GomokuPlayer for Player {
    fn get(&mut self, board: &Board) -> (usize, usize) {
        let mut input = String::new();
        println!("{}のターンです。座標を入力してください (x y): ", self.icon);
        std::io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() != 2 {
            println!("無効な入力です。");
            return self.get(board);
        }

        let (x, y) = (coords[0], coords[1]);
        if !board.settable(x, y) {
            println!("{}, {}には置けません。", x, y);
            return self.get(board);
        }
        (x, y)
    }

    fn get_icon(&self) -> char {
        self.icon
    }
}
