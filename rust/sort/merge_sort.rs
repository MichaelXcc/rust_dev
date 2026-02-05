/// 归并排序 (Merge Sort)
///
/// # 算法原理
/// 归并排序是分治法的典型应用：
/// 1. 将数组分成两半
/// 2. 递归排序两半
/// 3. 合并两个有序数组
///
/// # 时间复杂度: O(n log n) - 所有情况
/// # 空间复杂度: O(n) - 需要额外空间存储合并结果
/// # 稳定性: 稳定排序

/// 归并排序主函数
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // 分割点
    let mid = n / 2;

    // 递归排序左右两半
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    // 合并两个有序部分
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    merge(arr, &left, &right);
}

/// 合并两个有序数组
fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0; // 左数组索引
    let mut j = 0; // 右数组索引
    let mut k = 0; // 结果数组索引

    // 比较并合并
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    // 复制剩余元素
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// 自底向上的归并排序（迭代版本）
fn merge_sort_iterative<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut size = 1; // 当前合并的子数组大小

    while size < n {
        let mut left = 0;

        while left < n - size {
            let mid = left + size;
            let right = std::cmp::min(left + 2 * size, n);

            // 合并 [left, mid) 和 [mid, right)
            let left_arr = arr[left..mid].to_vec();
            let right_arr = arr[mid..right].to_vec();
            merge(&mut arr[left..right], &left_arr, &right_arr);

            left += 2 * size;
        }

        size *= 2;
    }
}

fn main() {
    println!("=== 归并排序 (Merge Sort) 演示 ===\n");

    // 递归版本
    let mut numbers = vec![38, 27, 43, 3, 9, 82, 10];
    println!("原始数组: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("递归排序后: {:?}", numbers);

    println!();

    // 迭代版本
    let mut numbers2 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("原始数组: {:?}", numbers2);
    merge_sort_iterative(&mut numbers2);
    println!("迭代排序后: {:?}", numbers2);

    println!();

    // 字符串排序
    let mut words = vec!["dog", "cat", "bird", "apple", "zebra"];
    println!("字符串: {:?}", words);
    merge_sort(&mut words);
    println!("排序后: {:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn test_merge_sort_iterative() {
        let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
        merge_sort_iterative(&mut arr);
        assert_eq!(arr, vec![3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
