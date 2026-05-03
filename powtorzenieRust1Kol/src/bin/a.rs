use std::collections::HashMap;
use std::collections::HashSet;

fn przestepny(rok: u32) {
    if rok % 4  == 0 || (rok % 100 == 0 && rok % 400 == 0) {
        println!("Rok {} jest przestepny.", rok);
    } else {
        println!("Rok {} nie jest przestepny.", rok);
    }
}

fn na_fahrenheita(c_temp: f32) -> f32 {
    let f_temp : f32 = 32.0 + (9.0 / 5.0) * c_temp;
    f_temp
}

fn na_celsjusza(f_temp: f32) -> f32 {
    let c_temp : f32 = (f_temp - 32.0)* (5.0 / 9.0);
    c_temp
}

fn roznica_czasu(czas1: &str, czas2: &str) -> Option<String> {
    let mut vec1 : Vec<i32> = Vec::new();
    let mut vec2 : Vec<i32> = Vec::new();

    for i in czas1.split(':') {
        match i.parse() {
            Ok(n) => vec1.push(n),
            Err(_) => {
                return None;
            }
        }
    }

    for i in czas2.split(':') {
        match i.parse() {
            Ok(n) => vec2.push(n),
            Err(_) => {
                return None;
            }
        }
    }

    if vec1.len() != 3 || vec2.len() != 3 {
        return None;
    }

    let sekundy1 = vec1[0] * 3600 + vec1[1] * 60 + vec1[2];
    let sekundy2 = vec2[0] * 3600 + vec2[1] * 60 + vec2[2];

    let roznica_w_sekundach = (sekundy1 - sekundy2).abs();

    let roznica_godz = roznica_w_sekundach / 3600;
    let roznica_min = (roznica_w_sekundach % 3600) / 60;
    let roznica_sek = roznica_w_sekundach % 60;

    // Tworzenie gotowego stringa za pomocą makra format!
    // {:02} oznacza, że jeśli liczba ma jedną cyfrę, Rust doda z przodu zero (np. "05")
    let roznica_czas = format!("{:02}:{:02}:{:02}", roznica_godz, roznica_min, roznica_sek);

    Some(roznica_czas)
}

fn factorial_while(mut number: u64) -> u64 {
    let mut result : u64 = number;
    number -= 1;

    while number > 0 {
        result *= number;
        number -= 1;
    }
    result
}

fn factorial_for(mut number: u64) -> u64 {
    for i in (1..number).rev() {
        number *= i;
    }
    number
}

fn reverse(number: i32) {
    let str_number = number.abs().to_string();

    for i in str_number.chars().rev() {
        print!("{i}");
    }
    println!();
}

fn median_and_mode(mut vec: Vec<i32>) -> Option<(i32, i32)>{
    if vec.is_empty() {
        return None;
    }

    vec.sort();
    let middle = vec.len() / 2;
    let median : i32 = vec[middle];

    let mut map = HashMap::new();
    for value in vec {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    let mode = map.into_iter().max_by_key(|&(_, count)| count).map(|(key, _)| key).unwrap();

    Some((median, mode))
}

fn duplikaty(vec: Vec<i32>) -> Option<i32> {
    let mut widziane = HashSet::new();

    for i in vec {
        if !widziane.insert(i) {
            return Some(i);
        }
    }
    None
}

fn iloczyn_kolejnych(a: i32, b: i32) -> i32 {
    let vec : Vec<i32> = (a..=b).collect();
    let result = vec.iter().fold(1, |acc, x| acc * x);
    result
}

//duplikaty tego samego znaku
// fn powtarzajacy_sie_znak(vec: Vec<&String>, c: char) -> Result<String, String> {
//
// }

fn main() {
    let rok = 2032;
    przestepny(rok);

    println!("Fahrenheit: {}", na_fahrenheita(30.7));
    println!("Celsjusza: {}", na_celsjusza(87.26));

    let czas1 = "15:31:10";
    let czas2 = "10:30:05";
    println!("{:?}", roznica_czasu(czas1, czas2));

    let number = 5;
    println!("Silnia z {} wynosi: {}", number, factorial_while(number));
    println!("Silnia z {} wynosi: {}", number, factorial_for(number));

    reverse(1234);

    let hello = String::from("Hello");
    let hello2 = String::from("Здравствуйте");
    println!("{}", hello.len());
    println!("{}", hello.chars().count());
    println!("{}", hello2.chars().count());

    let vec = vec![1, 5, 5, 3, 5, 7, 8, 5, 2, 2, 2, 9, 1];
    match median_and_mode(vec.clone()) {
        Some((median, mode)) => println!("Mediana wynosi: {median}, a moda: {mode:?}"),
        None => println!("Lista jest pusta!"),
    }

    println!("{:?}", duplikaty(vec));
    println!("{}", iloczyn_kolejnych(3, 5));

}
