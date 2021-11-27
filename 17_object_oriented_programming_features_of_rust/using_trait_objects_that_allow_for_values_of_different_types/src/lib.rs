// Defining a Trait for Common Behavior
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    /*
    vector里面的元素都是在Box里面实现了trait的类型
    这里不同于泛型，运行时只允许一种具体类型；而使用多态，能够在运行时有不同的类型
    pub struct ScreenGenericType<T>
    where T: Draw
    {
        pub components: Vec<T>,
    }

    impl<T> ScreenGenericType<T>
        where
            T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
     */
    pub components: Vec<Box<dyn Draw>>,

    /*
    Object Safety Is Required for Trait Objects
    You can only make object-safe traits into trait objects.
    Some complex rules govern all the properties that make a trait object safe, but in practice, only two rules are relevant.
    A trait is object safe if all the methods defined in the trait have the following properties:
        1.The return type isn’t Self.
        2.There are no generic type parameters.

    pub trait Clone {  Clone不是Object Safety
    fn clone(&self) -> Self;
    }
    所以下面无法用于Trait Objects
    pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
     */
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing the Trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}