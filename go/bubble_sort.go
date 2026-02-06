// Package main 实现冒泡排序算法
// 冒泡排序 (Bubble Sort) 是一种简单的排序算法
//
// 算法原理：
// 1. 比较相邻的两个元素，如果第一个比第二个大，就交换它们的位置
// 2. 对每一对相邻元素做同样的工作，从开始第一对到结尾的最后一对
// 3. 这样在最后的元素应该会是最大的数
// 4. 针对所有的元素重复以上的步骤，除了最后一个
// 5. 重复步骤1~4，直到排序完成
//
// 时间复杂度：
// - 最好情况：O(n) - 数组已经有序
// - 最坏情况：O(n²) - 数组完全逆序
// - 平均情况：O(n²)
//
// 空间复杂度：O(1) - 只需要常数级别的额外空间
//
// 稳定性：稳定排序（相等元素的相对位置不会改变）
package main

import "fmt"

// bubbleSort 冒泡排序函数
// 参数：arr - 待排序的整数切片
// 返回：排序后的切片（原地排序，返回的是同一个切片）
func bubbleSort(arr []int) []int {
	n := len(arr)

	// 外层循环：控制排序的轮数
	// 共需要 n-1 轮排序
	for i := 0; i < n-1; i++ {
		// swapped 标志用于优化：如果某一轮没有发生交换，说明数组已经有序
		swapped := false

		// 内层循环：进行相邻元素的比较和交换
		// 每轮结束后，最大的元素会"冒泡"到末尾
		// 所以每轮需要比较的次数是 n-1-i
		for j := 0; j < n-1-i; j++ {
			// 如果前一个元素比后一个元素大，则交换它们
			if arr[j] > arr[j+1] {
				// Go 语言的多重赋值，简洁地交换两个变量的值
				arr[j], arr[j+1] = arr[j+1], arr[j]
				swapped = true
			}
		}

		// 如果这一轮没有发生任何交换，说明数组已经有序，提前结束
		if !swapped {
			break
		}
	}

	return arr
}

// main 函数：程序入口，演示冒泡排序的使用
func main() {
	// 创建一个无序的整数切片
	arr := []int{64, 34, 25, 12, 22, 11, 90}

	fmt.Println("====== 冒泡排序 (Bubble Sort) ======")
	fmt.Println("原始数组:", arr)

	// 调用冒泡排序函数
	bubbleSort(arr)

	fmt.Println("排序后:", arr)

	// 测试已排序数组（最好情况）
	fmt.Println("\n--- 测试已排序数组 ---")
	sortedArr := []int{1, 2, 3, 4, 5, 6, 7}
	fmt.Println("原始数组:", sortedArr)
	bubbleSort(sortedArr)
	fmt.Println("排序后:", sortedArr)

	// 测试逆序数组（最坏情况）
	fmt.Println("\n--- 测试逆序数组 ---")
	reversedArr := []int{7, 6, 5, 4, 3, 2, 1}
	fmt.Println("原始数组:", reversedArr)
	bubbleSort(reversedArr)
	fmt.Println("排序后:", reversedArr)

	// 测试包含重复元素的数组
	fmt.Println("\n--- 测试包含重复元素的数组 ---")
	duplicateArr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", duplicateArr)
	bubbleSort(duplicateArr)
	fmt.Println("排序后:", duplicateArr)
}
