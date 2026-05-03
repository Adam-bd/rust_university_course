// Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą.

fn liczba_doskonala(x : i32) -> bool {
    let mut sum : i32 = 0;
    let mut i = 1;

    while i <= x / 2 {
        if x % i == 0 {
            sum += i;
        }
        i += 1;
    }

    if sum == x {
        true
    } else {
        false
    }
}

fn main() {

    println!("{}", liczba_doskonala(46));

}
