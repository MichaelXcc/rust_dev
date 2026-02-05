#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
插入排序 (Insertion Sort)

算法原理：
1. 从第一个元素开始，该元素可以认为已经被排序
2. 取出下一个元素，在已经排序的元素序列中从后向前扫描
3. 如果该元素（已排序）大于新元素，将该元素移到下一位置
4. 重复步骤3，直到找到已排序的元素小于或者等于新元素的位置
5. 将新元素插入到该位置后
6. 重复步骤2~5

类比：就像打扑克牌时整理手牌的过程

时间复杂度：
- 最好情况：O(n) - 数组已经有序
- 最坏情况：O(n²) - 数组完全逆序
- 平均情况：O(n²)

空间复杂度：O(1)
稳定性：稳定排序
"""

from typing import List
import bisect


def insertion_sort(arr: List[int]) -> List[int]:
    """
    插入排序函数
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的列表（原地排序）
    """
    n = len(arr)
    
    # 从第二个元素开始，因为第一个元素默认已经"有序"
    for i in range(1, n):
        # 保存当前要插入的元素
        key = arr[i]
        
        # j 指向已排序部分的最后一个元素
        j = i - 1
        
        # 在已排序序列中从后向前扫描，找到适当位置并移动元素
        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1
        
        # 将 key 插入到正确的位置
        arr[j + 1] = key
    
    return arr


def binary_insertion_sort(arr: List[int]) -> List[int]:
    """
    二分插入排序（优化版本）
    使用二分查找来确定插入位置，减少比较次数
    """
    for i in range(1, len(arr)):
        key = arr[i]
        # 使用二分查找找到插入位置
        pos = bisect.bisect_left(arr, key, 0, i)
        # 将 [pos, i-1] 范围内的元素向后移动
        arr[pos + 1:i + 1] = arr[pos:i]
        arr[pos] = key
    
    return arr


def insertion_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的插入排序"""
    n = len(arr)
    print(f"初始数组: {arr}")
    
    for i in range(1, n):
        key = arr[i]
        j = i - 1
        
        print(f"\n第 {i} 轮: 插入元素 {key}")
        
        move_count = 0
        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1
            move_count += 1
        
        arr[j + 1] = key
        
        if move_count > 0:
            print(f"  移动了 {move_count} 个元素，插入到位置 {j + 1}")
        else:
            print(f"  元素已在正确位置，无需移动")
        print(f"  当前数组: {arr}")
    
    return arr


if __name__ == "__main__":
    print("====== 插入排序 (Insertion Sort) ======")
    
    # 测试基本排序
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    insertion_sort(arr)
    print(f"排序后: {arr}")
    
    # 测试二分插入排序
    print("\n====== 二分插入排序 ======")
    arr2 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr2}")
    binary_insertion_sort(arr2)
    print(f"排序后: {arr2}")
    
    # 测试已排序数组（最好情况）
    print("\n--- 测试已排序数组 ---")
    sorted_arr = [1, 2, 3, 4, 5, 6, 7]
    print(f"原始数组: {sorted_arr}")
    insertion_sort(sorted_arr)
    print(f"排序后: {sorted_arr}")
    
    # 测试逆序数组（最坏情况）
    print("\n--- 测试逆序数组 ---")
    reversed_arr = [7, 6, 5, 4, 3, 2, 1]
    print(f"原始数组: {reversed_arr}")
    insertion_sort(reversed_arr)
    print(f"排序后: {reversed_arr}")
    
    # 测试包含重复元素的数组
    print("\n--- 测试包含重复元素的数组 ---")
    duplicate_arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    print(f"原始数组: {duplicate_arr}")
    insertion_sort(duplicate_arr)
    print(f"排序后: {duplicate_arr}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    demo_arr = [5, 2, 4, 6, 1, 3]
    insertion_sort_with_steps(demo_arr)
