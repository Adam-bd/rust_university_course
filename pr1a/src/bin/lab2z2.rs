/* Napisz funkcję, która dla danego całkowitego dodatniego n zwraca numer iteracji,
 w której osiągamy jedynkę w problemie Collatza (np. dla n=12 wynikiem jest 9).
*/

fn main(){
    let mut n : u64 = 37;
    let mut i = 0;

    while n != 1 {
        if n % 2 == 0 {
                n = n / 2;
        } else {
            n = 3*n + 1;
        }
        i += 1;
    }

    println!("{i}");
}
