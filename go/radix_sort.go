// Package main 实现基数排序算法
// 基数排序 (Radix Sort) 是一种非比较排序算法
//
// 算法原理：
// 1. 找出最大数，确定最大位数
// 2. 从最低位（个位）开始，对每一位进行排序
// 3. 可以使用计数排序作为子排序算法
// 4. 重复直到最高位排序完成
//
// 两种方式：
// - LSD (Least Significant Digit): 从最低位开始
// - MSD (Most Significant Digit): 从最高位开始
//
// 时间复杂度：O(d * (n + k))，d 是位数，k 是基数（通常为10）
// 空间复杂度：O(n + k)
// 稳定性：稳定
package main

import "fmt"

// radixSort 基数排序（LSD 方式，适用于非负整数）
func radixSort(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	// 找最大值以确定位数
	maxVal := arr[0]
	for _, v := range arr {
		if v > maxVal {
			maxVal = v
		}
	}

	// 从个位开始，对每一位进行计数排序
	for exp := 1; maxVal/exp > 0; exp *= 10 {
		countingSortByDigit(arr, exp)
	}

	return arr
}

// countingSortByDigit 按指定位进行计数排序
func countingSortByDigit(arr []int, exp int) {
	n := len(arr)
	output := make([]int, n)
	count := make([]int, 10) // 0-9 十个数字

	// 统计当前位上每个数字出现的次数
	for _, v := range arr {
		digit := (v / exp) % 10
		count[digit]++
	}

	// 累积计数
	for i := 1; i < 10; i++ {
		count[i] += count[i-1]
	}

	// 从后向前遍历以保持稳定性
	for i := n - 1; i >= 0; i-- {
		digit := (arr[i] / exp) % 10
		output[count[digit]-1] = arr[i]
		count[digit]--
	}

	copy(arr, output)
}

// radixSortWithSteps 带步骤输出的基数排序
func radixSortWithSteps(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	maxVal := arr[0]
	for _, v := range arr {
		if v > maxVal {
			maxVal = v
		}
	}

	fmt.Printf("初始数组: %v\n", arr)
	fmt.Printf("最大值: %d\n", maxVal)

	round := 1
	for exp := 1; maxVal/exp > 0; exp *= 10 {
		countingSortByDigit(arr, exp)
		fmt.Printf("第 %d 轮（按第 %d 位排序）: %v\n", round, round, arr)
		round++
	}

	return arr
}

func main() {
	fmt.Println("====== 基数排序 (Radix Sort) ======")

	arr := []int{170, 45, 75, 90, 802, 24, 2, 66}
	fmt.Println("原始数组:", arr)
	radixSort(arr)
	fmt.Println("排序后:", arr)

	// 演示排序过程
	fmt.Println("\n====== 演示排序过程 ======")
	arr2 := []int{329, 457, 657, 839, 436, 720, 355}
	radixSortWithSteps(arr2)

	// 测试单位数
	fmt.Println("\n--- 测试单位数 ---")
	arr3 := []int{5, 3, 8, 1, 9, 2, 7}
	fmt.Println("原始数组:", arr3)
	radixSort(arr3)
	fmt.Println("排序后:", arr3)

	// 测试相同位数
	fmt.Println("\n--- 测试相同位数 ---")
	arr4 := []int{123, 456, 789, 234, 567, 890}
	fmt.Println("原始数组:", arr4)
	radixSort(arr4)
	fmt.Println("排序后:", arr4)
}
