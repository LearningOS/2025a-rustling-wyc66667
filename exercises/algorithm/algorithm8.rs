/*
    queue
    This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &'static str> {  // 使用'static生命周期
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &'static str> {  // 使用'static生命周期
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// 修复命名规范：使用UpperCamelCase
pub struct MyStack<T>
{
    q1: Queue<T>,
    q2: Queue<T>
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new()
        }
    }
    
    pub fn push(&mut self, elem: T) {
        if self.q1.is_empty() {
            self.q2.enqueue(elem);
        } else {
            self.q1.enqueue(elem);
        }
    }
    
    // 修复借用冲突：避免同时借用self的多个部分
    pub fn pop(&mut self) -> Result<T, &'static str> {  // 使用'static生命周期
        // 检查栈是否为空
        if self.q1.is_empty() && self.q2.is_empty() {
            return Err("Stack is empty");
        }
        
        // 移动元素并获取结果
        if !self.q1.is_empty() {
            // 先移动元素，再获取结果，避免同时借用
            Self::move_elements(&mut self.q1, &mut self.q2)?;
            self.q1.dequeue()
        } else {
            Self::move_elements(&mut self.q2, &mut self.q1)?;
            self.q2.dequeue()
        }
    }
    
    // 辅助方法：设为静态方法避免self的额外借用
    fn move_elements(from: &mut Queue<T>, to: &mut Queue<T>) -> Result<(), &'static str> {
        while from.size() > 1 {
            let val = from.dequeue()?;
            to.enqueue(val);
        }
        Ok(())
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_queue(){
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}