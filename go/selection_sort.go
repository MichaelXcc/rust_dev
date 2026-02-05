// Package main 实现选择排序算法
// 选择排序 (Selection Sort) 是一种简单直观的排序算法
//
// 算法原理：
//  1. 首先在未排序序列中找到最小（或最大）元素，存放到排序序列的起始位置
//  2. 然后，再从剩余未排序元素中继续寻找最小（或最大）元素，
//     然后放到已排序序列的末尾
//  3. 重复第二步，直到所有元素均排序完毕
//
// 时间复杂度：
// - 最好情况：O(n²)
// - 最坏情况：O(n²)
// - 平均情况：O(n²)
// 注意：无论数组初始状态如何，都需要进行相同次数的比较
//
// 空间复杂度：O(1) - 只需要常数级别的额外空间
//
// 稳定性：不稳定排序（交换可能会改变相等元素的相对位置）
package main

import "fmt"

// selectionSort 选择排序函数
// 参数：arr - 待排序的整数切片
// 返回：排序后的切片（原地排序，返回的是同一个切片）
func selectionSort(arr []int) []int {
	n := len(arr)

	// 外层循环：控制当前需要填充的位置
	// 从第一个位置到倒数第二个位置
	for i := 0; i < n-1; i++ {
		// 假设当前位置的元素就是最小的
		minIndex := i

		// 内层循环：在未排序部分中找到最小元素的索引
		for j := i + 1; j < n; j++ {
			// 如果找到更小的元素，更新最小元素的索引
			if arr[j] < arr[minIndex] {
				minIndex = j
			}
		}

		// 如果最小元素不在当前位置，则交换
		// 这个判断可以减少不必要的交换操作
		if minIndex != i {
			arr[i], arr[minIndex] = arr[minIndex], arr[i]
		}
	}

	return arr
}

// selectionSortDescending 降序选择排序（找最大元素）
// 参数：arr - 待排序的整数切片
// 返回：降序排序后的切片
func selectionSortDescending(arr []int) []int {
	n := len(arr)

	for i := 0; i < n-1; i++ {
		// 找最大元素
		maxIndex := i

		for j := i + 1; j < n; j++ {
			if arr[j] > arr[maxIndex] {
				maxIndex = j
			}
		}

		if maxIndex != i {
			arr[i], arr[maxIndex] = arr[maxIndex], arr[i]
		}
	}

	return arr
}

// main 函数：程序入口，演示选择排序的使用
func main() {
	// 创建一个无序的整数切片
	arr := []int{64, 34, 25, 12, 22, 11, 90}

	fmt.Println("====== 选择排序 (Selection Sort) ======")
	fmt.Println("原始数组:", arr)

	// 调用选择排序函数
	selectionSort(arr)

	fmt.Println("升序排序后:", arr)

	// 测试降序排序
	fmt.Println("\n--- 测试降序排序 ---")
	arr2 := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr2)
	selectionSortDescending(arr2)
	fmt.Println("降序排序后:", arr2)

	// 测试已排序数组
	fmt.Println("\n--- 测试已排序数组 ---")
	sortedArr := []int{1, 2, 3, 4, 5, 6, 7}
	fmt.Println("原始数组:", sortedArr)
	selectionSort(sortedArr)
	fmt.Println("排序后:", sortedArr)

	// 测试包含重复元素的数组
	fmt.Println("\n--- 测试包含重复元素的数组 ---")
	duplicateArr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", duplicateArr)
	selectionSort(duplicateArr)
	fmt.Println("排序后:", duplicateArr)

	// 测试单元素数组
	fmt.Println("\n--- 测试单元素数组 ---")
	singleArr := []int{42}
	fmt.Println("原始数组:", singleArr)
	selectionSort(singleArr)
	fmt.Println("排序后:", singleArr)

	// 测试空数组
	fmt.Println("\n--- 测试空数组 ---")
	emptyArr := []int{}
	fmt.Println("原始数组:", emptyArr)
	selectionSort(emptyArr)
	fmt.Println("排序后:", emptyArr)
}
