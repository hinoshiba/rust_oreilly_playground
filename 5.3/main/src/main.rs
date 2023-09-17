fn main() {
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;
    //let m1 = &mut r.1; // error[E0596]: cannot borrow `r.1` as mutable, as it is behind a `&` reference

    let m = &mut w;
    let m0 = &mut m.0;
    *m0 = 137;

    let r1 = &m.1;

    println!("{}", r1);

    let m2 = &m.0;
    println!("{}", m2);

}
