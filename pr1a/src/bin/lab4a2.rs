//Napisz funkcję, która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis.

fn rzymskie(napis: &str) -> i32 {
    let mut sum = 0;
    let mut number = 0;
    let mut prev = 0;
    for i in napis.chars() {
        if i == 'I' {
            number = 1;
        } else if i == 'V' {
            number = 5;
        } else if i == 'X' {
            number = 10;
        } else if i == 'L' {
            number = 50;
        } else if i == 'C' {
            number = 100;
        } else if i == 'D' {
            number = 500;
        } else if i == 'M' {
            number = 1000;
        }

        if prev < number {
            sum += number - 2 * prev;
        } else {
         sum += number;
        }
        prev = number;
    }
    sum
}

fn main() {
    let napis = "III";
    let napis2 = "IX";
    let napis3 = "XIX";
    let napis4 = "MCMX";

    println!("{}", rzymskie(napis));
    println!("{}", rzymskie(napis2));
    println!("{}", rzymskie(napis3));
    println!("{}", rzymskie(napis4));
}

