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
use chess_methods::{CHECKMATE_ELF, CHECKMATE_ID};
use clap::Parser;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use shakmaty::{fen::Fen, CastlingMode, Chess, FromSetup, Position, Setup};

#[derive(Parser)]
struct Cli {
    #[arg(id = "MOVE", default_values = ["Nf6","Qxf7"])]
    moves: Vec<String>,

    #[arg(default_value = "r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 4 4")]
    board: String,
}

fn main() {
    let args = Cli::parse();

    let inputs = Inputs {
        board: args.board,
        moves: args.moves,
    };

    let receipt = chess(&inputs);

    // Verify receipt and parse it for committed data
    receipt.verify(CHECKMATE_ID).unwrap();
    let committed_state: String = receipt.journal.decode().unwrap();
    assert_eq!(inputs.board, committed_state);
    let fen = Fen::from_ascii(committed_state.as_bytes()).unwrap();
    let setup = Setup::from(fen);
    let pos = Chess::from_setup(setup, CastlingMode::Standard).unwrap();

    println!(
        "{:?}'s move with starting board of:\n\n{:?}\n🨀 The pover knows a set of moves resulting in a checkmate in {} turns!",
        pos.turn(),
        pos.board(),
        inputs.moves.len()
    );
}

fn chess(inputs: &Inputs) -> Receipt {
    let env = ExecutorEnv::builder()
        .write(inputs)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    prover.prove(env, CHECKMATE_ELF).unwrap().receipt
}

#[cfg(test)]
mod tests {
    use chess_core::Inputs;

    use crate::chess;

    #[test]
    fn main() {
        const TEST_BOARD: &str =
            "r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 4 4";
        const TEST_MOVES: [&str; 2] = ["Nf6", "Qxf7"];

        chess(&Inputs {
            board: String::from(TEST_BOARD),
            moves: TEST_MOVES.map(|m| m.to_string()).to_vec(),
        });
    }
}
