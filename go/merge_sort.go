// Package main 实现归并排序算法
// 归并排序 (Merge Sort) 是建立在归并操作上的一种有效的排序算法
// 该算法是采用分治法 (Divide and Conquer) 的一个非常典型的应用
//
// 算法原理（分治思想）：
// 1. 分解（Divide）：将待排序的 n 个元素的序列分成各含 n/2 个元素的两个子序列
// 2. 解决（Conquer）：使用归并排序递归地排序两个子序列
// 3. 合并（Combine）：合并两个已排序的子序列以产生已排序的答案
//
// 归并操作（关键步骤）：
// - 申请空间，使其大小为两个已经排序序列之和
// - 设定两个指针，最初位置分别为两个已经排序序列的起始位置
// - 比较两个指针所指向的元素，选择相对小的元素放入到合并空间，并移动指针到下一位置
// - 重复上述步骤直到某一指针达到序列尾
// - 将另一序列剩下的所有元素直接复制到合并序列尾
//
// 时间复杂度：
// - 最好情况：O(n log n)
// - 最坏情况：O(n log n)
// - 平均情况：O(n log n)
// 归并排序的时间复杂度非常稳定，不受输入数据的影响
//
// 空间复杂度：O(n) - 需要额外的空间来存储合并后的结果
//
// 稳定性：稳定排序（相等元素的相对位置不会改变）
package main

import "fmt"

// mergeSort 归并排序函数（递归实现）
// 参数：arr - 待排序的整数切片
// 返回：排序后的新切片
func mergeSort(arr []int) []int {
	n := len(arr)

	// 基础情况：如果数组长度小于等于1，直接返回
	if n <= 1 {
		return arr
	}

	// 找到中间点，将数组分成两半
	mid := n / 2

	// 递归排序左半部分
	left := mergeSort(arr[:mid])

	// 递归排序右半部分
	right := mergeSort(arr[mid:])

	// 合并两个已排序的部分
	return merge(left, right)
}

// merge 合并两个已排序的切片
// 参数：left - 左半部分已排序切片，right - 右半部分已排序切片
// 返回：合并后的已排序切片
func merge(left, right []int) []int {
	// 创建结果切片，容量为两个切片长度之和
	result := make([]int, 0, len(left)+len(right))

	// i 和 j 分别为 left 和 right 的索引
	i, j := 0, 0

	// 比较两个切片的元素，按顺序放入结果切片
	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			// 使用 <= 而不是 < 来保证稳定性
			result = append(result, left[i])
			i++
		} else {
			result = append(result, right[j])
			j++
		}
	}

	// 将左切片剩余的元素添加到结果中
	result = append(result, left[i:]...)

	// 将右切片剩余的元素添加到结果中
	result = append(result, right[j:]...)

	return result
}

// mergeSortInPlace 原地归并排序（减少空间分配）
// 参数：arr - 待排序的整数切片，temp - 临时存储切片
// 注意：这个版本虽然减少了空间分配，但仍然需要 O(n) 的辅助空间
func mergeSortInPlace(arr []int) {
	n := len(arr)
	if n <= 1 {
		return
	}

	// 创建一个临时数组
	temp := make([]int, n)

	// 调用辅助函数
	mergeSortHelper(arr, temp, 0, n-1)
}

// mergeSortHelper 归并排序辅助函数
func mergeSortHelper(arr, temp []int, left, right int) {
	if left >= right {
		return
	}

	mid := left + (right-left)/2

	// 递归排序左半部分
	mergeSortHelper(arr, temp, left, mid)

	// 递归排序右半部分
	mergeSortHelper(arr, temp, mid+1, right)

	// 合并两部分
	mergeInPlace(arr, temp, left, mid, right)
}

// mergeInPlace 原地合并函数
func mergeInPlace(arr, temp []int, left, mid, right int) {
	// 将数据复制到临时数组
	for i := left; i <= right; i++ {
		temp[i] = arr[i]
	}

	i, j := left, mid+1
	k := left

	// 合并回原数组
	for i <= mid && j <= right {
		if temp[i] <= temp[j] {
			arr[k] = temp[i]
			i++
		} else {
			arr[k] = temp[j]
			j++
		}
		k++
	}

	// 复制左边剩余元素
	for i <= mid {
		arr[k] = temp[i]
		i++
		k++
	}
	// 右边剩余元素已经在正确位置，无需复制
}

// mergeSortBottomUp 自底向上的归并排序（迭代实现）
// 这种实现避免了递归调用的开销
func mergeSortBottomUp(arr []int) {
	n := len(arr)
	if n <= 1 {
		return
	}

	temp := make([]int, n)

	// size 表示当前归并的子数组大小
	// 从 1 开始，每次翻倍
	for size := 1; size < n; size *= 2 {
		// 遍历所有大小为 size 的子数组对
		for left := 0; left < n-size; left += 2 * size {
			mid := left + size - 1
			// right 不能超过数组边界
			right := min(left+2*size-1, n-1)

			mergeInPlace(arr, temp, left, mid, right)
		}
	}
}

// min 返回两个整数中的较小值
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// mergeSortWithSteps 带步骤输出的归并排序（用于演示过程）
func mergeSortWithSteps(arr []int, depth int) []int {
	indent := ""
	for i := 0; i < depth; i++ {
		indent += "  "
	}

	n := len(arr)
	fmt.Printf("%s分解: %v\n", indent, arr)

	if n <= 1 {
		fmt.Printf("%s返回: %v (基础情况)\n", indent, arr)
		return arr
	}

	mid := n / 2

	left := mergeSortWithSteps(arr[:mid], depth+1)
	right := mergeSortWithSteps(arr[mid:], depth+1)

	merged := merge(left, right)
	fmt.Printf("%s合并 %v 和 %v -> %v\n", indent, left, right, merged)

	return merged
}

// main 函数：程序入口，演示归并排序的使用
func main() {
	// 创建一个无序的整数切片
	arr := []int{64, 34, 25, 12, 22, 11, 90}

	fmt.Println("====== 归并排序 (Merge Sort) ======")
	fmt.Println("原始数组:", arr)

	// 调用归并排序函数
	sorted := mergeSort(arr)

	fmt.Println("排序后:", sorted)

	// 演示排序过程
	fmt.Println("\n====== 演示排序过程 ======")
	arr2 := []int{38, 27, 43, 3, 9, 82, 10}
	fmt.Println("原始数组:", arr2)
	result := mergeSortWithSteps(arr2, 0)
	fmt.Println("最终结果:", result)

	// 测试原地归并排序
	fmt.Println("\n====== 原地归并排序 ======")
	arr3 := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr3)
	mergeSortInPlace(arr3)
	fmt.Println("排序后:", arr3)

	// 测试自底向上的归并排序
	fmt.Println("\n====== 自底向上归并排序 ======")
	arr4 := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr4)
	mergeSortBottomUp(arr4)
	fmt.Println("排序后:", arr4)

	// 测试已排序数组
	fmt.Println("\n--- 测试已排序数组 ---")
	sortedArr := []int{1, 2, 3, 4, 5, 6, 7}
	fmt.Println("原始数组:", sortedArr)
	result = mergeSort(sortedArr)
	fmt.Println("排序后:", result)

	// 测试逆序数组
	fmt.Println("\n--- 测试逆序数组 ---")
	reversedArr := []int{7, 6, 5, 4, 3, 2, 1}
	fmt.Println("原始数组:", reversedArr)
	result = mergeSort(reversedArr)
	fmt.Println("排序后:", result)

	// 测试包含重复元素的数组
	fmt.Println("\n--- 测试包含重复元素的数组 ---")
	duplicateArr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", duplicateArr)
	result = mergeSort(duplicateArr)
	fmt.Println("排序后:", result)
}
