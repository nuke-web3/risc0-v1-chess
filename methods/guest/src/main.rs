// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chess_core::Inputs;
use risc0_zkvm::guest::env;
use shakmaty::{fen::Fen, san::San, CastlingMode, Chess, FromSetup, Move, Position, Setup};

fn main() {
    let inputs: Inputs = env::read();
    let moves: Vec<String> = inputs.moves;
    let initial_state: String = inputs.board;
    env::commit(&initial_state);
    // This is optional to commit:
    env::commit(&moves.len());

    let setup = Setup::from(Fen::from_ascii(initial_state.as_bytes()).unwrap());
    let mut pos = Chess::from_setup(setup, CastlingMode::Standard).unwrap();

    for m in moves {
        // Checked move based on intial board state
        let m: Move = m.parse::<San>().unwrap().to_move(&pos).unwrap();
        pos = pos.play(&m).unwrap();
    }
    assert!(pos.is_checkmate());
}
