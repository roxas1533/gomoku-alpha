use crate::board::Board;
use crate::player::{GomokuPlayer, Player};

pub struct Game<P1: GomokuPlayer, P2: GomokuPlayer> {
    turn: bool,
    player1: P1,
    player2: P2,
    pub board: Board,
}

impl<P1: GomokuPlayer, P2: GomokuPlayer> Game<P1, P2> {
    pub fn new(player1: P1, player2: P2) -> Self {
        Game {
            turn: true,
            player1,
            player2,
            board: Board::new('#'),
        }
    }

    pub fn switch_turn(&mut self) {
        self.turn = !self.turn;
        if self.turn {
            println!("{}のターンです。", self.player1.get_icon());
        } else {
            println!("{}のターンです。", self.player2.get_icon());
        }
    }

    pub fn current_player(&mut self) -> &mut dyn GomokuPlayer {
        if self.turn {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    pub fn set(&mut self, x: usize, y: usize) -> bool {
        let current_player_icon = self.current_player().get_icon();
        if !self.board.settable(x, y) {
            println!("{},{} には置けません。", x, y);
            return false;
        }
        self.board.set(x, y, current_player_icon);
        println!("({}, {})におきました。", x, y);
        true
    }
}

// ヘルパー関数として、プレイヤー対プレイヤーの新しいゲームを作成する関数を追加
impl Game<Player, Player> {
    pub fn _new_pvp() -> Self {
        Game::new(Player::new('X'), Player::new('O'))
    }
}
