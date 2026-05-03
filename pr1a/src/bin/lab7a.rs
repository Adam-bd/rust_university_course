use std::collections::HashSet;
use std::collections::HashMap;

// Z użyciem iteratorów napisz funkcję, która usuwa duplikaty z wektora (zachowując kolejność).
fn new_vec_without_duplicate_char(v : &Vec<char>) -> Vec<char> {
    let mut set = HashSet::new();
    v.iter().filter(|&a| set.insert(a)).cloned().collect()
}

fn new_vec_without_duplicate_int(v : &Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    v.iter().filter(|&a| set.insert(a)).cloned().collect()
}

fn delete_duplicate_char(v : &mut Vec<char>) {
    let mut set = HashSet::new();
    v.retain(|a| set.insert(a.clone()));
}

fn delete_duplicate_int(v : &mut Vec<i32>) {
    let mut set = HashSet::new();
    v.retain(|a| set.insert(a.clone()));
}

// Z użyciem iteratorów napisz funkcję, która zlicza wystąpienia każdego elementu w wektorze i zwraca HashMap<i32, usize>.
fn count_elements(v : &Vec<i32>) -> HashMap<i32, usize> {
    let mut map = HashMap::new();
    v.iter().for_each(|&number| *map.entry(number).or_insert(0) += 1);
    map
}

// Z użyciem iteratorów napisz funkcję, która łączy dwa wektory w jeden, naprzemiennie pobierając elementy z obu.
fn concatenate_two_vectors(v1 : &Vec<i32>, v2 : &Vec<i32>) -> Vec<i32> {

}


fn main() {
    let mut vec = vec!['a', 'm', 'g', 'h', 'a', 'd', 'h', 'k'];
    let mut vec2 = vec![1, 2, 3, 4, 5, 3, 6, 2, 7, 5, 8, 3, 1, 4, 9];
    println!("{:?}", new_vec_without_duplicate_char(&vec));
    println!("{:?}", new_vec_without_duplicate_int(&vec2));
    delete_duplicate_char(&mut vec);
    delete_duplicate_int(&mut vec2);
    println!("{:?}", vec);
    println!("{:?}", vec2);

    let vec3 = vec![1, 1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4];
    println!("{:?}", count_elements(&vec3));

}
