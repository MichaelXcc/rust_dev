# Rust 排序算法集合

本目录包含了常见排序算法的 Rust 实现，每个文件都可以独立运行。

## 包含的排序算法

| 文件                | 算法     | 时间复杂度  | 空间复杂度 | 稳定性    |
| ------------------- | -------- | ----------- | ---------- | --------- |
| `bubble_sort.rs`    | 冒泡排序 | O(n²)       | O(1)       | ✅ 稳定   |
| `selection_sort.rs` | 选择排序 | O(n²)       | O(1)       | ❌ 不稳定 |
| `insertion_sort.rs` | 插入排序 | O(n²)       | O(1)       | ✅ 稳定   |
| `shell_sort.rs`     | 希尔排序 | O(n^1.3)    | O(1)       | ❌ 不稳定 |
| `merge_sort.rs`     | 归并排序 | O(n log n)  | O(n)       | ✅ 稳定   |
| `quick_sort.rs`     | 快速排序 | O(n log n)  | O(log n)   | ❌ 不稳定 |
| `heap_sort.rs`      | 堆排序   | O(n log n)  | O(1)       | ❌ 不稳定 |
| `counting_sort.rs`  | 计数排序 | O(n + k)    | O(n + k)   | ✅ 稳定   |
| `radix_sort.rs`     | 基数排序 | O(d(n + k)) | O(n + k)   | ✅ 稳定   |
| `bucket_sort.rs`    | 桶排序   | O(n + k)    | O(n + k)   | ✅ 稳定   |

## 如何运行

每个文件都是独立的，可以直接使用 `rustc` 编译运行：

```bash
# 编译并运行冒泡排序
rustc bubble_sort.rs -o bubble_sort && ./bubble_sort

# 编译并运行快速排序
rustc quick_sort.rs -o quick_sort && ./quick_sort

# 编译并运行所有排序算法
for file in *.rs; do
    name="${file%.rs}"
    echo "=== 运行 $name ==="
    rustc "$file" -o "$name" && ./"$name"
    echo ""
done
```

## 运行测试

每个文件都包含单元测试，可以使用以下命令运行：

```bash
# 运行单个文件的测试
rustc --test bubble_sort.rs -o bubble_sort_test && ./bubble_sort_test

# 或者使用 cargo（需要在 Cargo 项目中）
cargo test
```

## 算法选择指南

- **小规模数据** (n < 50): 插入排序
- **基本有序数据**: 插入排序、冒泡排序
- **一般情况**: 快速排序、归并排序
- **内存受限**: 堆排序、快速排序
- **稳定性要求**: 归并排序、插入排序
- **整数范围小**: 计数排序、基数排序
- **均匀分布数据**: 桶排序

## 学习建议

1. 先理解比较排序（冒泡、选择、插入）
2. 学习分治思想（归并、快速）
3. 掌握堆数据结构（堆排序）
4. 了解非比较排序（计数、基数、桶）
