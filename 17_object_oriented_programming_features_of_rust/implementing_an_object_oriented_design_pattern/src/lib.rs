// Defining Post and Creating a New Instance in the Draft State
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Storing the Text of the Post Content
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Ensuring the Content of a Draft Post Is Empty
    pub fn content(&self) -> &str {
        /*
        这里对Option使用as_ref方法是借用，返回Option<&Box<dyn State>>，不像take一样会修改ownership
        然后unwrap返回&Box<dyn State>，再调用content会自动解引用，调用State的content方法
         */
        self.state.as_ref().unwrap().content(self)
    }

    // Requesting a Review of the Post Changes Its State
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    /*
    这里方法第一个参数不同于self,&self,&mut self，使用Box<Self>，这里意味着调用的时候只能是Box<dyn State>类型
    并且会更改Box<Self>的ownership
     */
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // Adding the approve Method that Changes the Behavior of content
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /*
    默认实现，这里需要用到lifetime注解参数，是因为函数有一个引用类型的参数，并且函数返回值也涉及到这个引用参数
    需要显式高速编译器其引用参数的生命周期，否则会编译出错
     */
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str { // 覆盖默认实现
        &post.content
    }
}

