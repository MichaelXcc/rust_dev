/// 基数排序 (Radix Sort)
///
/// # 算法原理
/// 基数排序是一种非比较排序算法：
/// 1. 从最低位开始，对每一位进行稳定排序（通常使用计数排序）
/// 2. 依次处理每一位，直到最高位
/// 3. 排序完成
///
/// # 时间复杂度: O(d * (n + k))
/// - d: 数字位数
/// - n: 元素个数  
/// - k: 基数（通常为10）
///
/// # 空间复杂度: O(n + k)
/// # 稳定性: 稳定

/// 基数排序 - LSD (Least Significant Digit) 版本
fn radix_sort(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }

    // 找到最大值，确定最大位数
    let max_val = *arr.iter().max().unwrap();

    // 从最低位开始，对每一位进行计数排序
    let mut exp = 1u32; // 当前处理的位数 (1, 10, 100, ...)

    while max_val / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

/// 根据指定位进行计数排序
fn counting_sort_by_digit(arr: &mut [u32], exp: u32) {
    let n = arr.len();
    let mut output = vec![0u32; n];
    let mut count = [0usize; 10]; // 十进制，0-9

    // 统计当前位的数字出现次数
    for &num in arr.iter() {
        let digit = ((num / exp) % 10) as usize;
        count[digit] += 1;
    }

    // 累加计数
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // 反向遍历，构建输出数组（保证稳定性）
    for &num in arr.iter().rev() {
        let digit = ((num / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = num;
    }

    // 复制回原数组
    arr.copy_from_slice(&output);
}

/// 基数排序 - MSD (Most Significant Digit) 版本
/// 从最高位开始，递归处理
fn radix_sort_msd(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }

    let max_val = *arr.iter().max().unwrap();
    let max_exp = get_max_exp(max_val);

    radix_sort_msd_recursive(arr, max_exp);
}

fn get_max_exp(mut num: u32) -> u32 {
    let mut exp = 1u32;
    while num >= 10 {
        num /= 10;
        exp *= 10;
    }
    exp
}

fn radix_sort_msd_recursive(arr: &mut [u32], exp: u32) {
    if arr.len() <= 1 || exp == 0 {
        return;
    }

    // 按当前位分桶
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 10];

    for &num in arr.iter() {
        let digit = ((num / exp) % 10) as usize;
        buckets[digit].push(num);
    }

    // 递归处理每个桶，然后合并
    let mut index = 0;
    for bucket in buckets.iter_mut() {
        radix_sort_msd_recursive(bucket, exp / 10);
        for &num in bucket.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}

fn main() {
    println!("=== 基数排序 (Radix Sort) 演示 ===\n");

    // LSD版本
    let mut numbers: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
    println!("原始: {:?}", numbers);
    radix_sort(&mut numbers);
    println!("LSD排序后: {:?}", numbers);

    println!();

    // MSD版本
    let mut numbers2: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
    println!("原始: {:?}", numbers2);
    radix_sort_msd(&mut numbers2);
    println!("MSD排序后: {:?}", numbers2);

    println!();

    // 展示LSD过程
    let mut demo: Vec<u32> = vec![329, 457, 657, 839, 436, 720, 355];
    println!("演示: {:?}", demo);

    let max_val = *demo.iter().max().unwrap();
    let mut exp = 1u32;
    while max_val / exp > 0 {
        counting_sort_by_digit(&mut demo, exp);
        println!("按第{}位排序: {:?}", exp, demo);
        exp *= 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_radix_sort_msd() {
        let mut arr: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort_msd(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<u32> = vec![];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr: Vec<u32> = vec![42];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_same_digits() {
        let mut arr: Vec<u32> = vec![111, 222, 111, 333, 222];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![111, 111, 222, 222, 333]);
    }
}
