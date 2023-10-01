// mod printing;

fn main() {
    let mut x = 1;
    x += 1;
    println!("{}", x);

    let s: String = "HI there".to_string();
    let a_slice: &str = &s; // immutable pair of pointer to s.
    println!("{}  {}", s, a_slice);

    let four_ints: [i32; 3] = [1, 2, 3];
    let mut vectory: Vec<i32> = vec![1, 2];

    vectory.push(11);

    println!("{:?} {:?}", vectory, four_ints)
}
