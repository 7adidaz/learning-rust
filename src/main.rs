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

    println!("{:?} {:?}", vectory, four_ints);

    // -------

    let tuple: (i32, &str, f32) = (1, "some string", 3.4);
    // let (a, b, c) = tuple;
    println!("{:?}", tuple);

    // -------
    #[derive(Debug)]
    enum location {
        lang(f32),
        lat(f32),
    }

    let lang = location::lang(-15.3);
    println!("{:?}", lang);

    //----- generics

    enum complex<T> {
        real(T),
        img(T),
    }

    struct Foo<T> {
        bar: T,
    }

    impl<T> Foo<T> {
        // Methods take an explicit `self` parameter
        fn bar(&self) -> &T {
            // self is borrowed
            &self.bar
        }
        fn bar_mut(&mut self) -> &mut T {
            // self is mutably borrowed
            &mut self.bar
        }
        fn into_bar(self) -> T {
            // here self is consumed
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.bar());
    // traits are known as interfaces..
    trait Frobnicate<T> {
        fn Frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn Frobnicate(self) -> Option<T> {
            Some(self.bar)
        }
    }
    let another_foo = Foo { bar: 1 };
    println!("{:?}", another_foo.Frobnicate()); // Some(1)

    if true {
        for i in vectory.iter() {
            print!("{}, ", i);
        }
        println!();
    } else {
        println!("else");
    }

    // let s: String = String::from("i'm on the heap");

    let _arr: [i32; 5] = [0; 5];
    let slice = &_arr[1..3]; // points to a section of the array!

    for i in 0..slice.len() {
        match slice.get(i) {
            Some(xval) => print!("{{{}}}", xval),
            None => print!("x"),
        }
    }
    println!();
}
