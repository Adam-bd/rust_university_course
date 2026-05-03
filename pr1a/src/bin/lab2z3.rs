// Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga.

fn armstrong(x : u32) -> bool {
    let mut n = x;
    let mut i : u32 = 0;
    let mut sum : u32 = 0;
    while n > 0 {
        n = n / 10;
        i += 1;
    }

    n = x;
    let mut licznik = i;
    while licznik > 0 {
        let cyfra = (n % 10) as u32;
        sum += cyfra.pow(i);
        n = n / 10;
        licznik -= 1;
    }

    if sum == x {
        true
    } else {
        false
    }

}

fn main() {

    println!("{}", armstrong(134));

}
