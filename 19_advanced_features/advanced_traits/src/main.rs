mod specifying_placeholder_types_in_trait_definitions_with_associated_types;
mod default_generic_type_parameters_and_operator_overloading;
mod fully_qualified_syntax_for_disambiguation_calling_methods_with_the_same_name;
mod using_supertraits_to_require_one_traits_functionality_within_another_trait;
mod using_the_newtype_pattern_to_implement_external_traits_on_external_types;


pub use crate::specifying_placeholder_types_in_trait_definitions_with_associated_types::advanced_traits::{IteratorTrait, Counter};
pub use crate::default_generic_type_parameters_and_operator_overloading::default_generic_parameters_op_overload::Point;
pub use crate::fully_qualified_syntax_for_disambiguation_calling_methods_with_the_same_name::fully_qualified_syntax::{Pilot, Wizard, Human, Animal, Dog};
pub use crate::using_the_newtype_pattern_to_implement_external_traits_on_external_types::external_traits_on_external_types::Wrapper;

fn main() {
    // Specifying Placeholder Types in Trait Definitions with Associated Types
    let mut counter = Counter::new();
    for i in 1..11 {
        if let Some(v) = counter.next() {
            println!("No.{} current count:{}", i, counter.cnt);
        }
    }

    // Default Generic Type Parameters and Operator Overloading
    assert_eq!(
        Point::new(1, 0) + Point::new(2, 3),
        Point::new(3, 3)
    );

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
    // 语法为：<Type as Trait>::function(receiver_if_method, next_arg, ...);
    // method为带有&self或者&mut self这些receiver的方法
    // associated function为不带receiver的方法，某种类型有多个重复名字的实现时，需要使用Fully Qualified语法指定调用
    let person = Human;
    /*
    虽然Human还实现了Pilot、Wizard的fly接口，但这里只使用自身的fly接口
     */
    person.fly(); // 等同于Human::fly(&person);
    // 要调用自身实现的其他fly接口，需要额外的调用方式
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name()); // 这里希望是调用Dog作为Animal的baby_name方法，但是实际是调用了Dog的baby_name方法
    // println!("A baby dog is called a {}", Animal::baby_name()); 编译错误，Animal是接口，不像Pilot等具有&self指针，这里编译器无法知晓是哪种实现了Animal的类型
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // 实际调用了Animal的baby_name方法，显式告诉编译器，把Dog当作Animal，调用其baby_name方法

    // Using Supertraits to Require One Trait’s Functionality Within Another Trait

    // Using the Newtype Pattern to Implement External Traits on External Types
    // 第10章提到过，只允许对某个类型实现指定的接口的时候，要么接口或者类型是local crate
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
