// Package main 实现堆排序算法
// 堆排序 (Heap Sort) 利用堆这种数据结构设计的一种排序算法
//
// 堆的定义：
// - 最大堆：父节点的值总是大于或等于其子节点的值
// - 最小堆：父节点的值总是小于或等于其子节点的值
//
// 算法原理：
// 1. 构建最大堆：将无序数组构建成最大堆
// 2. 交换堆顶与末尾：将堆顶（最大值）与堆末尾元素交换
// 3. 调整堆：将剩余元素重新调整为最大堆
// 4. 重复：重复步骤2-3直到堆大小为1
//
// 时间复杂度：O(n log n) - 所有情况都是
// 空间复杂度：O(1) - 原地排序
// 稳定性：不稳定
package main

import "fmt"

// heapSort 堆排序主函数
func heapSort(arr []int) {
	n := len(arr)

	// 构建最大堆（从最后一个非叶子节点开始）
	// 最后一个非叶子节点的索引是 n/2 - 1
	for i := n/2 - 1; i >= 0; i-- {
		heapify(arr, n, i)
	}

	// 逐个提取堆顶元素
	for i := n - 1; i > 0; i-- {
		// 将堆顶（最大值）与当前末尾交换
		arr[0], arr[i] = arr[i], arr[0]
		// 对剩余元素重新调整为最大堆
		heapify(arr, i, 0)
	}
}

// heapify 调整堆，使以 i 为根的子树成为最大堆
// arr: 待调整的数组
// n: 堆的大小
// i: 根节点索引
func heapify(arr []int, n, i int) {
	largest := i     // 初始化最大值为根节点
	left := 2*i + 1  // 左子节点索引
	right := 2*i + 2 // 右子节点索引

	// 如果左子节点存在且大于根节点
	if left < n && arr[left] > arr[largest] {
		largest = left
	}

	// 如果右子节点存在且大于当前最大值
	if right < n && arr[right] > arr[largest] {
		largest = right
	}

	// 如果最大值不是根节点，则交换并继续调整
	if largest != i {
		arr[i], arr[largest] = arr[largest], arr[i]
		// 递归调整被影响的子树
		heapify(arr, n, largest)
	}
}

// heapSortWithSteps 带步骤输出的堆排序
func heapSortWithSteps(arr []int) {
	n := len(arr)
	fmt.Printf("初始数组: %v\n", arr)

	// 构建最大堆
	fmt.Println("\n--- 构建最大堆 ---")
	for i := n/2 - 1; i >= 0; i-- {
		heapify(arr, n, i)
		fmt.Printf("调整节点 %d 后: %v\n", i, arr)
	}

	// 逐个提取
	fmt.Println("\n--- 提取元素 ---")
	for i := n - 1; i > 0; i-- {
		arr[0], arr[i] = arr[i], arr[0]
		heapify(arr, i, 0)
		fmt.Printf("提取第 %d 大元素后: %v\n", n-i, arr)
	}
}

func main() {
	fmt.Println("====== 堆排序 (Heap Sort) ======")

	arr := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr)
	heapSort(arr)
	fmt.Println("排序后:", arr)

	// 演示排序过程
	fmt.Println("\n====== 演示排序过程 ======")
	arr2 := []int{4, 10, 3, 5, 1}
	heapSortWithSteps(arr2)

	// 测试逆序数组
	fmt.Println("\n--- 测试逆序数组 ---")
	arr3 := []int{7, 6, 5, 4, 3, 2, 1}
	fmt.Println("原始数组:", arr3)
	heapSort(arr3)
	fmt.Println("排序后:", arr3)

	// 测试重复元素
	fmt.Println("\n--- 测试重复元素 ---")
	arr4 := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", arr4)
	heapSort(arr4)
	fmt.Println("排序后:", arr4)
}
