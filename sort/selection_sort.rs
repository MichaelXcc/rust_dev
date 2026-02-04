/// 选择排序 (Selection Sort)
///
/// # 算法原理
/// 选择排序是一种简单直观的排序算法。它的工作原理是：
/// 1. 首先在未排序序列中找到最小（大）元素，存放到排序序列的起始位置
/// 2. 然后，再从剩余未排序元素中继续寻找最小（大）元素
/// 3. 放到已排序序列的末尾
/// 4. 重复第二步，直到所有元素均排序完毕
///
/// # 时间复杂度
/// - 最好情况: O(n²)
/// - 最坏情况: O(n²)
/// - 平均情况: O(n²)
/// 注意：无论初始状态如何，选择排序的比较次数都是相同的
///
/// # 空间复杂度
/// O(1) - 只需要常数级别的额外空间
///
/// # 稳定性
/// 不稳定排序 - 相等元素的相对顺序可能会改变
/// 例如：[5a, 8, 5b, 2] 排序后可能变成 [2, 5b, 5a, 8]

/// 选择排序函数
///
/// # 参数
/// - `arr`: 待排序的可变数组切片
///
/// # 示例
/// ```
/// let mut arr = vec![64, 25, 12, 22, 11];
/// selection_sort(&mut arr);
/// assert_eq!(arr, vec![11, 12, 22, 25, 64]);
/// ```
fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // 如果数组长度小于等于1，无需排序
    if n <= 1 {
        return;
    }

    // 外层循环：遍历每一个位置
    // 每次迭代确定第 i 个位置应该放置的元素
    for i in 0..n - 1 {
        // 假设当前位置的元素是最小的
        let mut min_index = i;

        // 内层循环：在未排序部分中找到最小元素的索引
        for j in (i + 1)..n {
            // 如果找到更小的元素，更新最小元素的索引
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // 如果最小元素不在当前位置，则交换
        // 这个判断可以减少不必要的交换操作
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

/// 双向选择排序（优化版本）
/// 每次遍历同时找到最小值和最大值，分别放到两端
/// 可以将遍历次数减少一半
fn selection_sort_bidirectional<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    if n <= 1 {
        return;
    }

    let mut left = 0; // 左边界（已排序的最小值部分）
    let mut right = n - 1; // 右边界（已排序的最大值部分）

    while left < right {
        let mut min_index = left;
        let mut max_index = left;

        // 在未排序区间 [left, right] 中同时找最小值和最大值
        for i in left..=right {
            if arr[i] < arr[min_index] {
                min_index = i;
            }
            if arr[i] > arr[max_index] {
                max_index = i;
            }
        }

        // 将最小值交换到左边
        if min_index != left {
            arr.swap(left, min_index);
        }

        // 注意：如果最大值在 left 位置，经过上面的交换后，
        // 最大值已经被移动到 min_index 位置
        if max_index == left {
            max_index = min_index;
        }

        // 将最大值交换到右边
        if max_index != right {
            arr.swap(right, max_index);
        }

        // 缩小未排序区间
        left += 1;
        right -= 1;
    }
}

/// 主函数 - 演示选择排序的使用
fn main() {
    println!("=== 选择排序 (Selection Sort) 演示 ===\n");

    // 示例1：整数数组排序
    let mut numbers = vec![64, 25, 12, 22, 11];
    println!("原始数组: {:?}", numbers);
    selection_sort(&mut numbers);
    println!("排序后:   {:?}", numbers);

    println!();

    // 示例2：使用双向选择排序
    let mut numbers2 = vec![29, 10, 14, 37, 13, 8, 25];
    println!("原始数组: {:?}", numbers2);
    selection_sort_bidirectional(&mut numbers2);
    println!("双向选择排序后: {:?}", numbers2);

    println!();

    // 示例3：带有重复元素的数组
    let mut with_duplicates = vec![4, 2, 4, 1, 3, 2, 1];
    println!("带重复元素: {:?}", with_duplicates);
    selection_sort(&mut with_duplicates);
    println!("排序后:     {:?}", with_duplicates);

    println!();

    // 示例4：字符排序
    let mut chars = vec!['d', 'b', 'a', 'c', 'e'];
    println!("字符数组: {:?}", chars);
    selection_sort(&mut chars);
    println!("排序后:   {:?}", chars);

    println!();

    // 示例5：边界情况
    let mut empty: Vec<i32> = vec![];
    let mut single = vec![42];
    let mut two = vec![2, 1];

    println!("空数组: {:?}", empty);
    selection_sort(&mut empty);
    println!("排序后: {:?}", empty);

    println!("单元素: {:?}", single);
    selection_sort(&mut single);
    println!("排序后: {:?}", single);

    println!("两元素: {:?}", two);
    selection_sort(&mut two);
    println!("排序后: {:?}", two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_array() {
        let mut arr = vec![64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reversed_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bidirectional() {
        let mut arr = vec![29, 10, 14, 37, 13, 8, 25];
        selection_sort_bidirectional(&mut arr);
        assert_eq!(arr, vec![8, 10, 13, 14, 25, 29, 37]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }
}
