/**
 * 归并排序 (Merge Sort)
 *
 * 算法原理：
 * 归并排序采用分治法（Divide and Conquer）的思想。
 * 将数组不断地一分为二，直到每个子数组只有一个元素（自然有序），
 * 然后将有序的子数组两两合并，最终得到完全有序的数组。
 *
 * 算法步骤：
 * 1. 分解：将数组从中间分成两个子数组。
 * 2. 递归：对两个子数组分别进行归并排序。
 * 3. 合并：将两个已排序的子数组合并成一个有序数组。
 *
 * 时间复杂度：
 * - 最好情况：O(n log n)
 * - 最坏情况：O(n log n)
 * - 平均情况：O(n log n)
 * 注：归并排序的时间复杂度在任何情况下都是O(n log n)
 *
 * 空间复杂度：O(n) - 需要额外的数组空间来合并
 *
 * 稳定性：稳定 - 合并时保持相等元素的相对顺序
 *
 * 特点：
 * - 时间复杂度稳定，不受输入数据的影响
 * - 适合处理链表排序（可以做到O(1)空间复杂度）
 * - 适合外部排序（处理无法一次性加载到内存的大数据）
 */

#include <iostream>
#include <vector>

/**
 * 合并两个有序子数组
 * @param arr 原数组
 * @param left 左边界
 * @param mid 中间位置
 * @param right 右边界
 *
 * 将 arr[left..mid] 和 arr[mid+1..right] 合并成一个有序数组
 */
void merge(std::vector<int> &arr, int left, int mid, int right) {
  // 计算两个子数组的大小
  int n1 = mid - left + 1; // 左子数组大小
  int n2 = right - mid;    // 右子数组大小

  // 创建临时数组
  std::vector<int> leftArr(n1);
  std::vector<int> rightArr(n2);

  // 复制数据到临时数组
  for (int i = 0; i < n1; i++) {
    leftArr[i] = arr[left + i];
  }
  for (int j = 0; j < n2; j++) {
    rightArr[j] = arr[mid + 1 + j];
  }

  // 合并临时数组回原数组
  int i = 0;    // 左子数组的索引
  int j = 0;    // 右子数组的索引
  int k = left; // 原数组的索引

  // 比较两个子数组的元素，将较小的放入原数组
  while (i < n1 && j < n2) {
    // 使用 <= 保证稳定性：相等时优先取左边的元素
    if (leftArr[i] <= rightArr[j]) {
      arr[k] = leftArr[i];
      i++;
    } else {
      arr[k] = rightArr[j];
      j++;
    }
    k++;
  }

  // 复制左子数组剩余的元素（如果有）
  while (i < n1) {
    arr[k] = leftArr[i];
    i++;
    k++;
  }

  // 复制右子数组剩余的元素（如果有）
  while (j < n2) {
    arr[k] = rightArr[j];
    j++;
    k++;
  }
}

/**
 * 归并排序（递归实现）
 * @param arr 待排序的数组
 * @param left 左边界
 * @param right 右边界
 */
void mergeSortRecursive(std::vector<int> &arr, int left, int right) {
  // 递归终止条件：只有一个元素或没有元素
  if (left >= right) {
    return;
  }

  // 计算中间位置（避免溢出的写法）
  int mid = left + (right - left) / 2;

  // 递归排序左半部分
  mergeSortRecursive(arr, left, mid);

  // 递归排序右半部分
  mergeSortRecursive(arr, mid + 1, right);

  // 合并两个有序部分
  merge(arr, left, mid, right);
}

/**
 * 归并排序入口函数
 * @param arr 待排序的数组
 */
void mergeSort(std::vector<int> &arr) {
  if (arr.empty())
    return;
  mergeSortRecursive(arr, 0, arr.size() - 1);
}

/**
 * 归并排序（迭代/自底向上实现）
 * 避免递归带来的函数调用开销
 * @param arr 待排序的数组
 */
void mergeSortIterative(std::vector<int> &arr) {
  int n = arr.size();
  if (n <= 1)
    return;

  // size表示当前要合并的子数组大小
  // 从1开始，每次翻倍：1, 2, 4, 8, ...
  for (int size = 1; size < n; size *= 2) {
    // 遍历所有需要合并的子数组对
    for (int left = 0; left < n - size; left += 2 * size) {
      int mid = left + size - 1;
      // 右边界取较小值，防止越界
      int right = std::min(left + 2 * size - 1, n - 1);

      // 合并 arr[left..mid] 和 arr[mid+1..right]
      merge(arr, left, mid, right);
    }
  }
}

/**
 * 打印数组的辅助函数
 */
void printArray(const std::vector<int> &arr, const std::string &message) {
  std::cout << message;
  for (int num : arr) {
    std::cout << num << " ";
  }
  std::cout << std::endl;
}

/**
 * 演示归并排序的过程
 */
void mergeSortWithSteps(std::vector<int> &arr, int left, int right,
                        int depth = 0) {
  std::string indent(depth * 2, ' ');

  if (left >= right) {
    std::cout << indent << "叶子节点: [" << arr[left] << "]" << std::endl;
    return;
  }

  int mid = left + (right - left) / 2;

  std::cout << indent << "分解: ";
  for (int i = left; i <= right; i++)
    std::cout << arr[i] << " ";
  std::cout << std::endl;

  mergeSortWithSteps(arr, left, mid, depth + 1);
  mergeSortWithSteps(arr, mid + 1, right, depth + 1);

  merge(arr, left, mid, right);

  std::cout << indent << "合并: ";
  for (int i = left; i <= right; i++)
    std::cout << arr[i] << " ";
  std::cout << std::endl;
}

/**
 * 主函数 - 演示归并排序
 */
int main() {
  // 递归实现
  std::cout << "========== 归并排序（递归实现）==========" << std::endl;
  std::vector<int> arr1 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr1, "排序前: ");
  mergeSort(arr1);
  printArray(arr1, "排序后: ");

  // 迭代实现
  std::cout << "\n========== 归并排序（迭代实现）==========" << std::endl;
  std::vector<int> arr2 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr2, "排序前: ");
  mergeSortIterative(arr2);
  printArray(arr2, "排序后: ");

  // 展示排序过程
  std::cout << "\n========== 归并排序过程展示 ==========" << std::endl;
  std::vector<int> arr3 = {38, 27, 43, 3, 9, 82, 10};
  mergeSortWithSteps(arr3, 0, arr3.size() - 1);

  // 测试大数组
  std::cout << "\n测试大数组排序：" << std::endl;
  std::vector<int> large;
  for (int i = 100; i > 0; i--) {
    large.push_back(i);
  }
  std::cout << "排序100个逆序元素..." << std::endl;
  mergeSort(large);
  std::cout << "前10个元素: ";
  for (int i = 0; i < 10; i++) {
    std::cout << large[i] << " ";
  }
  std::cout << "... 后10个元素: ";
  for (int i = 90; i < 100; i++) {
    std::cout << large[i] << " ";
  }
  std::cout << std::endl;

  return 0;
}
