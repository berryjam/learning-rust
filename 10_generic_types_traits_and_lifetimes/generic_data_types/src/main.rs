// 范型参数需定义在函数名与参数列表之间，而后参数列表及返回值才能使用该类型
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest { // 此处会编译错误，实际要限定范型类型，这里不能用>，要实现std::cmp::PartialOrd trait
//             largest = item;
//         }
//     }
//
//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

// In Method Definitions，泛型方法，需要在impl之后声明
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只作用在f32的Point类型上
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数，支持多种类型的值
struct MultiTypePoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MultiTypePoint<T, U> {
    fn mixup<V, W>(self, other: MultiTypePoint<V, W>) -> MultiTypePoint<T, W> {
        MultiTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    //
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = MultiTypePoint { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = MultiTypePoint { x: 5, y: 10.4 };
    let p2 = MultiTypePoint { x: "Hello", y: 'c' };

    // Performance of Code Using Generics Rust实际在编译时，会基于后续泛型的实际参数展开具体定义，因此运行时不会有额外的开销导致性能下降
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
