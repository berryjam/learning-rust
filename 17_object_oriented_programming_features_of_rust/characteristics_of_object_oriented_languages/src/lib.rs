pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Encapsulation that Hides Implementation Details 封装，只对外暴露必要的接口，不直接暴露数据，后续实际使用的数据结构即使发生变更，只要方法签名不变，那么调用代码也不需要变更
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
