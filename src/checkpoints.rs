extern crate num_bigint;

const MOD: u32 = 1048576;

#[inline(always)]
fn f(x: u32) -> u32 {
    (x * 33 + 1) % 1048576
}

fn compute_columns(
    mut v: u32,
    from_row: usize,
    to_row: usize,
    table_size: usize,
    cols: &mut [u32],
) -> u32 {
    for _ in from_row..to_row {
        for x in 0..table_size {
            let xv = unsafe { cols.get_unchecked_mut(x) };
            *xv = (*xv + v) % MOD;
            v = f(v);
        }
    }
    v
}

pub fn make_pad(table_size: u32, cypher_len: u32) -> Vec<u8> {
    let cols = {
        let table_size = table_size as usize;
        let mut cols: Vec<u32> = vec![0; table_size];
        compute_columns(1, 0, table_size, table_size, &mut cols);
        cols
    };

    let pad = {
        use std::fmt::Write;
        let mut bignum_str = String::new();
        for n in &cols {
            write!(bignum_str, "{}", n).unwrap();
        }
        let bi = num_bigint::BigUint::parse_bytes(bignum_str.as_bytes(), 10).unwrap();
        let mut pad = bi.to_radix_le(27);
        pad
    };

    pad.into_iter().rev().take(cypher_len as usize).collect()
}

fn main() {
    const MAX_TABLE_SIZE: u64 = 100_000;
    const MAX_ITERATIONS: u64 = MAX_TABLE_SIZE * MAX_TABLE_SIZE;
    const NUM_CHECKPOINTS: u64 = 20;
    const CHECKPOINT_EVERY: u64 = MAX_ITERATIONS / NUM_CHECKPOINTS;
    let mut checkpoints = Vec::new();

    let mut v = 0_u32;
    for i in 0..MAX_ITERATIONS {
        v = f(v);
        if i % CHECKPOINT_EVERY == 0 {
            eprintln!("checkpoint: {:?}", (i, v));
            checkpoints.push((i, v));
        }
    }

    println!("const checkpoints: [(u64, u32); {}] = [", checkpoints.len());
    for cp in &checkpoints {
        println!("\t{:?},", cp);
    }
    println!("]");

    eprintln!("making lookup table for bit table...");
    // let max_cipher = 10_u32.pow(6);
    let max_cipher = 1000;
    let pad = make_pad(MAX_TABLE_SIZE as u32, max_cipher);
    println!(
        "let pad_lut: Vec<(u32, u32, Vec<u8>)> = vec![({}, {}, vec!{:?})]);",
        max_cipher, MAX_TABLE_SIZE, pad
    );
}
