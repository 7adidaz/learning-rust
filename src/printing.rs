use std::{fmt, vec};
fn main() {
    let x = 10;

    /* using the {} for place holders and we can specify the order of them.  */
    println!("Hello, world! {1} {0}", x, x + 1);
    //                                              0       1
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // there also can be named arguments:
    println!("this is magic! .. {v}", v = "hi there");

    // can format using a format character
    // :b for binary, :o for octal, :x hexadecimal.
    println!("binary: {:b}", 2);

    println!("{number:>5}", number = 1);
    println!("{number:>number$}", number = 3);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // for printing a 'struct', one has to implement fmt::Display.
    // or can use the fmt::Debug traits.
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print using fmt::Debug
    println!("{:#?}", peter);

    // -------------------------------------
    // using fmt::Display
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // one can implement diffrent formatters (e.i :b and :o) by specifying fmt::Binary.
    // this handels the unspecified cases: {}
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // takes the stream 'f' and writes to it like println!
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("debut: {:?}", point);
}
