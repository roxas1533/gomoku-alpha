use wasm_bindgen::prelude::*;

pub mod ai;
pub mod board;
pub mod game;
pub mod player;

use crate::ai::AI;
use crate::board::{Board, SIZE};
use crate::player::GomokuPlayer;

// WebAssemblyから使用可能なGomokuGameクラス
#[wasm_bindgen]
pub struct GomokuGame {
    board: Board,
    ai: AI,
    current_player: String, // "black" または "white"
}

impl Default for GomokuGame {
    fn default() -> Self {
        Self {
            board: Board::new('#'),
            ai: AI::new('O'),
            current_player: "black".to_string(),
        }
    }
}

#[wasm_bindgen]
impl GomokuGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // コンソールログを有効にする
        #[cfg(feature = "console_error_panic_hook")]
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        Default::default()
    }

    // 盤面の状態を取得するメソッド
    #[wasm_bindgen]
    pub fn get_board(&self) -> JsValue {
        let mut result = Vec::new();

        for i in 0..SIZE {
            let mut row = Vec::new();
            for j in 0..SIZE {
                let cell = self.board.get_cell(i, j);
                let stone = match cell {
                    'X' => "black",
                    'O' => "white",
                    _ => "",
                };
                row.push(stone);
            }
            result.push(row);
        }

        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    // プレイヤーが石を置くメソッド
    #[wasm_bindgen]
    pub fn place_stone(&mut self, row: usize, col: usize) -> bool {
        if !self.board.settable(row, col) {
            return false;
        }

        // 現在のプレイヤーに基づいて石の種類を決定
        let stone = if self.current_player == "black" {
            'X'
        } else {
            'O'
        };
        self.board.set(row, col, stone);

        true
    }

    // AIの手を取得するメソッド
    #[wasm_bindgen]
    pub fn ai_move(&mut self) -> JsValue {
        let stone = if self.current_player == "black" {
            'X'
        } else {
            'O'
        };
        self.ai = AI::new(stone);

        let board_clone = self.board.clone();
        let (row, col) = self.ai.get(&board_clone);

        // AIが選んだ位置に石を置く
        if self.board.settable(row, col) {
            self.board.set(row, col, stone);

            js_sys::Array::of2(&JsValue::from(row as u32), &JsValue::from(col as u32)).into()
        } else {
            js_sys::Array::new().into()
        }
    }

    #[wasm_bindgen]
    pub fn switch_player(&mut self) {
        self.current_player = if self.current_player == "black" {
            "white".to_string()
        } else {
            "black".to_string()
        };
    }

    // 現在のプレイヤーを取得
    #[wasm_bindgen]
    pub fn get_current_player(&self) -> String {
        self.current_player.clone()
    }

    // 指定した位置で勝利かどうか判定
    #[wasm_bindgen]
    pub fn check_win(&self, row: usize, col: usize) -> bool {
        let stone = if self.current_player == "black" {
            'X'
        } else {
            'O'
        };
        self.board.check(row, col, stone)
    }
}

// JsValue変換用にserdeを使用
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<Array<string>>")]
    pub type BoardType;
}
