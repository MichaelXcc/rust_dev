#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
堆排序 (Heap Sort)

堆排序利用堆这种数据结构设计的一种排序算法

堆的定义：
- 最大堆：父节点的值总是大于或等于其子节点的值
- 最小堆：父节点的值总是小于或等于其子节点的值

算法原理：
1. 构建最大堆：将无序数组构建成最大堆
2. 交换堆顶与末尾：将堆顶（最大值）与堆末尾元素交换
3. 调整堆：将剩余元素重新调整为最大堆
4. 重复直到堆大小为1

时间复杂度：O(n log n) - 所有情况
空间复杂度：O(1) - 原地排序
稳定性：不稳定排序
"""

from typing import List


def heap_sort(arr: List[int]) -> List[int]:
    """
    堆排序主函数
    
    Args:
        arr: 待排序的整数列表
        
    Returns:
        排序后的列表（原地排序）
    """
    n = len(arr)
    
    # 构建最大堆（从最后一个非叶子节点开始）
    for i in range(n // 2 - 1, -1, -1):
        heapify(arr, n, i)
    
    # 逐个提取堆顶元素
    for i in range(n - 1, 0, -1):
        # 将堆顶（最大值）与当前末尾交换
        arr[0], arr[i] = arr[i], arr[0]
        # 对剩余元素重新调整为最大堆
        heapify(arr, i, 0)
    
    return arr


def heapify(arr: List[int], n: int, i: int) -> None:
    """
    调整堆，使以 i 为根的子树成为最大堆
    
    Args:
        arr: 待调整的数组
        n: 堆的大小
        i: 根节点索引
    """
    largest = i       # 初始化最大值为根节点
    left = 2 * i + 1  # 左子节点索引
    right = 2 * i + 2 # 右子节点索引
    
    # 如果左子节点存在且大于根节点
    if left < n and arr[left] > arr[largest]:
        largest = left
    
    # 如果右子节点存在且大于当前最大值
    if right < n and arr[right] > arr[largest]:
        largest = right
    
    # 如果最大值不是根节点，则交换并继续调整
    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]
        heapify(arr, n, largest)


def heap_sort_with_steps(arr: List[int]) -> List[int]:
    """带步骤输出的堆排序"""
    n = len(arr)
    print(f"初始数组: {arr}")
    
    # 构建最大堆
    print("\n--- 构建最大堆 ---")
    for i in range(n // 2 - 1, -1, -1):
        heapify(arr, n, i)
        print(f"调整节点 {i} 后: {arr}")
    
    # 逐个提取
    print("\n--- 提取元素 ---")
    for i in range(n - 1, 0, -1):
        arr[0], arr[i] = arr[i], arr[0]
        heapify(arr, i, 0)
        print(f"提取第 {n - i} 大元素后: {arr}")
    
    return arr


if __name__ == "__main__":
    print("====== 堆排序 (Heap Sort) ======")
    
    # 测试基本排序
    arr = [64, 34, 25, 12, 22, 11, 90]
    print(f"原始数组: {arr}")
    heap_sort(arr)
    print(f"排序后: {arr}")
    
    # 演示排序过程
    print("\n====== 演示排序过程 ======")
    arr2 = [4, 10, 3, 5, 1]
    heap_sort_with_steps(arr2)
    
    # 测试逆序数组
    print("\n--- 测试逆序数组 ---")
    arr3 = [7, 6, 5, 4, 3, 2, 1]
    print(f"原始数组: {arr3}")
    heap_sort(arr3)
    print(f"排序后: {arr3}")
    
    # 测试重复元素
    print("\n--- 测试重复元素 ---")
    arr4 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    print(f"原始数组: {arr4}")
    heap_sort(arr4)
    print(f"排序后: {arr4}")
