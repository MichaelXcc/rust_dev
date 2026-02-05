/// 计数排序 (Counting Sort)
///
/// # 算法原理
/// 计数排序是一种非比较排序算法：
/// 1. 找出待排序数组中的最大值和最小值
/// 2. 创建计数数组，统计每个值出现的次数
/// 3. 累加计数，得到每个值在输出数组中的位置
/// 4. 反向遍历原数组，将元素放到正确位置
///
/// # 时间复杂度: O(n + k)，其中k是数据范围
/// # 空间复杂度: O(n + k)
/// # 稳定性: 稳定（使用反向遍历时）
///
/// # 适用场景: 数据范围不大的整数排序

/// 计数排序 - 适用于非负整数
fn counting_sort(arr: &mut [usize]) {
    if arr.len() <= 1 {
        return;
    }

    // 找到最大值
    let max_val = *arr.iter().max().unwrap();

    // 创建计数数组
    let mut count = vec![0usize; max_val + 1];

    // 统计每个值的出现次数
    for &num in arr.iter() {
        count[num] += 1;
    }

    // 将排序后的值写回原数组
    let mut index = 0;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            arr[index] = val;
            index += 1;
        }
    }
}

/// 稳定版计数排序 - 适用于有符号整数
fn counting_sort_stable(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    // 找到最大值和最小值
    let min_val = *arr.iter().min().unwrap();
    let max_val = *arr.iter().max().unwrap();

    // 计算偏移量和范围
    let range = (max_val - min_val + 1) as usize;

    // 创建计数数组
    let mut count = vec![0usize; range];

    // 统计每个值的出现次数
    for &num in arr.iter() {
        count[(num - min_val) as usize] += 1;
    }

    // 累加计数，得到每个值的结束位置
    for i in 1..range {
        count[i] += count[i - 1];
    }

    // 创建输出数组，反向遍历保证稳定性
    let mut output = vec![0i32; arr.len()];
    for &num in arr.iter().rev() {
        let idx = (num - min_val) as usize;
        count[idx] -= 1;
        output[count[idx]] = num;
    }

    // 复制回原数组
    arr.copy_from_slice(&output);
}

fn main() {
    println!("=== 计数排序 (Counting Sort) 演示 ===\n");

    // 非负整数排序
    let mut numbers: Vec<usize> = vec![4, 2, 2, 8, 3, 3, 1];
    println!("原始: {:?}", numbers);
    counting_sort(&mut numbers);
    println!("排序后: {:?}", numbers);

    println!();

    // 有符号整数排序
    let mut signed = vec![-5, 3, -1, 0, 3, -5, 2, 1];
    println!("有符号: {:?}", signed);
    counting_sort_stable(&mut signed);
    println!("排序后: {:?}", signed);

    println!();

    // 展示计数过程
    let mut demo: Vec<usize> = vec![1, 4, 1, 2, 7, 5, 2];
    println!("演示数组: {:?}", demo);

    let max_val = *demo.iter().max().unwrap();
    let mut count = vec![0usize; max_val + 1];
    for &num in demo.iter() {
        count[num] += 1;
    }
    println!("计数数组: {:?}", count);

    counting_sort(&mut demo);
    println!("排序结果: {:?}", demo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr: Vec<usize> = vec![4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_counting_sort_stable() {
        let mut arr = vec![-5, 3, -1, 0, 3, -5, 2];
        counting_sort_stable(&mut arr);
        assert_eq!(arr, vec![-5, -5, -1, 0, 2, 3, 3]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<usize> = vec![];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr: Vec<usize> = vec![42];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_all_same() {
        let mut arr: Vec<usize> = vec![5, 5, 5, 5];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![5, 5, 5, 5]);
    }
}
