// Package main 实现插入排序算法
// 插入排序 (Insertion Sort) 是一种简单直观的排序算法
//
// 算法原理：
// 1. 从第一个元素开始，该元素可以认为已经被排序
// 2. 取出下一个元素，在已经排序的元素序列中从后向前扫描
// 3. 如果该元素（已排序）大于新元素，将该元素移到下一位置
// 4. 重复步骤3，直到找到已排序的元素小于或者等于新元素的位置
// 5. 将新元素插入到该位置后
// 6. 重复步骤2~5
//
// 类比：就像我们打扑克牌时整理手牌的过程
//
// 时间复杂度：
// - 最好情况：O(n) - 数组已经有序，只需要遍历一次
// - 最坏情况：O(n²) - 数组完全逆序
// - 平均情况：O(n²)
//
// 空间复杂度：O(1) - 只需要常数级别的额外空间
//
// 稳定性：稳定排序（相等元素的相对位置不会改变）
//
// 特点：对于小规模数据或者基本有序的数据，插入排序效率很高
package main

import "fmt"

// insertionSort 插入排序函数
// 参数：arr - 待排序的整数切片
// 返回：排序后的切片（原地排序，返回的是同一个切片）
func insertionSort(arr []int) []int {
	n := len(arr)

	// 从第二个元素开始，因为第一个元素默认已经"有序"
	for i := 1; i < n; i++ {
		// 保存当前要插入的元素
		key := arr[i]

		// j 指向已排序部分的最后一个元素
		j := i - 1

		// 在已排序序列中从后向前扫描
		// 找到适当的位置并将大于 key 的元素向后移动
		for j >= 0 && arr[j] > key {
			// 将元素向后移动一位
			arr[j+1] = arr[j]
			j--
		}

		// 将 key 插入到正确的位置
		// j+1 是因为循环结束后 j 指向的是小于等于 key 的元素位置
		arr[j+1] = key
	}

	return arr
}

// insertionSortWithSteps 带步骤输出的插入排序（用于演示过程）
// 参数：arr - 待排序的整数切片
// 返回：排序后的切片
func insertionSortWithSteps(arr []int) []int {
	n := len(arr)

	fmt.Println("开始插入排序过程：")
	fmt.Printf("初始数组: %v\n", arr)

	for i := 1; i < n; i++ {
		key := arr[i]
		j := i - 1

		fmt.Printf("\n第 %d 轮：插入元素 %d\n", i, key)

		moveCount := 0
		for j >= 0 && arr[j] > key {
			arr[j+1] = arr[j]
			j--
			moveCount++
		}

		arr[j+1] = key

		if moveCount > 0 {
			fmt.Printf("移动了 %d 个元素，插入到位置 %d\n", moveCount, j+1)
		} else {
			fmt.Printf("元素已在正确位置，无需移动\n")
		}
		fmt.Printf("当前数组: %v\n", arr)
	}

	return arr
}

// binaryInsertionSort 二分插入排序（优化版本）
// 使用二分查找来确定插入位置，减少比较次数
// 注意：虽然比较次数减少了，但移动次数没有变，所以时间复杂度仍然是 O(n²)
func binaryInsertionSort(arr []int) []int {
	n := len(arr)

	for i := 1; i < n; i++ {
		key := arr[i]

		// 使用二分查找找到插入位置
		left, right := 0, i
		for left < right {
			mid := (left + right) / 2
			if arr[mid] > key {
				right = mid
			} else {
				left = mid + 1
			}
		}

		// 将 [left, i-1] 范围内的元素向后移动一位
		for j := i; j > left; j-- {
			arr[j] = arr[j-1]
		}

		// 插入 key 到正确位置
		arr[left] = key
	}

	return arr
}

// main 函数：程序入口，演示插入排序的使用
func main() {
	// 创建一个无序的整数切片
	arr := []int{64, 34, 25, 12, 22, 11, 90}

	fmt.Println("====== 插入排序 (Insertion Sort) ======")
	fmt.Println("原始数组:", arr)

	// 调用插入排序函数
	insertionSort(arr)

	fmt.Println("排序后:", arr)

	// 演示排序过程
	fmt.Println("\n====== 演示排序过程 ======")
	arr2 := []int{5, 2, 4, 6, 1, 3}
	insertionSortWithSteps(arr2)

	// 测试二分插入排序
	fmt.Println("\n====== 二分插入排序 ======")
	arr3 := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr3)
	binaryInsertionSort(arr3)
	fmt.Println("排序后:", arr3)

	// 测试已排序数组（最好情况）
	fmt.Println("\n--- 测试已排序数组 ---")
	sortedArr := []int{1, 2, 3, 4, 5, 6, 7}
	fmt.Println("原始数组:", sortedArr)
	insertionSort(sortedArr)
	fmt.Println("排序后:", sortedArr)

	// 测试逆序数组（最坏情况）
	fmt.Println("\n--- 测试逆序数组 ---")
	reversedArr := []int{7, 6, 5, 4, 3, 2, 1}
	fmt.Println("原始数组:", reversedArr)
	insertionSort(reversedArr)
	fmt.Println("排序后:", reversedArr)

	// 测试包含重复元素的数组
	fmt.Println("\n--- 测试包含重复元素的数组 ---")
	duplicateArr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", duplicateArr)
	insertionSort(duplicateArr)
	fmt.Println("排序后:", duplicateArr)
}
