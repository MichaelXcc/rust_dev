/// 冒泡排序 (Bubble Sort)
/// 
/// # 算法原理
/// 冒泡排序是一种简单的排序算法。它重复地遍历要排序的列表，
/// 比较相邻的两个元素，如果它们的顺序错误就把它们交换过来。
/// 遍历列表的工作是重复进行的，直到没有再需要交换的元素，
/// 这意味着列表已经排序完成。
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

/// 冒泡排序函数
/// 
/// # 参数
/// - `arr`: 待排序的可变数组切片
/// 
/// # 示例
/// ```
/// let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
/// ```
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    
    // 如果数组长度小于等于1，无需排序
    if n <= 1 {
        return;
    }
    
    // 外层循环：控制排序的轮数
    // 每一轮都会把当前未排序部分的最大值"冒泡"到末尾
    for i in 0..n {
        // 优化标志：如果某一轮没有发生交换，说明数组已经有序
        let mut swapped = false;
        
        // 内层循环：比较相邻元素并交换
        // 注意：n - 1 - i 是因为每轮排序后，末尾的 i 个元素已经有序
        for j in 0..n - 1 - i {
            // 如果前一个元素大于后一个元素，则交换它们
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // 如果这一轮没有发生任何交换，说明数组已经有序，提前退出
        if !swapped {
            break;
        }
    }
}

/// 主函数 - 演示冒泡排序的使用
fn main() {
    println!("=== 冒泡排序 (Bubble Sort) 演示 ===\n");
    
    // 示例1：整数数组排序
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("原始数组: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("排序后:   {:?}", numbers);
    
    println!();
    
    // 示例2：已经有序的数组（最好情况）
    let mut sorted = vec![1, 2, 3, 4, 5];
    println!("已有序数组: {:?}", sorted);
    bubble_sort(&mut sorted);
    println!("排序后:     {:?}", sorted);
    
    println!();
    
    // 示例3：逆序数组（最坏情况）
    let mut reversed = vec![5, 4, 3, 2, 1];
    println!("逆序数组: {:?}", reversed);
    bubble_sort(&mut reversed);
    println!("排序后:   {:?}", reversed);
    
    println!();
    
    // 示例4：字符串数组排序
    let mut fruits = vec!["banana", "apple", "cherry", "date"];
    println!("字符串数组: {:?}", fruits);
    bubble_sort(&mut fruits);
    println!("排序后:     {:?}", fruits);
    
    println!();
    
    // 示例5：空数组和单元素数组
    let mut empty: Vec<i32> = vec![];
    let mut single = vec![42];
    println!("空数组: {:?}", empty);
    bubble_sort(&mut empty);
    println!("排序后: {:?}", empty);
    println!("单元素数组: {:?}", single);
    bubble_sort(&mut single);
    println!("排序后:     {:?}", single);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normal_array() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    
    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
    
    #[test]
    fn test_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_reversed_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_with_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }
}
