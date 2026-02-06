/**
 * 堆排序 (Heap Sort)
 *
 * 算法原理：利用堆这种数据结构进行排序。先将数组构建成最大堆，
 * 然后将堆顶（最大值）与末尾元素交换，缩小堆的范围，重新调整堆。
 *
 * 时间复杂度：O(n log n) - 所有情况
 * 空间复杂度：O(1) - 原地排序
 * 稳定性：不稳定
 */

#include <iostream>
#include <vector>

/**
 * 堆化：将以i为根的子树调整为最大堆
 * @param arr 数组
 * @param n 堆的大小
 * @param i 要调整的节点索引
 */
void heapify(std::vector<int> &arr, int n, int i) {
  int largest = i;       // 假设根节点最大
  int left = 2 * i + 1;  // 左子节点
  int right = 2 * i + 2; // 右子节点

  // 如果左子节点大于根节点
  if (left < n && arr[left] > arr[largest]) {
    largest = left;
  }

  // 如果右子节点大于当前最大值
  if (right < n && arr[right] > arr[largest]) {
    largest = right;
  }

  // 如果最大值不是根节点，则交换并继续调整
  if (largest != i) {
    std::swap(arr[i], arr[largest]);
    heapify(arr, n, largest); // 递归调整被交换的子树
  }
}

/**
 * 堆排序函数
 */
void heapSort(std::vector<int> &arr) {
  int n = arr.size();

  // 步骤1：构建最大堆（从最后一个非叶子节点开始）
  for (int i = n / 2 - 1; i >= 0; i--) {
    heapify(arr, n, i);
  }

  // 步骤2：依次将堆顶元素（最大值）移到数组末尾
  for (int i = n - 1; i > 0; i--) {
    std::swap(arr[0], arr[i]); // 将当前最大值移到末尾
    heapify(arr, i, 0);        // 重新调整堆（范围缩小）
  }
}

void printArray(const std::vector<int> &arr, const std::string &msg) {
  std::cout << msg;
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;
}

int main() {
  std::cout << "========== 堆排序演示 ==========" << std::endl;
  std::vector<int> arr = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr, "排序前: ");
  heapSort(arr);
  printArray(arr, "排序后: ");

  std::vector<int> arr2 = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  printArray(arr2, "\n已排序数组: ");
  heapSort(arr2);
  printArray(arr2, "排序后: ");

  std::vector<int> arr3 = {10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
  printArray(arr3, "\n逆序数组: ");
  heapSort(arr3);
  printArray(arr3, "排序后: ");

  return 0;
}
