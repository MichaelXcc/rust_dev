#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
冒泡排序 (Bubble Sort)

算法原理：
1. 比较相邻的两个元素，如果第一个比第二个大，就交换它们的位置
2. 对每一对相邻元素做同样的工作，从开始第一对到结尾的最后一对
3. 这样在最后的元素应该会是最大的数
4. 针对所有的元素重复以上的步骤，除了最后一个
5. 重复直到排序完成

时间复杂度：
- 最好情况：O(n) - 数组已经有序
- 最坏情况：O(n²) - 数组完全逆序
- 平均情况：O(n²)

空间复杂度：O(1) - 只需要常数级别的额外空间
稳定性：稳定排序
"""

from typing import List


def bubble_sort(arr: List[int]) -> List[int]:
    """
    冒泡排序函数
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的列表（原地排序）
    """
    n = len(arr)
    
    # 外层循环：控制排序的轮数
    for i in range(n - 1):
        # swapped 标志用于优化：如果某一轮没有发生交换，说明数组已经有序
        swapped = False
        
        # 内层循环：进行相邻元素的比较和交换
        # 每轮结束后，最大的元素会"冒泡"到末尾
        for j in range(n - 1 - i):
            # 如果前一个元素比后一个元素大，则交换它们
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True
        
        # 如果这一轮没有发生任何交换，说明数组已经有序，提前结束
        if not swapped:
            break
    
    return arr


def bubble_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的冒泡排序（用于演示过程）"""
    n = len(arr)
    print(f"初始数组: {arr}")
    
    for i in range(n - 1):
        swapped = False
        print(f"\n第 {i + 1} 轮:")
        
        for j in range(n - 1 - i):
            if arr[j] > arr[j + 1]:
                print(f"  交换 {arr[j]} 和 {arr[j + 1]}")
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True
        
        print(f"  本轮结果: {arr}")
        
        if not swapped:
            print("  没有交换，排序完成！")
            break
    
    return arr


if __name__ == "__main__":
    print("====== 冒泡排序 (Bubble Sort) ======")
    
    # 测试基本排序
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    bubble_sort(arr)
    print(f"排序后: {arr}")
    
    # 测试已排序数组（最好情况）
    print("\n--- 测试已排序数组 ---")
    sorted_arr = [1, 2, 3, 4, 5, 6, 7]
    print(f"原始数组: {sorted_arr}")
    bubble_sort(sorted_arr)
    print(f"排序后: {sorted_arr}")
    
    # 测试逆序数组（最坏情况）
    print("\n--- 测试逆序数组 ---")
    reversed_arr = [7, 6, 5, 4, 3, 2, 1]
    print(f"原始数组: {reversed_arr}")
    bubble_sort(reversed_arr)
    print(f"排序后: {reversed_arr}")
    
    # 测试包含重复元素的数组
    print("\n--- 测试包含重复元素的数组 ---")
    duplicate_arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    print(f"原始数组: {duplicate_arr}")
    bubble_sort(duplicate_arr)
    print(f"排序后: {duplicate_arr}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    demo_arr = [5, 3, 8, 4, 2]
    bubble_sort_with_steps(demo_arr)
