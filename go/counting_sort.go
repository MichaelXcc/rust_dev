// Package main 实现计数排序算法
// 计数排序 (Counting Sort) 是一种非比较排序算法
//
// 算法原理：
// 1. 找出数组中的最大值和最小值
// 2. 创建一个计数数组，统计每个值出现的次数
// 3. 根据计数数组，将元素放回原数组
//
// 适用场景：
// - 数据范围较小（k << n 时效率最高）
// - 需要稳定排序
// - 适合整数排序
//
// 时间复杂度：O(n + k)，k 是数据范围
// 空间复杂度：O(k)
// 稳定性：稳定
package main

import "fmt"

// countingSort 计数排序（基本版本）
func countingSort(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	// 找出最大值和最小值
	minVal, maxVal := arr[0], arr[0]
	for _, v := range arr {
		if v < minVal {
			minVal = v
		}
		if v > maxVal {
			maxVal = v
		}
	}

	// 创建计数数组
	rangeSize := maxVal - minVal + 1
	count := make([]int, rangeSize)

	// 统计每个元素出现的次数
	for _, v := range arr {
		count[v-minVal]++
	}

	// 根据计数数组重建原数组
	index := 0
	for i := 0; i < rangeSize; i++ {
		for count[i] > 0 {
			arr[index] = i + minVal
			index++
			count[i]--
		}
	}

	return arr
}

// countingSortStable 稳定版本的计数排序
func countingSortStable(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	// 找出最大值和最小值
	minVal, maxVal := arr[0], arr[0]
	for _, v := range arr {
		if v < minVal {
			minVal = v
		}
		if v > maxVal {
			maxVal = v
		}
	}

	// 创建计数数组
	rangeSize := maxVal - minVal + 1
	count := make([]int, rangeSize)

	// 统计每个元素出现的次数
	for _, v := range arr {
		count[v-minVal]++
	}

	// 将计数数组变为累积计数（前缀和）
	for i := 1; i < rangeSize; i++ {
		count[i] += count[i-1]
	}

	// 创建输出数组，从后向前遍历以保持稳定性
	output := make([]int, len(arr))
	for i := len(arr) - 1; i >= 0; i-- {
		output[count[arr[i]-minVal]-1] = arr[i]
		count[arr[i]-minVal]--
	}

	// 复制回原数组
	copy(arr, output)
	return arr
}

func main() {
	fmt.Println("====== 计数排序 (Counting Sort) ======")

	arr := []int{4, 2, 2, 8, 3, 3, 1}
	fmt.Println("原始数组:", arr)
	countingSort(arr)
	fmt.Println("排序后:", arr)

	// 稳定版本测试
	fmt.Println("\n--- 稳定版本 ---")
	arr2 := []int{4, 2, 2, 8, 3, 3, 1}
	fmt.Println("原始数组:", arr2)
	countingSortStable(arr2)
	fmt.Println("排序后:", arr2)

	// 包含负数
	fmt.Println("\n--- 包含负数 ---")
	arr3 := []int{-5, -10, 0, -3, 8, 5, -1, 10}
	fmt.Println("原始数组:", arr3)
	countingSort(arr3)
	fmt.Println("排序后:", arr3)

	// 大量重复元素
	fmt.Println("\n--- 大量重复元素 ---")
	arr4 := []int{1, 1, 1, 2, 2, 3, 3, 3, 3}
	fmt.Println("原始数组:", arr4)
	countingSort(arr4)
	fmt.Println("排序后:", arr4)
}
