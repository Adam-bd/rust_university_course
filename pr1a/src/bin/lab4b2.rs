//Zdefiniuj funkcję, która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów.

fn szyfruj(napis: &str, klucz: usize) -> String {

    let mut s : String = String::new();

    for j in (0..napis.len()).step_by(klucz) {
        for i in (j..(j + klucz)).rev(){
            if i < napis.len() {
                s.push(napis.chars().nth(i).unwrap());
            }
        }
    }

    s
}

fn main() {
    let napis = "Aladyn";
    let napis2 = "kaszanka";
    let napis3 = "kot Mruczek";
    println!("{}", szyfruj(napis, 2));
    println!("{}", szyfruj(napis, 5));
    println!("{}", szyfruj(napis2, 3));
    println!("{}", szyfruj(napis3, 9));

}
