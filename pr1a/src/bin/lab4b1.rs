//Napisz funkcję, która zwróci napis zawierający co drugi znak z danego napisu.

// fn co_drugi_znak(napis: &str) -> String {
//     let mut string = "".to_string();
//     let mut count = 0;
//     for i in napis.chars() {
//         if count % 2 == 0 {
//             string.push(i);
//         }
//         count += 1;
//     }
//     string
// }

fn co_drugi_znak(napis: &str) -> String {
    let mut s = String::new();
    for i in napis.chars().step_by(2) {
        s.push(i);
    }
    s
}

fn main() {
    let napis = "napis";
    println!("{}", co_drugi_znak(napis));
}
