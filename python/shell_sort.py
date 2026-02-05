#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
希尔排序 (Shell Sort)

希尔排序是插入排序的一种改进版本，也称为"缩小增量排序"

算法原理：
1. 选择一个增量序列 gap1, gap2, ..., gapk，其中 gapk = 1
2. 按增量序列的个数 k，对序列进行 k 趟排序
3. 每趟排序，根据对应的增量 gap，将待排序列分割成若干子序列，
   分别对各子表进行直接插入排序
4. 当增量 gap = 1 时，整个序列作为一个表来处理，排序完成

时间复杂度：取决于增量序列
- Shell 增量：O(n²)
- Hibbard 增量：O(n^1.5)
- Sedgewick 增量：O(n^1.3)

空间复杂度：O(1)
稳定性：不稳定排序
"""

from typing import List


def shell_sort(arr: List[int]) -> List[int]:
    """
    希尔排序（使用 Shell 原始增量序列：n/2, n/4, ..., 1）
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的列表（原地排序）
    """
    n = len(arr)
    gap = n // 2
    
    # gap 为增量，每次减半直到 gap = 1
    while gap > 0:
        # 对每个子序列进行插入排序
        for i in range(gap, n):
            temp = arr[i]
            j = i
            
            # 在子序列中进行插入排序
            while j >= gap and arr[j - gap] > temp:
                arr[j] = arr[j - gap]
                j -= gap
            
            arr[j] = temp
        
        gap //= 2
    
    return arr


def shell_sort_hibbard(arr: List[int]) -> List[int]:
    """使用 Hibbard 增量序列的希尔排序 (1, 3, 7, 15, 31, ...)"""
    n = len(arr)
    
    # 计算初始增量（最大的 2^k - 1 且小于 n）
    gap = 1
    while gap < n // 3:
        gap = gap * 2 + 1
    
    while gap >= 1:
        for i in range(gap, n):
            temp = arr[i]
            j = i
            
            while j >= gap and arr[j - gap] > temp:
                arr[j] = arr[j - gap]
                j -= gap
            
            arr[j] = temp
        
        gap = (gap - 1) // 2
    
    return arr


def shell_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的希尔排序"""
    n = len(arr)
    gap = n // 2
    
    print(f"初始数组: {arr}")
    round_num = 1
    
    while gap > 0:
        print(f"\n第 {round_num} 轮: 增量 gap = {gap}")
        
        for i in range(gap, n):
            temp = arr[i]
            j = i
            
            while j >= gap and arr[j - gap] > temp:
                arr[j] = arr[j - gap]
                j -= gap
            
            arr[j] = temp
        
        print(f"本轮结束后: {arr}")
        gap //= 2
        round_num += 1
    
    return arr


if __name__ == "__main__":
    print("====== 希尔排序 (Shell Sort) ======")
    
    # 测试基本排序
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    shell_sort(arr)
    print(f"排序后: {arr}")
    
    # 测试 Hibbard 增量序列
    print("\n====== Hibbard 增量序列 ======")
    arr2 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr2}")
    shell_sort_hibbard(arr2)
    print(f"排序后: {arr2}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    arr3 = [8, 9, 1, 7, 2, 3, 5, 4, 6, 0]
    shell_sort_with_steps(arr3)
    
    # 测试较大数组
    print("\n--- 测试较大数组 ---")
    large_arr = [35, 33, 42, 10, 14, 19, 27, 44, 26, 31, 28, 36, 11, 24, 18, 37]
    print(f"原始数组: {large_arr}")
    shell_sort(large_arr)
    print(f"排序后: {large_arr}")
    
    # 测试包含重复元素的数组
    print("\n--- 测试包含重复元素的数组 ---")
    duplicate_arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    print(f"原始数组: {duplicate_arr}")
    shell_sort(duplicate_arr)
    print(f"排序后: {duplicate_arr}")
