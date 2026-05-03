// Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).

fn main() {
   let mut i = 33;

   while i < 127 {
       let c = i as u8 as char;
       println!("{c} {i:?}");
       i += 1;
    }

}
