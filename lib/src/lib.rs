use std::str::FromStr;

use alloy_sol_types::sol;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}

//==================================================================//
pub const MODULUS: &str = "8155133734070055735139271277173718200941522166153710213522626777763679009805792017274916613411023848268056376687809186180768200590914945958831360737612803";
pub const BASE: &str = "4";
pub const RANGE: &str = "801983492846582734029851093470192831";
pub const EXPONENT: &str = "239586239580923";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Rangeproof_Input {
    pub base: BigUint, // BigUint cannot be 'Serialized' -> added 'feature:serde'
    pub modulus: BigUint,
    pub range: BigUint,
    pub result: BigUint,
}

impl Rangeproof_Input {
    // Copnstructor that allows custom input
    pub fn new(base_str: &str, modulus_str: &str, range_str: &str) -> Self {
        let base = BigUint::from_str(base_str).expect("Invalid number for Base");
        let modulus = BigUint::from_str(modulus_str).expect("Invalid number for Modulus");
        let range = BigUint::from_str(range_str).expect("Invalid number for Range");

        let result = BigUint::ZERO;

        println!("Initial parameter settings");
        println!("Base: {}", base);
        println!("Modulus: {}", modulus);
        println!("Range: {}", range);
        println!("Result of base^exponent % modulus: {}", result);

        Rangeproof_Input {
            base,
            modulus,
            range,
            result,
        }
    }
}
