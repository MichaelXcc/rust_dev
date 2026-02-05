// Package main 实现希尔排序算法
// 希尔排序 (Shell Sort) 是插入排序的一种改进版本，也称为"缩小增量排序"
//
// 算法原理：
//  1. 选择一个增量序列 gap1, gap2, ..., gapk，其中 gap1 > gap2 > ... > gapk = 1
//  2. 按增量序列的个数 k，对序列进行 k 趟排序
//  3. 每趟排序，根据对应的增量 gap，将待排序列分割成若干长度为 m 的子序列，
//     分别对各子表进行直接插入排序
//  4. 当增量 gap = 1 时，整个序列作为一个表来处理，排序完成
//
// 为什么希尔排序更快？
// - 当数据基本有序时，插入排序效率很高
// - 希尔排序通过大增量使数据逐步变得基本有序
// - 最后增量为1时的插入排序效率很高
//
// 时间复杂度：取决于增量序列的选择
// - 最坏情况：O(n²)（使用原始希尔增量）
// - 使用 Hibbard 增量：O(n^1.5)
// - 使用 Sedgewick 增量：O(n^1.3)
//
// 空间复杂度：O(1) - 只需要常数级别的额外空间
//
// 稳定性：不稳定排序（相等元素可能因为不同子序列的排序而改变相对位置）
package main

import "fmt"

// shellSort 希尔排序函数（使用 Shell 原始增量序列：n/2, n/4, ..., 1）
// 参数：arr - 待排序的整数切片
// 返回：排序后的切片（原地排序，返回的是同一个切片）
func shellSort(arr []int) []int {
	n := len(arr)

	// gap 为增量，初始值为数组长度的一半
	// 每次循环后 gap 减半，直到 gap = 1
	for gap := n / 2; gap > 0; gap /= 2 {
		// 对每个子序列进行插入排序
		// 从 gap 位置开始，对每个元素进行插入排序
		for i := gap; i < n; i++ {
			// 保存当前要插入的元素
			temp := arr[i]

			// j 从当前位置开始，向前以 gap 为步长进行比较
			j := i

			// 在子序列中进行插入排序
			// 比较 arr[j-gap] 和 temp，如果前者大，则后移
			for j >= gap && arr[j-gap] > temp {
				arr[j] = arr[j-gap]
				j -= gap
			}

			// 将 temp 插入到正确的位置
			arr[j] = temp
		}
	}

	return arr
}

// shellSortHibbard 使用 Hibbard 增量序列的希尔排序
// Hibbard 增量序列：1, 3, 7, 15, 31, ... (2^k - 1)
// 时间复杂度可以达到 O(n^1.5)
func shellSortHibbard(arr []int) []int {
	n := len(arr)

	// 计算初始增量（最大的 2^k - 1 且小于 n）
	gap := 1
	for gap < n/3 {
		gap = gap*2 + 1
	}

	// 按照 Hibbard 增量序列进行排序
	for gap >= 1 {
		for i := gap; i < n; i++ {
			temp := arr[i]
			j := i

			for j >= gap && arr[j-gap] > temp {
				arr[j] = arr[j-gap]
				j -= gap
			}

			arr[j] = temp
		}

		// Hibbard 增量的递减方式
		gap = (gap - 1) / 2
	}

	return arr
}

// shellSortSedgewick 使用 Sedgewick 增量序列的希尔排序
// Sedgewick 增量序列：1, 5, 19, 41, 109, ...
// 公式：9 * 4^k - 9 * 2^k + 1 或 4^k - 3 * 2^k + 1
// 时间复杂度可以达到 O(n^1.3)
func shellSortSedgewick(arr []int) []int {
	n := len(arr)

	// 预计算 Sedgewick 增量序列
	sedgewickSequence := []int{1}
	k := 1
	for {
		var gap int
		if k%2 == 0 {
			gap = 9*(1<<k) - 9*(1<<(k/2)) + 1
		} else {
			gap = 8*(1<<k) - 6*(1<<((k+1)/2)) + 1
		}
		if gap >= n {
			break
		}
		sedgewickSequence = append(sedgewickSequence, gap)
		k++
	}

	// 从最大的增量开始排序
	for i := len(sedgewickSequence) - 1; i >= 0; i-- {
		gap := sedgewickSequence[i]

		for j := gap; j < n; j++ {
			temp := arr[j]
			l := j

			for l >= gap && arr[l-gap] > temp {
				arr[l] = arr[l-gap]
				l -= gap
			}

			arr[l] = temp
		}
	}

	return arr
}

// shellSortWithSteps 带步骤输出的希尔排序（用于演示过程）
func shellSortWithSteps(arr []int) []int {
	n := len(arr)

	fmt.Println("开始希尔排序过程：")
	fmt.Printf("初始数组: %v\n", arr)

	round := 1
	for gap := n / 2; gap > 0; gap /= 2 {
		fmt.Printf("\n第 %d 轮：增量 gap = %d\n", round, gap)

		for i := gap; i < n; i++ {
			temp := arr[i]
			j := i

			for j >= gap && arr[j-gap] > temp {
				arr[j] = arr[j-gap]
				j -= gap
			}

			arr[j] = temp
		}

		fmt.Printf("本轮结束后: %v\n", arr)
		round++
	}

	return arr
}

// main 函数：程序入口，演示希尔排序的使用
func main() {
	// 创建一个无序的整数切片
	arr := []int{64, 34, 25, 12, 22, 11, 90}

	fmt.Println("====== 希尔排序 (Shell Sort) ======")
	fmt.Println("原始数组:", arr)

	// 调用希尔排序函数
	shellSort(arr)

	fmt.Println("排序后:", arr)

	// 演示排序过程
	fmt.Println("\n====== 演示排序过程 ======")
	arr2 := []int{8, 9, 1, 7, 2, 3, 5, 4, 6, 0}
	shellSortWithSteps(arr2)

	// 测试 Hibbard 增量序列
	fmt.Println("\n====== Hibbard 增量序列 ======")
	arr3 := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr3)
	shellSortHibbard(arr3)
	fmt.Println("排序后:", arr3)

	// 测试 Sedgewick 增量序列
	fmt.Println("\n====== Sedgewick 增量序列 ======")
	arr4 := []int{64, 34, 25, 12, 22, 11, 90}
	fmt.Println("原始数组:", arr4)
	shellSortSedgewick(arr4)
	fmt.Println("排序后:", arr4)

	// 测试大数组
	fmt.Println("\n--- 测试较大数组 ---")
	largeArr := []int{35, 33, 42, 10, 14, 19, 27, 44, 26, 31,
		28, 36, 11, 24, 18, 37, 25, 15, 29, 23}
	fmt.Println("原始数组:", largeArr)
	shellSort(largeArr)
	fmt.Println("排序后:", largeArr)

	// 测试包含重复元素的数组
	fmt.Println("\n--- 测试包含重复元素的数组 ---")
	duplicateArr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	fmt.Println("原始数组:", duplicateArr)
	shellSort(duplicateArr)
	fmt.Println("排序后:", duplicateArr)
}
