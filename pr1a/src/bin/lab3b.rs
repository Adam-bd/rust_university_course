fn rand_gen(seed: &mut u64) -> u64 {
    // m = 9, a = 4, c = 1
    *seed = (*seed * 4 + 1) % 9;
    *seed
}

fn main() {
    let mut seed = 8;
    println!("{}", rand_gen(&mut seed));
}


