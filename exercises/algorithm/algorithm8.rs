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

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
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

// 使用两个队列实现栈
pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // 入栈：将元素加入非空队列（如果都为空，加入q1）
    pub fn push(&mut self, elem: T) {
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    // 出栈：将非空队列的前n-1个元素移动到另一个队列，然后弹出最后一个元素
    pub fn pop(&mut self) -> Result<T, &str> {
        // 确定哪个队列非空
        let (source, target) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else if !self.q2.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            // 两个队列都为空
            return Err("Stack is empty");
        };

        // 将source中除最后一个元素外的所有元素移动到target
        while source.size() > 1 {
            let elem = source.dequeue().unwrap();
            target.enqueue(elem);
        }

        // 弹出source中剩下的最后一个元素（栈顶元素）
        source.dequeue()
    }

    // 检查栈是否为空（两个队列都为空）
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::new();
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