#[inline(always)]
fn f(x: u32) -> u32 {
    (x * 33 + 1) % 1048576
}

fn main() {
    const MAX_ITERATIONS: u64 = 100_000 * 100_000;
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
}
