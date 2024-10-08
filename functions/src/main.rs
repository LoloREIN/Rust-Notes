fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz saesaparilla(s)!", qty, oz);
    qty * oz
}

fn main() {
    let x = do_stuff(2.0, 12.5);
    print!("{}", x)
}
