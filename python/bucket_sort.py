#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
桶排序 (Bucket Sort)

桶排序是一种分布式排序算法

算法原理：
1. 设置一定数量的桶（空桶）
2. 遍历数组，将每个元素放入对应的桶中
3. 对每个非空桶进行排序
4. 按顺序遍历所有桶，将元素放回原数组

适用场景：
- 数据均匀分布
- 数据在一个可预知的范围内

时间复杂度：
- 平均：O(n + k)
- 最坏：O(n²)（所有元素落入同一个桶）
空间复杂度：O(n + k)
稳定性：取决于桶内排序算法
"""

from typing import List


def bucket_sort(arr: List[float]) -> List[float]:
    """
    桶排序（适用于 [0, 1) 范围的浮点数）
    
    Args:
        arr: 待排序的浮点数列表
        
    Returns:
        排序后的列表
    """
    if len(arr) <= 1:
        return arr
    
    n = len(arr)
    
    # 创建 n 个空桶
    buckets = [[] for _ in range(n)]
    
    # 将元素分配到桶中
    for num in arr:
        idx = int(num * n)
        if idx >= n:
            idx = n - 1
        buckets[idx].append(num)
    
    # 对每个桶内的元素排序
    for bucket in buckets:
        bucket.sort()
    
    # 合并所有桶
    result = []
    for bucket in buckets:
        result.extend(bucket)
    
    arr[:] = result
    return arr


def bucket_sort_int(arr: List[int], bucket_size: int = 10) -> List[int]:
    """
    整数桶排序
    
    Args:
        arr: 待排序的整数列表
        bucket_size: 每个桶的范围大小
        
    Returns:
        排序后的列表
    """
    if len(arr) <= 1:
        return arr
    
    min_val = min(arr)
    max_val = max(arr)
    
    # 计算桶的数量
    bucket_count = (max_val - min_val) // bucket_size + 1
    buckets = [[] for _ in range(bucket_count)]
    
    # 分配到桶
    for num in arr:
        idx = (num - min_val) // bucket_size
        buckets[idx].append(num)
    
    # 桶内排序并合并
    result = []
    for bucket in buckets:
        bucket.sort()
        result.extend(bucket)
    
    arr[:] = result
    return arr


def bucket_sort_with_steps(arr: List[float]) -> List[float]:
    """带步骤输出的桶排序"""
    n = len(arr)
    print(f"初始数组: {arr}")
    
    buckets = [[] for _ in range(n)]
    
    for num in arr:
        idx = int(num * n)
        if idx >= n:
            idx = n - 1
        buckets[idx].append(num)
    
    print(f"\n分配到各桶:")
    for i, bucket in enumerate(buckets):
        if bucket:
            print(f"  桶 {i}: {bucket}")
    
    for bucket in buckets:
        bucket.sort()
    
    print(f"\n桶内排序后:")
    for i, bucket in enumerate(buckets):
        if bucket:
            print(f"  桶 {i}: {bucket}")
    
    result = []
    for bucket in buckets:
        result.extend(bucket)
    
    print(f"\n合并结果: {result}")
    return result


if __name__ == "__main__":
    print("====== 桶排序 (Bucket Sort) ======")
    
    # 测试浮点数桶排序
    arr = [0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51]
    print(f"原始数组: {arr}")
    bucket_sort(arr)
    print(f"排序后: {arr}")
    
    # 测试整数桶排序
    print("\n--- 整数桶排序 ---")
    arr2 = [29, 25, 3, 49, 9, 37, 21, 43]
    print(f"原始数组: {arr2}")
    bucket_sort_int(arr2, 10)
    print(f"排序后: {arr2}")
    
    # 演示过程
    print("\n====== 演示排序过程 ======")
    arr3 = [0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68]
    bucket_sort_with_steps(arr3)
    
    # 大范围数据
    print("\n--- 大范围数据 ---")
    arr4 = [64, 34, 25, 12, 22, 11, 90, 100, 5, 88]
    print(f"原始数组: {arr4}")
    bucket_sort_int(arr4, 20)
    print(f"排序后: {arr4}")
