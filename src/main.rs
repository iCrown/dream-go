// Copyright 2017 Karl Sundequist Blomdahl <karl.sundequist.blomdahl@gmail.com>
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

extern crate dream_go;
extern crate time;

use dream_go::{dataset, gtp, nn, mcts};
use std::env;

/// Main function.
fn main() {
    // keep everything that is before the first "--" indicator as potential
    // program arguments
    let args = env::args()
        .skip(1)
        .take_while(|arg| arg != "--")
        .collect::<Vec<String>>();

    // everything after "--" and anything in the potential program arguments
    // that does not begin with a "-"
    let mut remaining = env::args()
        .skip(args.len() + 2)
        .collect::<Vec<String>>();

    for arg in &args {
        if !arg.starts_with("-") {
            remaining.push(arg.clone());
        }
    }

    if args.iter().any(|arg| arg == "--dataset") {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        for entry in dataset::of(&remaining) {
            if entry.write_into(&mut handle).is_err() {
                break
            }
        }
    } else if args.iter().any(|arg| arg == "--self-play") {
        let network = nn::Network::new()
            .expect("no model found");
        let n = if remaining.len() > 0 {
            remaining[0].parse::<usize>().unwrap()
        } else {
            1
        };

        for _ in 0..n {
            let result = mcts::self_play(&network);

            println!("{}", result);
        }
    } else if args.iter().any(|arg| arg == "--policy-play") {
        let network = nn::Network::new()
            .expect("no model found");
        let receiver = mcts::policy_play(network);

        let mut n = if remaining.len() > 0 {
            remaining[0].parse::<usize>().unwrap()
        } else {
            ::std::usize::MAX
        };

        while n > 0 {
            match receiver.recv() {
                Ok(result) => { println!("{}", result); },
                _ => { unreachable!(); }
            }

            n -= 1;
        }
    } else if args.iter().any(|arg| arg == "--gtp") || args.len() == 0 {
        gtp::run();
    } else {
        println!("Usage: ./dream-go [options]");
        println!("");
        println!("  --dataset <files...>  Extract a dataset for training from the given SGF files");
        println!("  --self-play <n>       Extract a dataset from self-play containing n examples");
        println!("  --policy-play <n>     Extract a dataset from self-play using only the policy network");
        println!("  --gtp                 Run GTP client (default)");
    }
}
