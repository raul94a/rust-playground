use point::Point;
use square::Square;

mod point;
mod square;

fn main() {
    let test = 5;

    let mut test_ref = &test;

    println!("test_ref initial {}", test_ref);

    test_ref = &10;

    let unreferenced_test = *test_ref;

    println!("Test value {}", test);
    println!("test_ref value {}", test_ref);
    println!("unreferenced_test {}", unreferenced_test);

    test_ref = &88;

    println!("test_ref modified {}", test_ref);

    // Lets create a square!

    let square = Square::new(
        Point { x: 0, y: 2 },
        Point { x: 2, y: 2 },
        Point { x: 2, y: 0 },
        Point { x: 0, y: 0 },
    );

    println!("My square {:?} has an area of {} and a perimeter of {}",square,square.area(),square.perimeter());
}
