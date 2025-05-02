import { useState, useEffect, useRef, useCallback } from "react";
import "./App.css";
import Header from "./Header";
import init, { GomokuGame } from "../gomoku-core/pkg/gomoku_alpha";
import { Button } from "@mantine/core";

function App() {
	const BOARD_SIZE = 15;
	const [board, setBoard] = useState<Array<Array<string>>>(
		Array(BOARD_SIZE)
			.fill(null)
			.map(() => Array(BOARD_SIZE).fill("")),
	);
	const [currentPlayer, setCurrentPlayer] = useState<string>("black");
	const [isGameOver, setIsGameOver] = useState<boolean>(false);
	const [winner, setWinner] = useState<string | null>(null);
	const [isWasmLoaded, setIsWasmLoaded] = useState<boolean>(false);
	const [isAIThinking, setIsAIThinking] = useState<boolean>(false);
	const gameRef = useRef<GomokuGame | null>(null);

	const initWasm = useCallback(async () => {
		try {
			await init();
			gameRef.current = new GomokuGame();
			// 初期状態の盤面を取得
			const initialBoard = gameRef.current.get_board() as Array<Array<string>>;
			setBoard(initialBoard);

			// 初期プレイヤーを取得
			setCurrentPlayer(gameRef.current.get_current_player());
            setIsGameOver(false);
			setIsWasmLoaded(true);
		} catch (error) {
			console.error("Failed to load WASM module:", error);
		}
	}, []);
	// WASMモジュールを初期化
	useEffect(() => {
		initWasm();
	}, [initWasm]);

	useEffect(() => {
		if (isWasmLoaded && currentPlayer === "white" && !isGameOver) {
			setIsAIThinking(true);

			setTimeout(() => {
				const game = gameRef.current;
				if (!game) return;

				// AIの手を取得
				const aiMoveResult = game.ai_move() as number[];
				if (aiMoveResult.length === 2) {
					const [row, col] = aiMoveResult;

					// 勝利チェック
					const hasWon = game.check_win(row, col);
					if (hasWon) {
						setIsGameOver(true);
						setWinner("white");
					}

					// 盤面の状態を更新
					setBoard(game.get_board() as Array<Array<string>>);
					game.switch_player();
					setCurrentPlayer(game.get_current_player());
                    setIsAIThinking(false);
				}
			}, 500);

		}
	}, [currentPlayer, isGameOver, isWasmLoaded]);

	const handleCellClick = (row: number, col: number) => {
		if (!isWasmLoaded || board[row][col] !== "" || isGameOver || isAIThinking)
			return;

		if (gameRef.current === null) return;

		// 石を置く
		const placed = gameRef.current.place_stone(row, col);
		if (!placed) return;

		// 勝利チェック
		const hasWon = gameRef.current.check_win(row, col);
		if (hasWon) {
			setIsGameOver(true);
			setWinner(currentPlayer);
		}

		// 盤面の状態を更新
		setBoard(gameRef.current.get_board() as Array<Array<string>>);
		// 現在のプレイヤーを更新
		gameRef.current.switch_player();
		setCurrentPlayer(gameRef.current.get_current_player());
	};

	const resetGame = () => {
		initWasm();
	};

	return (
		<div className="app-container">
			<Header />
			<main className="main-content">
				<div className="game-controls">
					<Button variant="light" onClick={resetGame} disabled={!isWasmLoaded}>
						ゲームをリセット
					</Button>
				</div>
				<div className="game-info">
					{isGameOver ? (
						<div className="game-over">
							<span className={`player-indicator ${winner}`} />
							<span>{winner === "black" ? "黒" : "白"}の勝ちです！</span>
						</div>
					) : (
						<div className="player-turn">
							<span className={`player-indicator ${currentPlayer}`} />
							<span>
								{isAIThinking
									? "AIが考え中..."
									: `${currentPlayer === "black" ? "黒" : "白"}の番です`}
							</span>
						</div>
					)}
				</div>
				<div className="gomoku-board">
					{board.map((row, rowIndex) => (
						// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
						<div key={`row-${rowIndex}`} className="board-row">
							{row.map((cell, colIndex) => (
								// biome-ignore lint/a11y/useKeyWithClickEvents: <explanation>
								<div
									// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
									key={`${rowIndex}-${colIndex}`}
									className={`board-cell ${cell}`}
									onClick={() => handleCellClick(rowIndex, colIndex)}
								>
									{cell && <div className={`stone ${cell}`} />}
								</div>
							))}
						</div>
					))}
				</div>
			</main>
		</div>
	);
}

export default App;
