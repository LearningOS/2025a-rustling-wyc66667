/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T>(array: &mut [T])
	//TODO
where
    T: Ord, // 填补：添加类型约束，要求元素可比较
{
    // 填补开始：冒泡排序实现
    let n = array.len();
    // 外层循环控制排序轮次
    for i in 0..n {
        // 内层循环进行相邻元素比较和交换
        // 每轮结束后，最大的元素会"冒泡"到末尾，因此可以减少比较次数
        for j in 0..n - i - 1 {
            // 比较相邻元素，如果前一个大于后一个则交换
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}