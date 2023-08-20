fn main() {
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    println!("{:?}", v);

    v.push(11);
    v.push(13);

    println!("{:?}", v);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    let mut v2 = Vec::new();
    v2.push("hello");
    v2.push("world");
    println!("{:?}", v2);
    //v2.push(3);  //expected `&str`, found integer
}
