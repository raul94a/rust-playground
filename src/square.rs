use crate::point::Point;
#[derive(Debug)]
pub struct Square {
    top_left: Point,
    top_right: Point,
    bottom_right: Point,
    bottom_left: Point,
}

impl Square {
    pub fn new(top_left: Point, top_right: Point, bottom_right: Point, bottom_left: Point) -> Square {
        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = top_right;
        let Point { x: x3, y: y3 } = bottom_right;
        let Point { x: x4, y: y4 } = bottom_left;

        return Square {
            top_left: top_left,
            top_right: top_right,
            bottom_right: bottom_right,
            bottom_left: bottom_left,
        };
    }

    fn _check_pointss(
        top_left: Point,
        top_right: Point,
        bottom_right: Point,
        bottom_left: Point,
    ) -> bool {
        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = top_right;
        let Point { x: x3, y: y3 } = bottom_right;
        let Point { x: x4, y: y4 } = bottom_left;

        let left_delta = (y1 - y4).abs();
        let right_delta = (y2 - y3).abs();
        let top_delta = (x1 - x2).abs();
        let bottom_delta = (x3 - x4).abs();

        let is_height_zero = (left_delta - right_delta) == 0;
        let is_width_zero = (top_delta - bottom_delta) == 0;

        return is_height_zero && is_width_zero;
    }

    pub fn height(&self) -> i32 {
        let Point { x: _, y: y2 } = self.top_right; // y2 para altura
        let Point { x: __, y: y3 } = self.bottom_right; // y3 para altura
        

        return (y2 - y3).abs();
    }
    pub fn width(&self) -> i32 {
        let Point { x: x1, y: _ } = self.top_left; // x1 para anchura
        let Point { x: x2, y: __ } = self.top_right; // x2 para acnchura
        

        return (x1 - x2).abs();
    }

    pub fn area(&self) -> i32 {
        return self.width() * self.height();
    }

    
    pub fn perimeter(&self) -> i32 {
        return 2 * self.height() + 2* self.width();
    }
}
