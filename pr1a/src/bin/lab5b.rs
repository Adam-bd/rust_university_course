fn wartosc_cyfry(c: char) -> Result<u8, String> {
    if let Some(a)= c.to_digit(10) {
        Ok(a as u8)
    }
    else {
        Err("Podany znak nie jest cyfrą!".to_string())
    }

}

fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    if a.is_empty() || b.is_empty() {
        return Err("Napisy nie mogą być puste".to_string());
    }
}

fn main() {
    println!("{:?}", wartosc_cyfry('4'));
    println!("{:?}", wartosc_cyfry('A'));
}
