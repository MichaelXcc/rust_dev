/**
 * 插入排序 (Insertion Sort)
 *
 * 算法原理：
 * 插入排序的工作方式像是整理扑克牌。我们将数组分成"已排序"和"未排序"两部分，
 * 每次从未排序部分取出第一个元素，插入到已排序部分的适当位置。
 *
 * 算法步骤：
 * 1. 将第一个元素视为已排序部分。
 * 2. 取出下一个元素，在已排序部分从后向前扫描。
 * 3. 如果已排序的元素大于新元素，将该元素后移一位。
 * 4. 重复步骤3，直到找到已排序的元素小于或等于新元素的位置。
 * 5. 将新元素插入到该位置。
 * 6. 重复步骤2~5，直到所有元素都排序完毕。
 *
 * 时间复杂度：
 * - 最好情况：O(n) - 当数组已经有序时
 * - 最坏情况：O(n²) - 当数组逆序时
 * - 平均情况：O(n²)
 *
 * 空间复杂度：O(1) - 只需要常数级别的额外空间
 *
 * 稳定性：稳定 - 相等元素的相对顺序不会改变
 *
 * 适用场景：
 * - 小规模数据排序
 * - 数据基本有序的情况
 * - 作为其他高级排序算法的子过程（如希尔排序、Tim排序）
 */

#include <iostream>
#include <vector>

/**
 * 插入排序函数（基本版本）
 * @param arr 待排序的数组（引用传递，直接修改原数组）
 */
void insertionSort(std::vector<int> &arr) {
  int n = arr.size();

  // 从第二个元素开始，第一个元素默认已排序
  for (int i = 1; i < n; i++) {
    // 保存当前要插入的元素
    int key = arr[i];

    // j指向已排序部分的最后一个元素
    int j = i - 1;

    // 在已排序部分从后向前查找插入位置
    // 同时将大于key的元素向后移动
    while (j >= 0 && arr[j] > key) {
      arr[j + 1] = arr[j]; // 元素后移
      j--;                 // 继续向前比较
    }

    // 找到插入位置，将key插入
    // j+1 是因为循环结束时j已经多减了1
    arr[j + 1] = key;
  }
}

/**
 * 二分插入排序（优化版本）
 * 使用二分查找来定位插入位置，减少比较次数
 * 注意：虽然比较次数减少了，但移动次数不变，时间复杂度仍为O(n²)
 * @param arr 待排序的数组
 */
void binaryInsertionSort(std::vector<int> &arr) {
  int n = arr.size();

  for (int i = 1; i < n; i++) {
    int key = arr[i];

    // 使用二分查找定位插入位置
    int left = 0;
    int right = i - 1;

    // 二分查找：找到第一个大于key的位置
    while (left <= right) {
      int mid = left + (right - left) / 2;
      if (arr[mid] > key) {
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }

    // left现在就是key应该插入的位置
    // 将[left, i-1]的元素都向后移动一位
    for (int j = i - 1; j >= left; j--) {
      arr[j + 1] = arr[j];
    }

    // 插入key到正确位置
    arr[left] = key;
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
 * 演示插入排序的过程
 * @param arr 待排序的数组
 */
void insertionSortWithSteps(std::vector<int> &arr) {
  int n = arr.size();
  std::cout << "初始数组: ";
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;

  for (int i = 1; i < n; i++) {
    int key = arr[i];
    int j = i - 1;

    std::cout << "第" << i << "轮: 插入元素 " << key << " -> ";

    while (j >= 0 && arr[j] > key) {
      arr[j + 1] = arr[j];
      j--;
    }
    arr[j + 1] = key;

    for (int num : arr)
      std::cout << num << " ";
    std::cout << std::endl;
  }
}

/**
 * 主函数 - 演示插入排序
 */
int main() {
  // 演示基本插入排序
  std::cout << "========== 插入排序演示 ==========" << std::endl;
  std::vector<int> arr1 = {64, 34, 25, 12, 22, 11, 90};
  printArray(arr1, "排序前: ");
  insertionSort(arr1);
  printArray(arr1, "排序后: ");

  // 演示二分插入排序
  std::cout << "\n========== 二分插入排序演示 ==========" << std::endl;
  std::vector<int> arr2 = {64, 34, 25, 12, 22, 11, 90};
  printArray(arr2, "排序前: ");
  binaryInsertionSort(arr2);
  printArray(arr2, "排序后: ");

  // 展示排序过程
  std::cout << "\n========== 插入排序过程展示 ==========" << std::endl;
  std::vector<int> arr3 = {5, 2, 4, 6, 1, 3};
  insertionSortWithSteps(arr3);

  // 测试已排序数组（最好情况）
  std::cout << "\n测试已排序数组（最好情况 O(n)）：" << std::endl;
  std::vector<int> sorted = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  printArray(sorted, "排序前: ");
  insertionSort(sorted);
  printArray(sorted, "排序后: ");

  // 测试逆序数组（最坏情况）
  std::cout << "\n测试逆序数组（最坏情况 O(n²)）：" << std::endl;
  std::vector<int> reversed = {10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
  printArray(reversed, "排序前: ");
  insertionSort(reversed);
  printArray(reversed, "排序后: ");

  return 0;
}
