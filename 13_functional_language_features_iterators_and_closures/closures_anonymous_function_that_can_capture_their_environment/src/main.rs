use std::thread;
use std::time::Duration;

/**
闭包可以通过3种方式捕获执行环境信息（如访问变量等），编译器能够自动推断
FnOnce: 对某个变量只能获取一次ownership （所有闭包都实现了该trait）
FnMut: mutable引用，能够修改变量，但不改变ownership （闭包访问的是引用，不改变ownership时会实现该trait）
Fn: immutable引用，不能修改变量，也不改变ownership （闭包访问的是immutable引用时，会实现该trait）
 */
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// origin
// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }

// Refactoring Using Functions
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = simulated_expensive_calculation(intensity);
//
//     if intensity < 25 { // 优点：相比上一版本避免调用两次simulated_expensive_calculation，节省不必要的开销
//         println!("Today, do {} pushups!", expensive_result);
//         println!("Next, do {} situps!", expensive_result);
//     } else {
//         if random_number == 3 { // 缺点：这个分支不会用到simulated_expensive_calculation，但依旧执行了该高开销的函数
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_result);
//         }
//     }
// }

// Refactoring with Closures to Store Code
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| { // 多个参数，则|params1,params2|
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    /*
     let expensive_closure = |num: u32| -> u32 { // 闭包具有类型推断，可以不用对参数及返回类型注明类型
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
     */

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

// Storing Closures Using Generic Parameters and the Fn Traits
struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn generate_workout_with_closures_cache(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    /*
    let example_closure = |x| x;

    let s = example_closure(String::from("hello")); 编译器在闭包第一次调用时推断出参数类型为String
    let n = example_closure(5);  第二次使用闭包时，如果参数类型不为String，则编译报错
     */

    // Capturing the Environment with Closures
    let x = 4;

    let equal_to_x = |z| z == x; // 闭包可以访问上下文信息，但是函数不可以，下面会出现编译错误
    // fn equal_to_x_function(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;

    assert!(equal_to_x(y));

    // assert!(equal_to_x_function(y));

    let x = vec![1, 2, 3];

    let equal_to_x_with_move = move |z| z == x; // 在没有实现Copy trait的时候，赋值语句将使得x被moved到闭包内部，ownership被修改，后续将不能被访问

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    // assert!(equal_to_x_with_move(y))  这里将产生编译错误
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2); // 上一行代码已经缓存了值

    assert_eq!(v2, 2);
}