use clap::{self, Parser};
use hex::decode;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use tiny_keccak::{Hasher, Keccak};

#[derive(Parser)]
struct Args {
    ///Base domain
    input: String,
    ///Hex pattern to match
    pattern: String,

    ///Whether or not to match on prefix or suffix
    #[arg(long, short, default_value_t = false)]
    suffix: bool,
}

fn main() {
    let args = Args::parse();

    // Get the input string from the command line
    let input = &args.input;

    // strip 0x from the hex prefix if it exists
    let hex_arg = args.pattern.strip_prefix("0x").unwrap_or(&args.pattern);
    // Decode the hex prefix
    let pattern = decode(hex_arg).expect("Decoding failed");

    let suffix = args.suffix;

    // Number of batches to run in parallel
    let num_batches = 8;
    // Shared variable to indicate if the input has been found
    let found = Arc::new(Mutex::new(false));

    // Run the batches in parallel
    (0..num_batches).into_par_iter().for_each(|_| {
        // Create a new thread-local random number generator
        let mut rng = rand::thread_rng();

        // Generate a random 8-byte hex number
        let mut hex_number = rng.gen::<u64>();

        loop {
            // Increment the hex number
            hex_number += 1;
            // Format the input string with the hex number
            let new_input = format!("efficient_{:08x}.{}", hex_number, input);

            // Calculate the Keccak hash of the input string
            let mut output = [0u8; 4];
            let mut keccak = Keccak::v256();
            keccak.update(new_input.as_bytes());
            keccak.finalize(&mut output);

            // Check if the bytes of the hash match the prefix
            let prefix_match: bool;
            if suffix {
                prefix_match = pattern
                    .iter()
                    .rev()
                    .zip(output.iter().rev())
                    .all(|(x, y)| x == y);
            } else {
                prefix_match = pattern.iter().zip(output.iter()).all(|(x, y)| x == y);
            }

            // get the lock on the shared variable now that work is done
            let mut _found = found.lock().unwrap();
            // if the prefix matches the first 4 bytes of the hash, print the input
            if prefix_match {
                *_found = true;
                println!("Input: {}", new_input);
                println!("Keccak Hash: {:?}", output);
                break;
            }
            // if a suitable input has been found in another thread, break out of the loop
            if *_found {
                break;
            }
        }
    })
}
