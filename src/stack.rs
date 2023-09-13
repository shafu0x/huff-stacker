#[derive(Debug)]
pub struct Stack {
    pub values: Vec<Vec<String>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn push(&mut self, value: String) {
        if self.values.len() == 0 {
            self.values.push(vec![value]);
        } else {
            let mut last_values = self.values.last().unwrap().clone();
            last_values.push(value);
            self.values.push(last_values);
        }
    }

    // pub fn pop(&mut self) -> Option<String> {
    //     if self.values.len() == 0 {
    //         return None;
    //     }

    //     let mut last_values = self.values.last().unwrap().clone();
    //     let popped = last_values.pop();
    //     self.values.push(last_values);
    //     popped
    // }

    pub fn pop2(&mut self) -> Option<String> {
        if self.values.len() == 0 {
            return None;
        }

        let mut last_values = self.values.last().unwrap().clone();
        last_values.pop();
        let popped2 = last_values.pop();
        self.values.push(last_values);
        popped2
    }

    pub fn pop_and_push(&mut self, value: String) -> Option<String> {
        if self.values.len() == 0 {
            return None;
        }

        let mut last_values = self.values.last().unwrap().clone();
        let popped = last_values.pop();
        last_values.push(value);
        self.values.push(last_values);
        popped
    }

    // pub fn peek(&self) -> Option<&String> {
    //     self.values.last()
    // }

    // pub fn len(&self) -> usize {
    //     self.values.len()
    // }
}
