// Package main 实现桶排序算法
// 桶排序 (Bucket Sort) 是一种分布式排序算法
//
// 算法原理：
// 1. 设置一定数量的桶（空桶）
// 2. 遍历数组，将每个元素放入对应的桶中
// 3. 对每个非空桶进行排序（可以使用其他排序算法）
// 4. 按顺序遍历所有桶，将元素放回原数组
//
// 适用场景：
// - 数据均匀分布
// - 数据在一个可预知的范围内
//
// 时间复杂度：
// - 平均：O(n + k)，k 是桶的数量
// - 最坏：O(n²)（所有元素落入同一个桶）
// 空间复杂度：O(n + k)
// 稳定性：取决于桶内排序算法
package main

import (
	"fmt"
	"sort"
)

// bucketSort 桶排序（适用于浮点数 0~1 范围）
func bucketSort(arr []float64) []float64 {
	n := len(arr)
	if n <= 1 {
		return arr
	}

	// 创建 n 个空桶
	buckets := make([][]float64, n)
	for i := range buckets {
		buckets[i] = make([]float64, 0)
	}

	// 将元素分配到桶中
	for _, v := range arr {
		// 计算桶索引（假设数据在 [0, 1) 范围内）
		idx := int(v * float64(n))
		if idx >= n {
			idx = n - 1
		}
		buckets[idx] = append(buckets[idx], v)
	}

	// 对每个桶内的元素排序
	for i := range buckets {
		sort.Float64s(buckets[i])
	}

	// 合并所有桶
	index := 0
	for _, bucket := range buckets {
		for _, v := range bucket {
			arr[index] = v
			index++
		}
	}

	return arr
}

// bucketSortInt 整数桶排序
func bucketSortInt(arr []int, bucketSize int) []int {
	if len(arr) <= 1 {
		return arr
	}

	// 找最大最小值
	minVal, maxVal := arr[0], arr[0]
	for _, v := range arr {
		if v < minVal {
			minVal = v
		}
		if v > maxVal {
			maxVal = v
		}
	}

	// 计算桶数量
	bucketCount := (maxVal-minVal)/bucketSize + 1
	buckets := make([][]int, bucketCount)
	for i := range buckets {
		buckets[i] = make([]int, 0)
	}

	// 分配到桶
	for _, v := range arr {
		idx := (v - minVal) / bucketSize
		buckets[idx] = append(buckets[idx], v)
	}

	// 桶内排序并合并
	index := 0
	for _, bucket := range buckets {
		sort.Ints(bucket)
		for _, v := range bucket {
			arr[index] = v
			index++
		}
	}

	return arr
}

func main() {
	fmt.Println("====== 桶排序 (Bucket Sort) ======")

	// 浮点数桶排序
	arr := []float64{0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51}
	fmt.Println("原始数组:", arr)
	bucketSort(arr)
	fmt.Println("排序后:", arr)

	// 整数桶排序
	fmt.Println("\n--- 整数桶排序 ---")
	arr2 := []int{29, 25, 3, 49, 9, 37, 21, 43}
	fmt.Println("原始数组:", arr2)
	bucketSortInt(arr2, 10)
	fmt.Println("排序后:", arr2)

	// 大范围数据
	fmt.Println("\n--- 大范围数据 ---")
	arr3 := []int{64, 34, 25, 12, 22, 11, 90, 100, 5, 88}
	fmt.Println("原始数组:", arr3)
	bucketSortInt(arr3, 20)
	fmt.Println("排序后:", arr3)
}
