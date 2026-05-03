/*Napisz funkcję

fn met_newt(f: fn(x: f64) -> f64, fprim: fn(x: f64) -> f64, x0: f64, eps: f64, n: u128) -> f64

realizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona (przy założeniu, że funkcje w parametrach spełniają odpowiednie założenia; druga jest pochodną pierwszej) — w czterech wersjach:

    - iteracyjnej z pętlą loop (z ewentualnymi break continue return); <--
    - iteracyjnej z pętlą while (bez żadnych break continue return);
    - rekurencyjnej;
    - iteracyjnej z pętlą for (z ewentualnymi break continue return). */

fn met_newt_l(f: fn(x: f64) -> f64, fprim: fn(x: f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    let mut i = 0;
    let mut x = x0;

    loop {
        x = x - f(x) / fprim(x);
        i += 1;

        println!("{x}");

        if f(x).abs() < eps || i >= n{
            break;
        }
    }
    x
}

fn met_newt_w(f: fn(x: f64) -> f64, fprim: fn(x: f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    let mut i = 0;
    let mut x = x0;
    while f(x).abs() >= eps && i < n {
        x = x - f(x) / fprim(x);
        i += 1;
        println!("{x}");
    }
    x
}

fn met_newt_r(f: fn(x: f64) -> f64, fprim: fn(x: f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    if f(x0).abs() < eps || n == 0{
        return x0;
    }

    let x = x0 - f(x0) / fprim(x0);
    println!("{}", x);
    met_newt_r(f, fprim, x, eps, n - 1)

}

fn met_newt_f(f: fn(x: f64) -> f64, fprim: fn(x: f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    let mut x = x0;
    for i in 1..n {
        if f(x).abs() < eps {
            break;
        } else {
            x = x - f(x)/ fprim(x);
        }
        println!("{x}")
    }
    x
}

fn f1(x: f64) -> f64 {
    (x - 3.0) * (x - 1.0) * (x + 8.0)
}

fn f1prim(x : f64) -> f64 {
    3.0 * x * x + 8.0 * x - 29.0
}

fn main() {
    // let f = |x: f64| x * x - 2.0;
    // let fprim = |x: f64| 2.0 * x;
    // let x0 = 0.1;
    let x0 = 2.1;
    // let x0 = -7.0;
    let eps = 0.00001;
    let n = 10;

    met_newt_l(f1, f1prim, x0, eps, n);
    println!("---------");
    met_newt_w(f1, f1prim, x0, eps, n);
    println!("---------");
    met_newt_r(f1, f1prim, x0, eps, n);
    println!("---------");
    met_newt_f(f1, f1prim, x0, eps, n);
}
