#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
基数排序 (Radix Sort)

基数排序是一种非比较排序算法

算法原理：
1. 找出最大数，确定最大位数
2. 从最低位（个位）开始，对每一位进行排序
3. 使用计数排序作为子排序算法
4. 重复直到最高位排序完成

两种方式：
- LSD (Least Significant Digit): 从最低位开始
- MSD (Most Significant Digit): 从最高位开始

时间复杂度：O(d * (n + k))，d 是位数，k 是基数
空间复杂度：O(n + k)
稳定性：稳定排序
"""

from typing import List


def radix_sort(arr: List[int]) -> List[int]:
    """
    基数排序（LSD 方式，适用于非负整数）
    
    Args:
        arr: 待排序的非负整数列表
        
    Returns:
        排序后的列表
    """
    if len(arr) <= 1:
        return arr
    
    # 找最大值以确定位数
    max_val = max(arr)
    
    # 从个位开始，对每一位进行计数排序
    exp = 1
    while max_val // exp > 0:
        counting_sort_by_digit(arr, exp)
        exp *= 10
    
    return arr


def counting_sort_by_digit(arr: List[int], exp: int) -> None:
    """按指定位进行计数排序"""
    n = len(arr)
    output = [0] * n
    count = [0] * 10  # 0-9 十个数字
    
    # 统计当前位上每个数字出现的次数
    for num in arr:
        digit = (num // exp) % 10
        count[digit] += 1
    
    # 累积计数
    for i in range(1, 10):
        count[i] += count[i - 1]
    
    # 从后向前遍历以保持稳定性
    for i in range(n - 1, -1, -1):
        digit = (arr[i] // exp) % 10
        output[count[digit] - 1] = arr[i]
        count[digit] -= 1
    
    arr[:] = output


def radix_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的基数排序"""
    print(f"初始数组: {arr}")
    
    max_val = max(arr)
    print(f"最大值: {max_val}")
    
    exp = 1
    round_num = 1
    while max_val // exp > 0:
        counting_sort_by_digit(arr, exp)
        print(f"第 {round_num} 轮（按第 {round_num} 位排序）: {arr}")
        exp *= 10
        round_num += 1
    
    return arr


def radix_sort_negative(arr: List[int]) -> List[int]:
    """支持负数的基数排序"""
    if len(arr) <= 1:
        return arr
    
    # 分离正数和负数
    positives = [x for x in arr if x >= 0]
    negatives = [-x for x in arr if x < 0]
    
    # 分别排序
    if positives:
        radix_sort(positives)
    if negatives:
        radix_sort(negatives)
        negatives = [-x for x in reversed(negatives)]
    
    # 合并结果
    result = negatives + positives
    arr[:] = result
    return arr


if __name__ == "__main__":
    print("====== 基数排序 (Radix Sort) ======")
    
    # 测试基本排序
    arr = [170, 45, 75, 90, 802, 24, 2, 66]
    print(f"原始数组: {arr}")
    radix_sort(arr)
    print(f"排序后: {arr}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    arr2 = [329, 457, 657, 839, 436, 720, 355]
    radix_sort_with_steps(arr2)
    
    # 测试单位数
    print("\n--- 测试单位数 ---")
    arr3 = [5, 3, 8, 1, 9, 2, 7]
    print(f"原始数组: {arr3}")
    radix_sort(arr3)
    print(f"排序后: {arr3}")
    
    # 测试支持负数
    print("\n--- 测试支持负数 ---")
    arr4 = [170, -45, 75, -90, 802, -24, 2, -66]
    print(f"原始数组: {arr4}")
    radix_sort_negative(arr4)
    print(f"排序后: {arr4}")
    
    # 测试相同位数
    print("\n--- 测试相同位数 ---")
    arr5 = [123, 456, 789, 234, 567, 890]
    print(f"原始数组: {arr5}")
    radix_sort(arr5)
    print(f"排序后: {arr5}")
