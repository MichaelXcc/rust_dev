/// 插入排序 (Insertion Sort)
///
/// # 算法原理
/// 插入排序的工作方式就像我们整理扑克牌一样：
/// 1. 从第一个元素开始，该元素可以认为已经被排序
/// 2. 取出下一个元素，在已经排序的元素序列中从后向前扫描
/// 3. 如果已排序的元素大于新元素，将该元素移到下一位置
/// 4. 重复步骤3，直到找到已排序的元素小于或等于新元素的位置
/// 5. 将新元素插入到该位置后
/// 6. 重复步骤2~5，直到所有元素都被排序
///
/// # 时间复杂度
/// - 最好情况: O(n) - 当数组已经有序时
/// - 最坏情况: O(n²) - 当数组逆序时
/// - 平均情况: O(n²)
///
/// # 空间复杂度
/// O(1) - 只需要常数级别的额外空间
///
/// # 稳定性
/// 稳定排序 - 相等元素的相对顺序不会改变
///
/// # 适用场景
/// - 小规模数据
/// - 数据基本有序的情况
/// - 作为其他高级排序算法的子过程（如快速排序的小数组优化）

/// 插入排序函数（移动版本）
/// 使用元素移动而非交换，效率更高
///
/// # 参数
/// - `arr`: 待排序的可变数组切片
fn insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    // 如果数组长度小于等于1，无需排序
    if n <= 1 {
        return;
    }

    // 从第二个元素开始，因为第一个元素自己就是有序的
    for i in 1..n {
        // 保存当前要插入的元素
        let key = arr[i].clone();

        // j 用于查找插入位置，从当前位置向前扫描
        let mut j = i;

        // 将比 key 大的元素向后移动
        // 注意：j > 0 确保不会越界
        while j > 0 && arr[j - 1] > key {
            // 使用 clone 来移动元素（Rust 需要显式处理所有权）
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }

        // 将 key 插入到正确的位置
        arr[j] = key;
    }
}

/// 插入排序函数（交换版本）
/// 使用相邻元素交换，代码更简洁但效率稍低
fn insertion_sort_swap<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    if n <= 1 {
        return;
    }

    for i in 1..n {
        // 从当前位置向前，通过相邻交换将元素移动到正确位置
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// 二分插入排序
/// 使用二分查找来确定插入位置，减少比较次数
/// 但移动次数仍然是 O(n²)
fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    if n <= 1 {
        return;
    }

    for i in 1..n {
        let key = arr[i].clone();

        // 使用二分查找在已排序部分 [0, i) 中找到插入位置
        // 找到第一个大于 key 的元素位置
        let insert_pos = binary_search_insert_position(&arr[..i], &key);

        // 将 [insert_pos, i) 范围内的元素向后移动一位
        // 使用 rotate_right 可以高效地完成这个操作
        arr[insert_pos..=i].rotate_right(1);
    }
}

/// 二分查找插入位置
/// 返回第一个大于 target 的元素的索引
fn binary_search_insert_position<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        // 使用 <= 保证稳定性
        // 当遇到相等元素时，继续在右半部分查找
        if arr[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

/// 主函数 - 演示插入排序的使用
fn main() {
    println!("=== 插入排序 (Insertion Sort) 演示 ===\n");

    // 示例1：整数数组排序（移动版本）
    let mut numbers = vec![12, 11, 13, 5, 6];
    println!("原始数组: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("插入排序后: {:?}", numbers);

    println!();

    // 示例2：使用交换版本
    let mut numbers2 = vec![31, 41, 59, 26, 41, 58];
    println!("原始数组: {:?}", numbers2);
    insertion_sort_swap(&mut numbers2);
    println!("交换版本排序后: {:?}", numbers2);

    println!();

    // 示例3：使用二分插入排序
    let mut numbers3 = vec![37, 23, 0, 17, 12, 72, 31];
    println!("原始数组: {:?}", numbers3);
    binary_insertion_sort(&mut numbers3);
    println!("二分插入排序后: {:?}", numbers3);

    println!();

    // 示例4：已经有序的数组（最好情况）
    let mut sorted = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("已有序数组: {:?}", sorted);
    insertion_sort(&mut sorted);
    println!("排序后: {:?}", sorted);

    println!();

    // 示例5：逆序数组（最坏情况）
    let mut reversed = vec![8, 7, 6, 5, 4, 3, 2, 1];
    println!("逆序数组: {:?}", reversed);
    insertion_sort(&mut reversed);
    println!("排序后: {:?}", reversed);

    println!();

    // 示例6：字符串排序
    let mut words = vec!["dog", "cat", "bird", "apple"];
    println!("字符串数组: {:?}", words);
    insertion_sort(&mut words);
    println!("排序后: {:?}", words);

    println!();

    // 示例7：展示插入排序的过程
    println!("--- 展示排序过程 ---");
    let mut demo = vec![5, 2, 4, 6, 1, 3];
    println!("初始: {:?}", demo);
    for i in 1..demo.len() {
        let key = demo[i];
        let mut j = i;
        while j > 0 && demo[j - 1] > demo[j] {
            demo.swap(j - 1, j);
            j -= 1;
        }
        println!("第{}轮 (插入{}): {:?}", i, key, demo);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![12, 11, 13, 5, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![5, 6, 11, 12, 13]);
    }

    #[test]
    fn test_insertion_sort_swap() {
        let mut arr = vec![12, 11, 13, 5, 6];
        insertion_sort_swap(&mut arr);
        assert_eq!(arr, vec![5, 6, 11, 12, 13]);
    }

    #[test]
    fn test_binary_insertion_sort() {
        let mut arr = vec![12, 11, 13, 5, 6];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, vec![5, 6, 11, 12, 13]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reversed_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_stability() {
        // 测试稳定性：相等元素应保持原有顺序
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Item {
            key: i32,
            index: usize,
        }

        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Item {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.key.cmp(&other.key)
            }
        }

        let mut arr = vec![
            Item { key: 3, index: 0 },
            Item { key: 1, index: 1 },
            Item { key: 3, index: 2 },
            Item { key: 1, index: 3 },
        ];

        insertion_sort(&mut arr);

        // 验证相等元素保持原有的相对顺序
        assert!(arr[0].key == 1 && arr[0].index == 1);
        assert!(arr[1].key == 1 && arr[1].index == 3);
        assert!(arr[2].key == 3 && arr[2].index == 0);
        assert!(arr[3].key == 3 && arr[3].index == 2);
    }
}
