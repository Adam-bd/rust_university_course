// Utwórz (i wyświetl) przy użyciu pętli oraz bez ich użycia (z iteratorami zamiast tego) wektor zawierający:
fn main(){
    // 1. małe litery alfabetu angielskiego;
    // let v = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    // 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    //
    // println!("Za pomocą pętli: ");
    // for i in &v {
    //     print!("{} ", i);
    // }
    // println!();
    // println!("Za pomocą iteratora: ");
    // let v2: Vec<char> = v.into_iter().collect();
    // print!("{:?}", v2);

    // 2. kwadraty 10. kolejnych liczb całkowitych począwszy od 1;
    // let mut v : Vec<i32> = Vec::new();
    // for i in 1_i32..11 {
    //     v.push(i.pow(2));
    // }
    // println!("Za pomocą pętli: ");
    // for i in &v {
    //     print!("{} ", i);
    // }
    // println!();
    // println!("Za pomocą iteratora: ");
    // let v2: Vec<i32> = (1_i32..11).map(|i| i.pow(2)).collect();
    // v2.iter().for_each(|i| print!("{} ", i));

    // 3. 10 kolejnych potęg dwójki;
    // println!("Za pomocą pętli: ");
    // let mut v : Vec<i32> =  Vec::new();
    // for i in 0..10 {
    //     v.push(2_i32.pow(i));
    // }
    // for i in &v {
    //     print!("{} ", i);
    // }
    //
    // println!();
    // println!("Za pomocą iteratora: ");
    // let v2: Vec<i32> = (0..10).map(|i| 2_i32.pow(i)).collect();
    // v2.iter().for_each(|i| print!("{} ", i));

    // 4. odwrotności wszystkich liczb od 1 do 20;
    // println!("Za pomocą pętli: ");
    // let mut v : Vec<f32> = Vec::new();
    // for i in 1..=20 {
    //     v.push(1.0 / i as f32);
    // }
    // for i in &v {
    //     print!("{} ", i);
    // }
    //
    // println!();
    // println!("Za pomocą iteratora: ");
    // let v2 : Vec<f32> = (1..=20).map(|i| 1.0 / i as f32).collect();
    // v2.iter().for_each(|i| print!("{} ", i));

    // 5. liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.
    println!("Za pomocą pętli: ");
    let mut v : Vec<i32> = Vec::new();
    for i in 1..=100 {
        if i % 3 == 0 && i % 4 != 0 {
            v.push(i);
        }
    }
    for i in v {
        print!("{} ", i);
    }
    println!();
    println!("Za pomocą iteratora: ");
    let v2 : Vec<i32> = (1..=100).filter(|i| (i % 3 == 0) && (i % 4 != 0)).collect();
    v2.iter().for_each(|i| print!("{} ", i));

    println!();
}
