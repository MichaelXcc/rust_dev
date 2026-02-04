/// 希尔排序 (Shell Sort)
///
/// # 算法原理
/// 希尔排序是插入排序的改进版本，使用增量序列分组进行插入排序。
///
/// # 时间复杂度
/// - 平均: O(n^1.3) ~ O(n^1.5)
///
/// # 空间复杂度: O(1)
/// # 稳定性: 不稳定

fn shell_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp = arr[i].clone();
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap].clone();
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}

fn main() {
    println!("=== 希尔排序演示 ===\n");

    let mut numbers = vec![12, 34, 54, 2, 3, 8, 9, 1, 5, 7];
    println!("原始: {:?}", numbers);
    shell_sort(&mut numbers);
    println!("排序后: {:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut arr = vec![12, 34, 54, 2, 3];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![2, 3, 12, 34, 54]);
    }
}
