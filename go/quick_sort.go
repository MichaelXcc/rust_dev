// Package main 实现快速排序算法
// 快速排序 (Quick Sort) 是一种高效的排序算法，采用分治法策略
//
// 时间复杂度：平均 O(n log n)，最坏 O(n²)
// 空间复杂度：O(log n) 递归栈
// 稳定性：不稳定
package main

import (
	"fmt"
	"math/rand"
	"time"
)

// quickSort 快速排序主函数
func quickSort(arr []int) {
	if len(arr) <= 1 {
		return
	}
	quickSortHelper(arr, 0, len(arr)-1)
}

// quickSortHelper 快速排序辅助函数（递归）
func quickSortHelper(arr []int, low, high int) {
	if low < high {
		pivotIndex := partition(arr, low, high)
		quickSortHelper(arr, low, pivotIndex-1)
		quickSortHelper(arr, pivotIndex+1, high)
	}
}

// partition 分区函数 - Lomuto 方案
func partition(arr []int, low, high int) int {
	pivot := arr[high]
	i := low - 1
	for j := low; j < high; j++ {
		if arr[j] < pivot {
			i++
			arr[i], arr[j] = arr[j], arr[i]
		}
	}
	arr[i+1], arr[high] = arr[high], arr[i+1]
	return i + 1
}

// quickSort3Way 三路快速排序（适用于大量重复元素）
func quickSort3Way(arr []int, low, high int) {
	if low >= high {
		return
	}
	lt, gt, i := low, high, low+1
	pivot := arr[low]
	for i <= gt {
		if arr[i] < pivot {
			arr[lt], arr[i] = arr[i], arr[lt]
			lt++
			i++
		} else if arr[i] > pivot {
			arr[gt], arr[i] = arr[i], arr[gt]
			gt--
		} else {
			i++
		}
	}
	quickSort3Way(arr, low, lt-1)
	quickSort3Way(arr, gt+1, high)
}

// quickSortRandom 随机基准快速排序
func quickSortRandom(arr []int, low, high int) {
	if low < high {
		rand.Seed(time.Now().UnixNano())
		randIdx := low + rand.Intn(high-low+1)
		arr[randIdx], arr[high] = arr[high], arr[randIdx]
		pivotIndex := partition(arr, low, high)
		quickSortRandom(arr, low, pivotIndex-1)
		quickSortRandom(arr, pivotIndex+1, high)
	}
}

func main() {
	fmt.Println("====== 快速排序 (Quick Sort) ======")

	arr := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr)
	quickSort(arr)
	fmt.Println("排序后:", arr)

	// 三路快排测试
	fmt.Println("\n--- 三路快速排序 ---")
	arr2 := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", arr2)
	quickSort3Way(arr2, 0, len(arr2)-1)
	fmt.Println("排序后:", arr2)
}
