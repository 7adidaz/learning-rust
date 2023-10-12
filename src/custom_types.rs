//unit struct (used for generics)
struct Unit;

// tuple struct
struct Pair(i32, i32);

struct Point {
    x: f32,
    y: f32,
}

//struct
struct Rectangel {
    top_left: Pair,
    bottom_right: Pair,
}

enum Event {
    SteerLeft,
    SteerRight,
}

enum ImpCounter {
    North,
    South,
}

fn event_caller(event: Event) {
    match event {
        Event::SteerLeft => println!("Steering left"),
        Event::SteerRight => println!("Steering to the right"),
    }
}

fn main() {
    let bottom_right: Point = Point { x: 10.3, y: 5f32 };
    let top_left: Point = Point {
        x: 2f32,
        ..bottom_right
    };

    let left_steer = Event::SteerLeft;
    event_caller(left_steer);

    let north = ImpCounter::North;
    println!("{}", north as i32);

    // one also can use the word "use" to set manual scoping
    use crate::ImpCounter::*;
    println!("{}", South as i32);
}
