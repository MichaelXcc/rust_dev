/// 快速排序 (Quick Sort)
///
/// # 算法原理
/// 快速排序使用分治策略：
/// 1. 选择一个基准元素(pivot)
/// 2. 将数组分为两部分：小于pivot和大于pivot
/// 3. 递归排序两部分
///
/// # 时间复杂度
/// - 最好/平均: O(n log n)
/// - 最坏: O(n²) - 当每次选择的pivot都是最大或最小值时
///
/// # 空间复杂度: O(log n) - 递归栈空间
/// # 稳定性: 不稳定

/// 快速排序主函数
fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    // 分区并获取pivot的最终位置
    let pivot_index = partition(arr);

    // 递归排序左右两部分
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

/// Lomuto分区方案
/// 选择最后一个元素作为pivot
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    let pivot_index = n - 1;

    // i 表示小于pivot的区域的右边界
    let mut i = 0;

    for j in 0..pivot_index {
        // 如果当前元素小于pivot，将其移到左边区域
        if arr[j] < arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    // 将pivot放到正确的位置
    arr.swap(i, pivot_index);
    i
}

/// 使用Hoare分区方案的快速排序
fn quick_sort_hoare<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition_hoare(arr);

    quick_sort_hoare(&mut arr[..=pivot_index]);
    quick_sort_hoare(&mut arr[pivot_index + 1..]);
}

/// Hoare分区方案
/// 使用双指针从两端向中间扫描
fn partition_hoare<T: Ord>(arr: &mut [T]) -> usize {
    // 选择中间元素作为pivot，避免最坏情况
    let mid = arr.len() / 2;
    arr.swap(0, mid);

    let mut i = 0;
    let mut j = arr.len() - 1;

    loop {
        // 从左边找到第一个大于等于pivot的元素
        while arr[i] < arr[0] {
            i += 1;
        }

        // 从右边找到第一个小于等于pivot的元素
        while arr[j] > arr[0] {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

/// 三路快排 - 适合处理大量重复元素
fn quick_sort_3way<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let (lt, gt) = partition_3way(arr);

    if lt > 0 {
        quick_sort_3way(&mut arr[..lt]);
    }
    if gt < arr.len() - 1 {
        quick_sort_3way(&mut arr[gt + 1..]);
    }
}

/// 三路分区：将数组分成 <pivot, =pivot, >pivot 三部分
fn partition_3way<T: Ord + Clone>(arr: &mut [T]) -> (usize, usize) {
    let pivot = arr[arr.len() / 2].clone();

    let mut lt = 0; // arr[0..lt] < pivot
    let mut i = 0; // arr[lt..i] == pivot
    let mut gt = arr.len(); // arr[gt..] > pivot

    while i < gt {
        if arr[i] < pivot {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i] > pivot {
            gt -= 1;
            arr.swap(i, gt);
        } else {
            i += 1;
        }
    }

    (lt, gt - 1)
}

fn main() {
    println!("=== 快速排序 (Quick Sort) 演示 ===\n");

    // Lomuto分区
    let mut numbers = vec![10, 7, 8, 9, 1, 5];
    println!("原始: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("Lomuto排序后: {:?}", numbers);

    println!();

    // Hoare分区
    let mut numbers2 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("原始: {:?}", numbers2);
    quick_sort_hoare(&mut numbers2);
    println!("Hoare排序后: {:?}", numbers2);

    println!();

    // 三路快排
    let mut with_dups = vec![4, 2, 4, 1, 4, 3, 4, 2];
    println!("有重复元素: {:?}", with_dups);
    quick_sort_3way(&mut with_dups);
    println!("三路快排后: {:?}", with_dups);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![10, 7, 8, 9, 1, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 5, 7, 8, 9, 10]);
    }

    #[test]
    fn test_quick_sort_hoare() {
        let mut arr = vec![10, 7, 8, 9, 1, 5];
        quick_sort_hoare(&mut arr);
        assert_eq!(arr, vec![1, 5, 7, 8, 9, 10]);
    }

    #[test]
    fn test_quick_sort_3way() {
        let mut arr = vec![4, 2, 4, 1, 4, 3];
        quick_sort_3way(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 4, 4]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}
