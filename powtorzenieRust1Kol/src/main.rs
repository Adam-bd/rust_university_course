use std::collections::HashMap;

fn ad_mod(a: u32, b: u32, p: u32) -> u32 {
    let sum = a + b;
    sum % p
}

fn mul_mod(a: u32, b: u32, p: u32) -> u32 {
    let mul = a * b;
    mul % p
}

fn generate_addition_table(p: u32) -> HashMap<(u32, u32), u32> {
    let mut map : HashMap<(u32, u32), u32> = HashMap::new();

    for i in 0..p {
        for j in 0..p {
            map.insert((i, j), ad_mod(i, j, p));
        }
    }
    map
}

fn  evaluate_add(string: String, addition: &HashMap<(u32, u32), u32>) -> Option<u32> {
    let mut v : Vec<u32> = Vec::new();

    for i in string.split('+') {
        match i.parse() {
            Ok(n) => v.push(n),
            Err(_) => {
                return None;
            }
        }
    }

    let mut current : u32 = v[0];
    for &i in &v[1..] {
        let tuple = (current, i);
        match addition.get(&tuple) {
            Some(n) => current = *n,
            None => {
                return None;
            }
        }
    }
    Some(current)
}

fn generate_multiplication_table(p: u32) -> HashMap<(u32, u32), u32> {
    let mut map : HashMap<(u32, u32), u32> = HashMap::new();

    for i in 0..p {
        for j in 0..p {
            map.insert((i, j), mul_mod(i, j, p));
        }
    }
    map
}

fn  evaluate_mul(string: String, multiplication: &HashMap<(u32, u32), u32>) -> Option<u32> {
    let mut v : Vec<u32> = Vec::new();

    for i in string.split('*') {
        match i.parse() {
            Ok(n) => v.push(n),
            Err(_) => {
                return None;
            }
        }
    }

    let mut current : u32 = v[0];
    for &i in &v[1..] {
        let tuple = (current, i);
        match multiplication.get(&tuple) {
            Some(n) => current = *n,
            None => {
                return None;
            }
        }
    }
    Some(current)
}

fn evaluate(string: &String, addition: &HashMap<(u32, u32), u32>, multiplication: &HashMap<(u32, u32), u32>) -> Option<u32> {
    let mut str_vec : Vec<String> = Vec::new();

    for i in string.split('+') {
        match i.parse() {
            Ok(n) => str_vec.push(n),
            Err(_) => {
                return None;
            }
        }
    }

    for i in str_vec {
        evaluate_mul(i, multiplication);
    }
}

fn main() {
    // println!("{}", ad_mod(2, 2, 5));

    let addition_table = generate_addition_table(7);
    // println!("{:?}", addition_table);
    println!("{:?}", evaluate_add("6+2+3".to_string(), &addition_table));

    let multiplication_table = generate_multiplication_table(7);
    println!("{:?}", evaluate_mul("6*2*3".to_string(), &multiplication_table));

}
