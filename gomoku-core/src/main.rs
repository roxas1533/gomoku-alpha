mod ai;
mod board;
mod game;
mod player;

use ai::AI;
use player::Player;

fn main() {
    // let mut game = game::Game::new_pvp();

    // プレイヤー対AIのゲーム
    let mut game = game::Game::new(Player::new('X'), AI::new('O'));

    game.board.display();
    loop {
        let board_clone = game.board.clone();

        let current_player = game.current_player();
        let coords = current_player.get(&board_clone);
        let current_icon = current_player.get_icon();

        let (x, y) = (coords.0, coords.1);
        let result = game.set(x, y);

        if result {
            if game.board.check(x, y, current_icon) {
                game.board.display();
                println!("{}の勝ちです！", current_icon);
                break;
            }
            game.switch_turn();
        } else {
            println!("もう一度入力してください。");
        }
        game.board.display();
    }
}
