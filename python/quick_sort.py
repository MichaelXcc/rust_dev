#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
快速排序 (Quick Sort)

快速排序是一种高效的排序算法，采用分治法策略
由 Tony Hoare 在 1960 年提出

算法原理：
1. 选择基准（Pivot）：从数列中挑出一个元素作为基准
2. 分区（Partition）：将小于基准的元素放左边，大于基准的放右边
3. 递归排序：递归地排序左右两个子数组

时间复杂度：
- 最好/平均情况：O(n log n)
- 最坏情况：O(n²) - 每次选择的基准都是最值

空间复杂度：O(log n) 递归栈
稳定性：不稳定排序
"""

from typing import List
import random


def quick_sort(arr: List[int]) -> List[int]:
    """
    快速排序（简洁的 Pythonic 实现）
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的新列表
    """
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quick_sort(left) + middle + quick_sort(right)


def quick_sort_inplace(arr: List[int], low: int = 0, high: int = None) -> None:
    """
    原地快速排序
    
    Args:
        arr: 待排序的整数列表
        low: 起始索引
        high: 结束索引
    """
    if high is None:
        high = len(arr) - 1
    
    if low < high:
        pivot_idx = partition(arr, low, high)
        quick_sort_inplace(arr, low, pivot_idx - 1)
        quick_sort_inplace(arr, pivot_idx + 1, high)


def partition(arr: List[int], low: int, high: int) -> int:
    """Lomuto 分区方案"""
    pivot = arr[high]
    i = low - 1
    
    for j in range(low, high):
        if arr[j] < pivot:
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
    
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1


def quick_sort_random(arr: List[int], low: int = 0, high: int = None) -> None:
    """随机基准快速排序（避免最坏情况）"""
    if high is None:
        high = len(arr) - 1
    
    if low < high:
        # 随机选择基准
        rand_idx = random.randint(low, high)
        arr[rand_idx], arr[high] = arr[high], arr[rand_idx]
        
        pivot_idx = partition(arr, low, high)
        quick_sort_random(arr, low, pivot_idx - 1)
        quick_sort_random(arr, pivot_idx + 1, high)


def quick_sort_3way(arr: List[int], low: int = 0, high: int = None) -> None:
    """
    三路快速排序（荷兰国旗问题）
    适用于包含大量重复元素的数组
    """
    if high is None:
        high = len(arr) - 1
    
    if low >= high:
        return
    
    lt, gt, i = low, high, low + 1
    pivot = arr[low]
    
    while i <= gt:
        if arr[i] < pivot:
            arr[lt], arr[i] = arr[i], arr[lt]
            lt += 1
            i += 1
        elif arr[i] > pivot:
            arr[gt], arr[i] = arr[i], arr[gt]
            gt -= 1
        else:
            i += 1
    
    quick_sort_3way(arr, low, lt - 1)
    quick_sort_3way(arr, gt + 1, high)


if __name__ == "__main__":
    print("====== 快速排序 (Quick Sort) ======")
    
    # 测试简洁版本
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    sorted_arr = quick_sort(arr)
    print(f"排序后: {sorted_arr}")
    
    # 测试原地排序
    print("\n====== 原地快速排序 ======")
    arr2 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr2}")
    quick_sort_inplace(arr2)
    print(f"排序后: {arr2}")
    
    # 测试随机基准
    print("\n====== 随机基准快速排序 ======")
    arr3 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr3}")
    quick_sort_random(arr3)
    print(f"排序后: {arr3}")
    
    # 测试三路快排（大量重复元素）
    print("\n====== 三路快速排序 ======")
    arr4 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 5, 5, 5, 1, 2, 3]
    print(f"原始数组: {arr4}")
    quick_sort_3way(arr4)
    print(f"排序后: {arr4}")
    
    # 测试已排序数组
    print("\n--- 测试已排序数组（使用随机基准）---")
    sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    print(f"原始数组: {sorted_arr}")
    quick_sort_random(sorted_arr)
    print(f"排序后: {sorted_arr}")
