use hex::decode;
use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::sync::{Arc, Mutex};
use tiny_keccak::{Hasher, Keccak};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() != 3 {
        eprintln!("Usage: {} <input_string> <hex prefix>", args[0]);
        return;
    }

    // Get the input string from the command line
    let input = &args[1];

    // strip 0x from the hex prefix if it exists
    let hex_arg = args[2].strip_prefix("0x").unwrap_or(&args[2]);
    // Decode the hex prefix
    let prefix = decode(hex_arg).expect("Decoding failed");

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

            // Check if the first bytes of the hash match the prefix
            let prefix_match = prefix.iter().enumerate().all(|(i, &x)| x == output[i]);

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
