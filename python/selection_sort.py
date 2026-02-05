#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
选择排序 (Selection Sort)

算法原理：
1. 首先在未排序序列中找到最小（或最大）元素，存放到排序序列的起始位置
2. 然后，再从剩余未排序元素中继续寻找最小（或最大）元素，放到已排序序列的末尾
3. 重复第二步，直到所有元素均排序完毕

时间复杂度：O(n²) - 所有情况都相同
空间复杂度：O(1)
稳定性：不稳定排序
"""

from typing import List


def selection_sort(arr: List[int]) -> List[int]:
    """
    选择排序函数（升序）
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的列表（原地排序）
    """
    n = len(arr)
    
    # 外层循环：控制当前需要填充的位置
    for i in range(n - 1):
        # 假设当前位置的元素就是最小的
        min_idx = i
        
        # 内层循环：在未排序部分中找到最小元素的索引
        for j in range(i + 1, n):
            if arr[j] < arr[min_idx]:
                min_idx = j
        
        # 如果最小元素不在当前位置，则交换
        if min_idx != i:
            arr[i], arr[min_idx] = arr[min_idx], arr[i]
    
    return arr


def selection_sort_descending(arr: List[int]) -> List[int]:
    """选择排序（降序）"""
    n = len(arr)
    
    for i in range(n - 1):
        max_idx = i
        
        for j in range(i + 1, n):
            if arr[j] > arr[max_idx]:
                max_idx = j
        
        if max_idx != i:
            arr[i], arr[max_idx] = arr[max_idx], arr[i]
    
    return arr


def selection_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的选择排序"""
    n = len(arr)
    print(f"初始数组: {arr}")
    
    for i in range(n - 1):
        min_idx = i
        
        for j in range(i + 1, n):
            if arr[j] < arr[min_idx]:
                min_idx = j
        
        if min_idx != i:
            print(f"第 {i + 1} 轮: 找到最小值 {arr[min_idx]}，与位置 {i} 的 {arr[i]} 交换")
            arr[i], arr[min_idx] = arr[min_idx], arr[i]
        else:
            print(f"第 {i + 1} 轮: 位置 {i} 的 {arr[i]} 已是最小值")
        
        print(f"         结果: {arr}")
    
    return arr


if __name__ == "__main__":
    print("====== 选择排序 (Selection Sort) ======")
    
    # 测试基本排序
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    selection_sort(arr)
    print(f"升序排序后: {arr}")
    
    # 测试降序排序
    print("\n--- 测试降序排序 ---")
    arr2 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr2}")
    selection_sort_descending(arr2)
    print(f"降序排序后: {arr2}")
    
    # 测试已排序数组
    print("\n--- 测试已排序数组 ---")
    sorted_arr = [1, 2, 3, 4, 5, 6, 7]
    print(f"原始数组: {sorted_arr}")
    selection_sort(sorted_arr)
    print(f"排序后: {sorted_arr}")
    
    # 测试包含重复元素的数组
    print("\n--- 测试包含重复元素的数组 ---")
    duplicate_arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    print(f"原始数组: {duplicate_arr}")
    selection_sort(duplicate_arr)
    print(f"排序后: {duplicate_arr}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    demo_arr = [29, 10, 14, 37, 13]
    selection_sort_with_steps(demo_arr)
