/* Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych imię i nazwisko, a zwraca napis
powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska
mają być duże, pozostałe małe. Na przykład, dla danych "jan" oraz "KOWALSKI" funkcja ma zwracać napis "J. Kowalski".
Wskazówka: użyj metod to_lowercase oraz to_uppercase.
*/

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut napis = String::new();

    napis.push(imie.to_uppercase().chars().nth(0).unwrap());
    napis.push_str(". ");

    napis.push(nazwisko.to_uppercase().chars().nth(0).unwrap());

    for i in 1..nazwisko.len() {
        napis.push(nazwisko.to_lowercase().chars().nth(i).unwrap());
    }

    napis
}

fn main() {
    let imie = "michal";
    let nazwisko = "nowak";
    let imie2 = "jan";
    let nazwisko2 = "KOWALSKI";
    println!("{}", wizytowka(imie, nazwisko));
    println!("{}", wizytowka(imie2, nazwisko2));

}
