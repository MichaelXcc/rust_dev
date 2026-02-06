/**
 * 选择排序 (Selection Sort)
 *
 * 算法原理：
 * 选择排序是一种简单直观的排序算法。它的工作原理是：
 * 首先在未排序序列中找到最小（大）元素，存放到排序序列的起始位置，
 * 然后，再从剩余未排序元素中继续寻找最小（大）元素，
 * 然后放到已排序序列的末尾。以此类推，直到所有元素均排序完毕。
 *
 * 算法步骤：
 * 1. 在未排序序列中找到最小元素，与第一个位置的元素交换。
 * 2. 在剩余未排序序列中找到最小元素，与第二个位置的元素交换。
 * 3. 以此类推，直到所有元素都排列好。
 *
 * 时间复杂度：
 * - 最好情况：O(n²)
 * - 最坏情况：O(n²)
 * - 平均情况：O(n²)
 * 注：无论数组是否有序，都需要进行n(n-1)/2次比较
 *
 * 空间复杂度：O(1) - 只需要常数级别的额外空间
 *
 * 稳定性：不稳定 - 交换可能改变相等元素的相对顺序
 * 例如：[5a, 5b, 2] -> [2, 5b, 5a]，5a和5b的相对顺序改变了
 */

#include <iostream>
#include <vector>

/**
 * 选择排序函数
 * @param arr 待排序的数组（引用传递，直接修改原数组）
 */
void selectionSort(std::vector<int> &arr) {
  int n = arr.size();

  // 外层循环：控制当前要确定的位置
  // i表示当前要找最小值放置的位置
  for (int i = 0; i < n - 1; i++) {
    // 假设当前位置的元素就是最小值
    int minIndex = i;

    // 内层循环：在未排序部分中查找最小元素的索引
    for (int j = i + 1; j < n; j++) {
      // 如果找到更小的元素，更新最小值索引
      if (arr[j] < arr[minIndex]) {
        minIndex = j;
      }
    }

    // 如果最小元素不在当前位置，则交换
    // 这个判断可以减少不必要的交换操作
    if (minIndex != i) {
      std::swap(arr[i], arr[minIndex]);
    }
  }
}

/**
 * 双向选择排序（优化版本）
 * 每次循环同时找到最小值和最大值，分别放到两端
 * @param arr 待排序的数组
 */
void bidirectionalSelectionSort(std::vector<int> &arr) {
  int n = arr.size();
  int left = 0;      // 左边界
  int right = n - 1; // 右边界

  while (left < right) {
    int minIndex = left;
    int maxIndex = left;

    // 在未排序区间[left, right]中同时找最小和最大值
    for (int i = left; i <= right; i++) {
      if (arr[i] < arr[minIndex]) {
        minIndex = i;
      }
      if (arr[i] > arr[maxIndex]) {
        maxIndex = i;
      }
    }

    // 将最小值交换到左边界
    if (minIndex != left) {
      std::swap(arr[left], arr[minIndex]);
      // 如果最大值刚好在left位置，交换后需要更新maxIndex
      if (maxIndex == left) {
        maxIndex = minIndex;
      }
    }

    // 将最大值交换到右边界
    if (maxIndex != right) {
      std::swap(arr[right], arr[maxIndex]);
    }

    // 缩小未排序区间
    left++;
    right--;
  }
}

/**
 * 打印数组的辅助函数
 * @param arr 要打印的数组
 * @param message 打印前的提示信息
 */
void printArray(const std::vector<int> &arr, const std::string &message) {
  std::cout << message;
  for (int num : arr) {
    std::cout << num << " ";
  }
  std::cout << std::endl;
}

/**
 * 主函数 - 演示选择排序
 */
int main() {
  // 测试基本选择排序
  std::cout << "========== 选择排序演示 ==========" << std::endl;
  std::vector<int> arr1 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr1, "排序前: ");
  selectionSort(arr1);
  printArray(arr1, "排序后: ");

  // 测试双向选择排序
  std::cout << "\n========== 双向选择排序演示 ==========" << std::endl;
  std::vector<int> arr2 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr2, "排序前: ");
  bidirectionalSelectionSort(arr2);
  printArray(arr2, "排序后: ");

  // 测试已排序数组
  std::cout << "\n测试已排序数组：" << std::endl;
  std::vector<int> sorted = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  printArray(sorted, "排序前: ");
  selectionSort(sorted);
  printArray(sorted, "排序后: ");

  // 测试逆序数组
  std::cout << "\n测试逆序数组：" << std::endl;
  std::vector<int> reversed = {10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
  printArray(reversed, "排序前: ");
  selectionSort(reversed);
  printArray(reversed, "排序后: ");

  return 0;
}
