use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

/*

trait Add<Rhs=Self> { // 语法为<PlaceholderType=ConcreteType>，这里是<Rhs=Self>，即自己为默认关联类型
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
 */

impl Add for Point { // 这里不需要指定泛型参数，即使用默认泛型参数，为Point
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { // 非默认参数，即
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}