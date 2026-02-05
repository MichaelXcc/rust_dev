#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
计数排序 (Counting Sort)

计数排序是一种非比较排序算法

算法原理：
1. 找出数组中的最大值和最小值
2. 创建一个计数数组，统计每个值出现的次数
3. 根据计数数组，将元素放回原数组

适用场景：
- 数据范围较小（k << n 时效率最高）
- 需要稳定排序
- 适合整数排序

时间复杂度：O(n + k)，k 是数据范围
空间复杂度：O(k)
稳定性：稳定排序
"""

from typing import List


def counting_sort(arr: List[int]) -> List[int]:
    """
    计数排序（基本版本）
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的列表（原地排序）
    """
    if len(arr) <= 1:
        return arr
    
    # 找出最大值和最小值
    min_val = min(arr)
    max_val = max(arr)
    
    # 创建计数数组
    range_size = max_val - min_val + 1
    count = [0] * range_size
    
    # 统计每个元素出现的次数
    for num in arr:
        count[num - min_val] += 1
    
    # 根据计数数组重建原数组
    index = 0
    for i in range(range_size):
        while count[i] > 0:
            arr[index] = i + min_val
            index += 1
            count[i] -= 1
    
    return arr


def counting_sort_stable(arr: List[int]) -> List[int]:
    """
    稳定版本的计数排序
    使用累积计数和从后向前遍历来保持稳定性
    """
    if len(arr) <= 1:
        return arr
    
    min_val = min(arr)
    max_val = max(arr)
    range_size = max_val - min_val + 1
    
    # 统计次数
    count = [0] * range_size
    for num in arr:
        count[num - min_val] += 1
    
    # 累积计数（前缀和）
    for i in range(1, range_size):
        count[i] += count[i - 1]
    
    # 创建输出数组，从后向前遍历以保持稳定性
    output = [0] * len(arr)
    for i in range(len(arr) - 1, -1, -1):
        output[count[arr[i] - min_val] - 1] = arr[i]
        count[arr[i] - min_val] -= 1
    
    # 复制回原数组
    arr[:] = output
    return arr


def counting_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的计数排序"""
    print(f"初始数组: {arr}")
    
    min_val = min(arr)
    max_val = max(arr)
    range_size = max_val - min_val + 1
    
    count = [0] * range_size
    for num in arr:
        count[num - min_val] += 1
    
    print(f"数据范围: [{min_val}, {max_val}]")
    print(f"计数数组: {count}")
    
    index = 0
    for i in range(range_size):
        while count[i] > 0:
            arr[index] = i + min_val
            index += 1
            count[i] -= 1
    
    print(f"排序结果: {arr}")
    return arr


if __name__ == "__main__":
    print("====== 计数排序 (Counting Sort) ======")
    
    # 测试基本排序
    arr = [4, 2, 2, 8, 3, 3, 1]
    print(f"原始数组: {arr}")
    counting_sort(arr)
    print(f"排序后: {arr}")
    
    # 测试稳定版本
    print("\n--- 稳定版本 ---")
    arr2 = [4, 2, 2, 8, 3, 3, 1]
    print(f"原始数组: {arr2}")
    counting_sort_stable(arr2)
    print(f"排序后: {arr2}")
    
    # 包含负数
    print("\n--- 包含负数 ---")
    arr3 = [-5, -10, 0, -3, 8, 5, -1, 10]
    print(f"原始数组: {arr3}")
    counting_sort(arr3)
    print(f"排序后: {arr3}")
    
    # 演示过程
    print("\n====== 演示排序过程 ======")
    arr4 = [1, 4, 1, 2, 7, 5, 2]
    counting_sort_with_steps(arr4)
    
    # 大量重复元素
    print("\n--- 大量重复元素 ---")
    arr5 = [1, 1, 1, 2, 2, 3, 3, 3, 3]
    print(f"原始数组: {arr5}")
    counting_sort(arr5)
    print(f"排序后: {arr5}")
