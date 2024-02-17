fn main() {
    let mut numer: f64 = 2.0; // numerator
    let mut denom: f64 = 1.0; // denominator
    let mut pihalf = numer / denom; // pi/2
    loop {
        denom += 2.0;
        pihalf *= numer / denom;
        numer += 2.0;
        pihalf *= numer / denom;
        println!("Pi = {}", (2.0 * pihalf));
    }
}