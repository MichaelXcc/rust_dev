#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
归并排序 (Merge Sort)

归并排序是建立在归并操作上的一种有效的排序算法，采用分治法策略

算法原理（分治思想）：
1. 分解（Divide）：将待排序的序列分成两个子序列
2. 解决（Conquer）：递归地排序两个子序列
3. 合并（Combine）：合并两个已排序的子序列

时间复杂度：O(n log n) - 所有情况都相同
空间复杂度：O(n) - 需要额外空间存储合并结果
稳定性：稳定排序
"""

from typing import List


def merge_sort(arr: List[int]) -> List[int]:
    """
    归并排序函数（递归实现）
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的新列表
    """
    # 基础情况：如果数组长度小于等于1，直接返回
    if len(arr) <= 1:
        return arr
    
    # 找到中间点，将数组分成两半
    mid = len(arr) // 2
    
    # 递归排序左半部分和右半部分
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    
    # 合并两个已排序的部分
    return merge(left, right)


def merge(left: List[int], right: List[int]) -> List[int]:
    """
    合并两个已排序的列表
    
    Args:
        left: 左半部分已排序列表
        right: 右半部分已排序列表
        
    Returns:
        合并后的已排序列表
    """
    result = []
    i = j = 0
    
    # 比较两个列表的元素，按顺序放入结果列表
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:  # 使用 <= 保证稳定性
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    
    # 将剩余元素添加到结果中
    result.extend(left[i:])
    result.extend(right[j:])
    
    return result


def merge_sort_inplace(arr: List[int], left: int = 0, right: int = None) -> None:
    """原地归并排序（减少空间分配）"""
    if right is None:
        right = len(arr) - 1
    
    if left < right:
        mid = (left + right) // 2
        merge_sort_inplace(arr, left, mid)
        merge_sort_inplace(arr, mid + 1, right)
        merge_inplace(arr, left, mid, right)


def merge_inplace(arr: List[int], left: int, mid: int, right: int) -> None:
    """原地合并"""
    temp = arr[left:right + 1]
    i, j = 0, mid - left + 1
    k = left
    
    while i <= mid - left and j <= right - left:
        if temp[i] <= temp[j]:
            arr[k] = temp[i]
            i += 1
        else:
            arr[k] = temp[j]
            j += 1
        k += 1
    
    while i <= mid - left:
        arr[k] = temp[i]
        i += 1
        k += 1


def merge_sort_bottom_up(arr: List[int]) -> List[int]:
    """自底向上的归并排序（迭代实现）"""
    n = len(arr)
    size = 1
    
    while size < n:
        for left in range(0, n - size, size * 2):
            mid = left + size - 1
            right = min(left + 2 * size - 1, n - 1)
            merge_inplace(arr, left, mid, right)
        size *= 2
    
    return arr


def merge_sort_with_steps(arr: List[int], depth: int = 0) -> List[int]:
    """带步骤输出的归并排序"""
    indent = "  " * depth
    print(f"{indent}分解: {arr}")
    
    if len(arr) <= 1:
        print(f"{indent}返回: {arr} (基础情况)")
        return arr
    
    mid = len(arr) // 2
    left = merge_sort_with_steps(arr[:mid], depth + 1)
    right = merge_sort_with_steps(arr[mid:], depth + 1)
    
    merged = merge(left, right)
    print(f"{indent}合并 {left} 和 {right} -> {merged}")
    
    return merged


if __name__ == "__main__":
    print("====== 归并排序 (Merge Sort) ======")
    
    # 测试基本排序
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    sorted_arr = merge_sort(arr)
    print(f"排序后: {sorted_arr}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    arr2 = [38, 27, 43, 3, 9, 82, 10]
    print(f"原始数组: {arr2}")
    result = merge_sort_with_steps(arr2)
    print(f"最终结果: {result}")
    
    # 测试原地归并排序
    print("\n====== 原地归并排序 ======")
    arr3 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr3}")
    merge_sort_inplace(arr3)
    print(f"排序后: {arr3}")
    
    # 测试自底向上归并排序
    print("\n====== 自底向上归并排序 ======")
    arr4 = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr4}")
    merge_sort_bottom_up(arr4)
    print(f"排序后: {arr4}")
    
    # 测试包含重复元素的数组
    print("\n--- 测试包含重复元素的数组 ---")
    duplicate_arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    print(f"原始数组: {duplicate_arr}")
    result = merge_sort(duplicate_arr)
    print(f"排序后: {result}")
