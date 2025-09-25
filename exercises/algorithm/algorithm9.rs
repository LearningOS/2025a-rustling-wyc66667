use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引0 unused，从1开始存储元素
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // 添加元素到堆
    pub fn add(&mut self, value: T) {
        self.count += 1;
        // 如果容量不足则扩展
        if self.count >= self.items.len() {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }
        
        // 上浮操作：将新元素放到正确位置
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 使用比较器判断是否需要交换
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break; // 满足堆性质，停止上浮
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 找到符合比较器条件的子节点索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // 只有左子节点
        if right_idx > self.count {
            return left_idx;
        }
        
        // 比较左右子节点，返回符合比较器条件的那个
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建最小堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建最大堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

// 实现迭代器，用于弹出堆顶元素
impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }
        
        // 堆顶元素是索引1的元素
        let top = self.items.swap_remove(1);
        self.count -= 1;
        
        // 如果还有元素，需要下沉操作维持堆性质
        if !self.is_empty() {
            // 将最后一个元素移到堆顶
            if self.count + 1 < self.items.len() {
                self.items.swap(1, self.count + 1);
            }
            
            // 下沉操作
            let mut idx = 1;
            while self.children_present(idx) {
                let child_idx = self.smallest_child_idx(idx);
                // 使用比较器判断是否需要交换
                if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                    self.items.swap(idx, child_idx);
                    idx = child_idx;
                } else {
                    break; // 满足堆性质，停止下沉
                }
            }
        }
        
        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}