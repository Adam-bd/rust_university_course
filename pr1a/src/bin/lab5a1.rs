// Napisz funkcje zamieniającą zapis liczby całkowitej bez znaku w systemie ósemkowym na zapis w systemie dwójkowym. Wynik ma być najkrótszy możliwy, niepusty. Wynik None ma oznaczać wystąpienie w parametrze z niedozwolonego znaku (spoza cyfr ósemkowych) lub pusty napis w parametrze.

fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    let mut s = String::new();

    for i in z.chars() {
        if i > '7' {
            return None;
        }
        if i == '0' {
            s.push_str("000");
        } else if i == '1' {
            s.push_str("001");
        } else if i == '2' {
            s.push_str("010");
        } else if i == '3' {
            s.push_str("011");
        } else if i == '4' {
            s.push_str("100");
        } else if i == '5' {
            s.push_str("101");
        } else if i == '6' {
            s.push_str("110");
        } else if i == '7' {
            s.push_str("111");
        }
    }

    Some(s)
}

// Napisz funkcje obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest None.

fn wartosc_syst2(z: &str) -> Option<u8> {
    let length = z.len();
    if length == 0 {
        return None;
    }
    let mut wartosc : u8 = 0;
    let mut liczba_zer = 0;

    for i in z.chars() {
        if i == '0' {
            liczba_zer += 1;
        } else {
            break;
        }
    }

    if length - liczba_zer > 8 {
        return None;
    }

    let mut potega = 0;
    for i in z.chars().rev(){
        if i != '0' && i != '1' {
            return None;
        }
        if i == '1' {
            wartosc += 2_u8.pow(potega);
        }
        potega += 1;
    }
    Some(wartosc)
}

// Napisz funkcje obliczającą wartość całkowitą bez znaku zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest None.

fn wartosc_syst8(z: &str) -> Option<u8> {
    let dwojkowo = zamien_syst8_na_syst2(z)?;
    let wartosc = wartosc_syst2(&dwojkowo)?;
    Some(wartosc)
}

fn main() {
    println!("1 funkcja:");
    println!("{:?}", zamien_syst8_na_syst2("52"));
    println!("{:?}", zamien_syst8_na_syst2("647"));
    println!("{:?}", zamien_syst8_na_syst2("73219"));

    println!("\n2 funkcja:");

    println!("{:?}", wartosc_syst2("101"));
    println!("{:?}", wartosc_syst2("000000000000000101"));
    println!("{:?}", wartosc_syst2("1010301"));
    println!("{:?}", wartosc_syst2("000000000001111000110000100101"));

    println!("\n3 funkcja:");
    println!("{:?}", wartosc_syst8("52")); //101010 = 42d
    println!("{:?}", wartosc_syst8("377")); //11111111 = 225d
    println!("{:?}", wartosc_syst8("19"));
}
