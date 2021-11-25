use std::borrow::BorrowMut;
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    // 相比Rc<RefCell<List>>，这里不能修改List内容比如往里面添加元素，但是可以修改所指向的List
    Nil,
}


// 引用循环将导致内存泄漏，因为计数永远不为0，导致内存无法被回收
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail()); 无限递归，将产生stack overflow

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 由于Weak<T>引用有可能已经被回收，所以在使用前要确保引用存储，调用ungrade将返回Option<Rc<T>>，检测其值是否为Some、None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()), // 弱引用
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // downgrade将获取类型为Weak<T>的智能指针，weak_count加1，而strong_count不变，另外即便weak_count降至0，也不会被回收
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    //  Visualizing Changes to strong_count and weak_count
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf), // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch),  // 1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf), // 0
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf), // 0
    );
}

// Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

// Creating a Tree Data Structure: a Node with Child Nodes
#[derive(Debug)]
struct Node {
    value: i32,
    // 这里不能使用Rc<T>，否则会出现引用循环，而子节点不能拥有父节点，但是父节点能拥有子节点，所以应该对父节点是一个弱引用
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}