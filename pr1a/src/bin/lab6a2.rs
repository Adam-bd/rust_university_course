fn krotkie_napisy(v : &Vec<String>) -> Vec<String> {
    v.iter().filter(|i| i.chars().count() < 4).cloned().collect()
}

fn napisy_bez_aA(v : &Vec<String>) -> Vec<String> {
    v.
}

fn main() {
    let v = vec!["ala".to_string(), "ma".to_string(), "czerwonego".to_string(), "kota".to_string()];

    println!("{:?} ", krotkie_napisy(&v));

}
