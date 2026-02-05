/// 桶排序 (Bucket Sort)
///
/// # 算法原理
/// 桶排序将数据分到有限数量的桶中：
/// 1. 根据数据范围创建若干个桶
/// 2. 将每个元素放入对应的桶中
/// 3. 对每个非空桶进行排序
/// 4. 按顺序合并所有桶中的元素
///
/// # 时间复杂度
/// - 最好/平均: O(n + k)，k是桶的数量
/// - 最坏: O(n²)，当所有元素都在同一个桶中
///
/// # 空间复杂度: O(n + k)
/// # 稳定性: 取决于桶内排序算法

/// 桶排序 - 适用于 [0, 1) 范围的浮点数
fn bucket_sort_float(arr: &mut [f64]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // 创建n个空桶
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];

    // 将元素分配到桶中
    for &num in arr.iter() {
        // 计算桶索引：num * n
        let bucket_idx = (num * n as f64) as usize;
        // 确保索引不越界
        let idx = bucket_idx.min(n - 1);
        buckets[idx].push(num);
    }

    // 对每个桶进行排序（使用插入排序，适合小数组）
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
    }

    // 合并所有桶
    let mut index = 0;
    for bucket in buckets {
        for num in bucket {
            arr[index] = num;
            index += 1;
        }
    }
}

/// 桶排序 - 适用于整数
fn bucket_sort_int(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // 找到最大值和最小值
    let min_val = *arr.iter().min().unwrap();
    let max_val = *arr.iter().max().unwrap();

    if min_val == max_val {
        return;
    }

    // 计算桶的数量和每个桶的范围
    let bucket_count = n;
    let range = (max_val - min_val) as f64;

    // 创建桶
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); bucket_count];

    // 分配元素到桶
    for &num in arr.iter() {
        let bucket_idx = (((num - min_val) as f64 / range) * (bucket_count - 1) as f64) as usize;
        buckets[bucket_idx].push(num);
    }

    // 对每个桶排序并合并
    let mut index = 0;
    for bucket in buckets.iter_mut() {
        bucket.sort(); // 使用标准库排序
        for &num in bucket.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}

/// 插入排序（用于桶内排序）
fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    println!("=== 桶排序 (Bucket Sort) 演示 ===\n");

    // 浮点数排序 [0, 1)
    let mut floats = vec![0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68];
    println!("浮点数: {:?}", floats);
    bucket_sort_float(&mut floats);
    println!("排序后: {:?}", floats);

    println!();

    // 整数排序
    let mut integers = vec![29, 25, 3, 49, 9, 37, 21, 43];
    println!("整数: {:?}", integers);
    bucket_sort_int(&mut integers);
    println!("排序后: {:?}", integers);

    println!();

    // 展示分桶过程
    println!("--- 分桶过程演示 ---");
    let demo = vec![0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51];
    let n = demo.len();
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];

    println!("原始: {:?}", demo);
    for &num in demo.iter() {
        let idx = (num * n as f64) as usize;
        let idx = idx.min(n - 1);
        println!("  {} -> 桶{}", num, idx);
        buckets[idx].push(num);
    }

    println!("\n各桶内容:");
    for (i, bucket) in buckets.iter().enumerate() {
        if !bucket.is_empty() {
            println!("  桶{}: {:?}", i, bucket);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort_float() {
        let mut arr = vec![0.78, 0.17, 0.39, 0.26, 0.72];
        bucket_sort_float(&mut arr);
        assert_eq!(arr, vec![0.17, 0.26, 0.39, 0.72, 0.78]);
    }

    #[test]
    fn test_bucket_sort_int() {
        let mut arr = vec![29, 25, 3, 49, 9, 37, 21, 43];
        bucket_sort_int(&mut arr);
        assert_eq!(arr, vec![3, 9, 21, 25, 29, 37, 43, 49]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<f64> = vec![];
        bucket_sort_float(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![0.5];
        bucket_sort_float(&mut arr);
        assert_eq!(arr, vec![0.5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![0.5, 0.3, 0.5, 0.3, 0.5];
        bucket_sort_float(&mut arr);
        assert_eq!(arr, vec![0.3, 0.3, 0.5, 0.5, 0.5]);
    }
}
