/// 堆排序 (Heap Sort)
///
/// # 算法原理
/// 堆排序利用堆这种数据结构进行排序：
/// 1. 将数组构建成最大堆
/// 2. 将堆顶（最大值）与末尾元素交换
/// 3. 缩小堆的范围，重新调整堆
/// 4. 重复步骤2-3直到堆大小为1
///
/// # 时间复杂度: O(n log n) - 所有情况
/// # 空间复杂度: O(1) - 原地排序
/// # 稳定性: 不稳定

/// 堆排序主函数
fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // 第一步：构建最大堆
    // 从最后一个非叶子节点开始，自底向上调整
    // 最后一个非叶子节点的索引是 n/2 - 1
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // 第二步：逐个取出堆顶元素
    for i in (1..n).rev() {
        // 将堆顶（最大值）与当前末尾交换
        arr.swap(0, i);
        // 对剩余元素重新调整堆
        heapify(arr, i, 0);
    }
}

/// 堆调整函数（下沉操作）
/// 确保以 root 为根的子树满足最大堆性质
///
/// # 参数
/// - `arr`: 数组
/// - `heap_size`: 堆的大小（数组中参与堆操作的元素数量）
/// - `root`: 需要调整的根节点索引
fn heapify<T: Ord>(arr: &mut [T], heap_size: usize, root: usize) {
    let mut largest = root; // 假设根节点最大
    let left = 2 * root + 1; // 左子节点
    let right = 2 * root + 2; // 右子节点

    // 如果左子节点比根节点大
    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }

    // 如果右子节点比当前最大的还大
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    // 如果最大的不是根节点，需要交换并继续调整
    if largest != root {
        arr.swap(root, largest);
        // 递归调整被影响的子树
        heapify(arr, heap_size, largest);
    }
}

/// 迭代版本的堆调整（避免递归栈溢出）
fn heapify_iterative<T: Ord>(arr: &mut [T], heap_size: usize, mut root: usize) {
    loop {
        let mut largest = root;
        let left = 2 * root + 1;
        let right = 2 * root + 2;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }

        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == root {
            break;
        }

        arr.swap(root, largest);
        root = largest;
    }
}

/// 使用迭代版本的堆排序
fn heap_sort_iterative<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    for i in (0..n / 2).rev() {
        heapify_iterative(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify_iterative(arr, i, 0);
    }
}

fn main() {
    println!("=== 堆排序 (Heap Sort) 演示 ===\n");

    let mut numbers = vec![12, 11, 13, 5, 6, 7];
    println!("原始: {:?}", numbers);
    heap_sort(&mut numbers);
    println!("排序后: {:?}", numbers);

    println!();

    // 展示堆构建过程
    let mut demo = vec![4, 10, 3, 5, 1];
    println!("堆构建演示:");
    println!("初始: {:?}", demo);

    let n = demo.len();
    for i in (0..n / 2).rev() {
        heapify(&mut demo, n, i);
        println!("调整节点{}: {:?}", i, demo);
    }
    println!("最大堆: {:?}", demo);

    println!();

    // 迭代版本
    let mut numbers2 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("原始: {:?}", numbers2);
    heap_sort_iterative(&mut numbers2);
    println!("迭代版排序后: {:?}", numbers2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![12, 11, 13, 5, 6, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn test_heap_sort_iterative() {
        let mut arr = vec![12, 11, 13, 5, 6, 7];
        heap_sort_iterative(&mut arr);
        assert_eq!(arr, vec![5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
