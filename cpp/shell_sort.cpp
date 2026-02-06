/**
 * 希尔排序 (Shell Sort)
 *
 * 算法原理：
 * 希尔排序是插入排序的一种改进版本，也称为"缩小增量排序"。
 * 其核心思想是将原始数组分成若干子序列，每个子序列的元素在原数组中间隔相同的增量gap，
 * 对每个子序列分别进行插入排序。随着算法的进行，增量逐渐减小，
 * 当增量减小到1时，整个数组就变成了一个子序列，最终完成排序。
 *
 * 为什么希尔排序比插入排序快？
 * 1. 插入排序对于几乎已排序的数据效率很高。
 * 2. 希尔排序通过先让间隔较大的元素基本有序，再让较小间隔的元素有序，
 *    最后当gap=1时，数组已经基本有序，插入排序效率很高。
 *
 * 常用的增量序列：
 * 1. Shell增量：N/2, N/4, ..., 1（希尔最初提出）
 * 2. Hibbard增量：2^k - 1, ..., 7, 3, 1
 * 3. Knuth增量：(3^k - 1) / 2, ..., 13, 4, 1
 * 4. Sedgewick增量：复杂公式计算
 *
 * 时间复杂度：
 * - 取决于增量序列的选择
 * - Shell增量：O(n²)
 * - Hibbard增量：O(n^1.5)
 * - Sedgewick增量：O(n^(4/3))
 *
 * 空间复杂度：O(1) - 原地排序
 *
 * 稳定性：不稳定 - 相同元素可能在不同子序列中被交换
 */

#include <iostream>
#include <vector>

/**
 * 希尔排序 - 使用Shell增量序列（N/2, N/4, ..., 1）
 * @param arr 待排序的数组
 */
void shellSort(std::vector<int> &arr) {
  int n = arr.size();

  // 初始增量为数组长度的一半，每次循环减半，直到增量为1
  for (int gap = n / 2; gap > 0; gap /= 2) {
    // 对每个子序列进行插入排序
    // 从gap位置开始，每个元素与其所在子序列的前面元素比较
    for (int i = gap; i < n; i++) {
      // 保存当前要插入的元素
      int temp = arr[i];
      int j = i;

      // 在当前子序列中进行插入排序
      // j >= gap 保证不越界
      // arr[j - gap] > temp 保证找到正确的插入位置
      while (j >= gap && arr[j - gap] > temp) {
        arr[j] = arr[j - gap]; // 元素后移
        j -= gap;              // 移动到子序列的前一个元素
      }

      // 插入元素到正确位置
      arr[j] = temp;
    }
  }
}

/**
 * 希尔排序 - 使用Knuth增量序列
 * Knuth增量：h = 3*h + 1，即 1, 4, 13, 40, 121, ...
 * @param arr 待排序的数组
 */
void shellSortKnuth(std::vector<int> &arr) {
  int n = arr.size();

  // 计算初始增量（Knuth序列）
  // 找到不超过n/3的最大增量
  int gap = 1;
  while (gap < n / 3) {
    gap = 3 * gap + 1; // 1, 4, 13, 40, 121, ...
  }

  // 从最大增量开始，逐步缩小
  while (gap >= 1) {
    // 对每个子序列进行插入排序
    for (int i = gap; i < n; i++) {
      int temp = arr[i];
      int j = i;

      while (j >= gap && arr[j - gap] > temp) {
        arr[j] = arr[j - gap];
        j -= gap;
      }
      arr[j] = temp;
    }

    // 缩小增量：h = (h - 1) / 3
    gap = gap / 3;
  }
}

/**
 * 希尔排序 - 使用Hibbard增量序列
 * Hibbard增量：2^k - 1，即 1, 3, 7, 15, 31, ...
 * @param arr 待排序的数组
 */
void shellSortHibbard(std::vector<int> &arr) {
  int n = arr.size();

  // 计算初始增量（Hibbard序列）
  // 找到不超过n的最大 2^k - 1
  int k = 1;
  while ((1 << k) - 1 < n) {
    k++;
  }
  k--; // 回退一步，确保增量不超过n

  // 从最大增量开始
  while (k >= 1) {
    int gap = (1 << k) - 1; // 2^k - 1

    for (int i = gap; i < n; i++) {
      int temp = arr[i];
      int j = i;

      while (j >= gap && arr[j - gap] > temp) {
        arr[j] = arr[j - gap];
        j -= gap;
      }
      arr[j] = temp;
    }

    k--;
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
 * 演示希尔排序的过程
 */
void shellSortWithSteps(std::vector<int> &arr) {
  int n = arr.size();

  std::cout << "初始数组: ";
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;

  for (int gap = n / 2; gap > 0; gap /= 2) {
    std::cout << "\n增量 gap = " << gap << "：" << std::endl;

    for (int i = gap; i < n; i++) {
      int temp = arr[i];
      int j = i;

      while (j >= gap && arr[j - gap] > temp) {
        arr[j] = arr[j - gap];
        j -= gap;
      }
      arr[j] = temp;
    }

    std::cout << "排序结果: ";
    for (int num : arr)
      std::cout << num << " ";
    std::cout << std::endl;
  }
}

/**
 * 主函数 - 演示希尔排序
 */
int main() {
  // Shell增量序列
  std::cout << "========== 希尔排序（Shell增量）==========" << std::endl;
  std::vector<int> arr1 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr1, "排序前: ");
  shellSort(arr1);
  printArray(arr1, "排序后: ");

  // Knuth增量序列
  std::cout << "\n========== 希尔排序（Knuth增量）==========" << std::endl;
  std::vector<int> arr2 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr2, "排序前: ");
  shellSortKnuth(arr2);
  printArray(arr2, "排序后: ");

  // Hibbard增量序列
  std::cout << "\n========== 希尔排序（Hibbard增量）==========" << std::endl;
  std::vector<int> arr3 = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr3, "排序前: ");
  shellSortHibbard(arr3);
  printArray(arr3, "排序后: ");

  // 展示排序过程
  std::cout << "\n========== 希尔排序过程展示 ==========" << std::endl;
  std::vector<int> arr4 = {8, 9, 1, 7, 2, 3, 5, 4, 6, 0};
  shellSortWithSteps(arr4);

  return 0;
}
