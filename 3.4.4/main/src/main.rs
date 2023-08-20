fn main() {
    let v1: Vec<i64> = vec![1];
    println!("{:?}", v1);

    let mut v2 = vec![1];
    v2.push(2);
    v2.push(3);
    println!("{:?}", v2);

    /*
    let v3 = vec![1];
    v3.push(2);
    println!("{:?}", v3);

    /*
error[E0596]: cannot borrow `v3` as mutable, as it is not declared as mutable
  --> src/main.rs:10:5
   |
10 |     v3.push(2);
   |     ^^^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
9  |     let mut v3 = vec![1];
   |         +++

For more information about this error, try `rustc --explain E0596`.
     */
    */

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(&v);
    print(&a);
    print(sv);
    print(sa);

    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}
