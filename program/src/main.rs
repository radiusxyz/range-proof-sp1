//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};

use crypto_bigint::U2048;
use num_bigint::BigUint;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ResultComp {
    pub val: bool,
}

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let n = sp1_zkvm::io::read::<u32>();

    // zeroknight
    /*
    let my_vec = sp1_zkvm::io::read_vec();

    let mut my_slice = [0_u8; 32];
    sp1_zkvm::io::commit_slice(&my_slice);
    */

    let mut a_max: [u8; 256] = [0x99; 256];
    let mut b_max: [u8; 256] = [0xff; 256];

    let a_max_big = BigUint::from_bytes_le(&a_max);
    let b_max_big = BigUint::from_bytes_le(&b_max);

    let comp = ResultComp {
        val: a_max_big < b_max_big,
    };
    // sp1_zkvm::io::commit(&comp.val);
    sp1_zkvm::io::commit(&comp);

    /*
        // Compute the n'th fibonacci number using a function from the workspace lib crate.
        let (a, b) = fibonacci(n);

        // Encode the public values of the program.
        let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });

        // Commit to the public values of the program. The final proof will have a commitment to all the
        // bytes that were committed to.
        sp1_zkvm::io::commit_slice(&bytes);
    */
}
