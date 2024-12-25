    use crate::{perimeter::HasPerimeter, point::Point};

    use rand::{Rng};

    pub struct Triangle {
        a: Point,
        b: Point,
        c: Point,
    }

    impl HasPerimeter for Triangle {
        fn perimeter(&self) -> f64 {
            return (self.a() + self.b() + self.c()).into()
        }
    }

    impl Triangle {
        pub fn new(a: Point, b: Point, c: Point) -> Triangle {
            return Triangle { a, b, c };
        }
        // Defined by /AB
        pub fn a(&self) -> f32 {
            return self._module(&self.a, &self.b);
        }
        // Defined by /BC
        pub fn b(&self) -> f32 {
            return self._module(&self.b, &self.c);
        }

        // Defined by /CA
        pub fn c(&self) -> f32 {
            return self._module(&self.c, &self.a);
        }

        pub fn hypotenuse(&self) -> f32 {
            let ab = self.a();
            let bc = self.b();
            let ca = self.c();

            let mut vector = vec![ab, bc, ca];
            vector.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
            return vector[0];
        }

        fn _module(&self, a: &Point, b: &Point) -> f32 {
            let Point { x: x1, y: y1 } = a;
            let Point { x: x2, y: y2 } = b;
            let a2 = (x1 - x2).abs();
            let b2 = (y1 - y2).abs();

            return (((b2 * b2) as f32) + ((a2 * a2) as f32)).sqrt();
        }

        pub fn high_order_function<F: Fn(i32) -> String>(&self, func: F) -> i32{
            
            let mut len = 0;
            let mut rng = rand::thread_rng();
            let random_number: i32 = rng.gen_range(1..80001);

            len = func(random_number).len();

            println!("El número aleatorio ha sido {}",random_number);
            
            return len as i32;

        }
    }
