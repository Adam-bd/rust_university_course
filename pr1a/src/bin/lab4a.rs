// Napisz funkcję, która zliczy i zwróci ile jest danych znaków w danym napisie.
fn liczba_wystapien(napis: &str, znak: char) -> i32 {
    let mut count = 0;
    for i in napis.chars() {
        if znak == i {
            count += 1;
        }
    }
    count
}

fn main() {
    let napis = "anakonda";
    let znak = 'a';
    let napis2 = "kajak";
    let znak2 = 'k';
    println!("{}", liczba_wystapien(napis, znak));
    println!("{}", liczba_wystapien(napis2, znak2));
}
