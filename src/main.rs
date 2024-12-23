use point::Point;
use square::Square;
use triangle::Triangle;
mod point;
mod square;
mod triangle;

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

    println!(
        "My square {:?} has an area of {} and a perimeter of {}",
        square,
        square.area(),
        square.perimeter()
    );

    let upper = 200;

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);

    let triangle = Triangle::new(
        Point { x: 0, y: 0 },
        Point { x: 4, y: 0 },
        Point { x: 3, y: 5 },
    );
    let hipotenusa = triangle.hypotenuse();
    println!("La hipotenusa es {}", hipotenusa);
    let ab = triangle.a();
    let bc = triangle.b();
    let ca = triangle.c();

    println!("El lado AB es {}, el lado BC es {}, y el lado CA es {}",ab,bc,ca);

    // test high order fn
    let length = triangle.high_order_function(|l| format!("La longitud es de {}",l));
    println!("length {}",length);
    
}

fn is_odd(n: u32) -> bool {
    return n % 2 == 0;
}
