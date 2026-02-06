/**
 * 基数排序 (Radix Sort)
 *
 * 算法原理：按照低位先排序，然后收集；再按照高位排序，然后再收集；
 * 依次类推，直到最高位。适用于整数排序。
 *
 * 时间复杂度：O(d * (n + k))，d是位数，k是基数（通常为10）
 * 空间复杂度：O(n + k)
 * 稳定性：稳定
 */

#include <algorithm>
#include <iostream>
#include <vector>

/**
 * 获取数组中最大值
 */
int getMax(const std::vector<int> &arr) {
  return *std::max_element(arr.begin(), arr.end());
}

/**
 * 对指定位数进行计数排序
 * @param arr 待排序数组
 * @param exp 当前位数（1, 10, 100, ...）
 */
void countingSortByDigit(std::vector<int> &arr, int exp) {
  int n = arr.size();
  std::vector<int> output(n);
  std::vector<int> count(10, 0); // 基数为10

  // 统计当前位上每个数字出现的次数
  for (int num : arr) {
    int digit = (num / exp) % 10;
    count[digit]++;
  }

  // 计算累积计数
  for (int i = 1; i < 10; i++) {
    count[i] += count[i - 1];
  }

  // 从后向前遍历，保证稳定性
  for (int i = n - 1; i >= 0; i--) {
    int digit = (arr[i] / exp) % 10;
    output[count[digit] - 1] = arr[i];
    count[digit]--;
  }

  arr = output;
}

/**
 * 基数排序（LSD - 从最低位开始）
 */
void radixSort(std::vector<int> &arr) {
  if (arr.empty())
    return;

  int maxVal = getMax(arr);

  // 从最低位开始，对每一位进行计数排序
  for (int exp = 1; maxVal / exp > 0; exp *= 10) {
    countingSortByDigit(arr, exp);
  }
}

void printArray(const std::vector<int> &arr, const std::string &msg) {
  std::cout << msg;
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;
}

int main() {
  std::cout << "========== 基数排序演示 ==========" << std::endl;
  std::vector<int> arr = {170, 45, 75, 90, 802, 24, 2, 66};
  printArray(arr, "排序前: ");
  radixSort(arr);
  printArray(arr, "排序后: ");

  std::cout << "\n测试更多数据：" << std::endl;
  std::vector<int> arr2 = {329, 457, 657, 839, 436, 720, 355};
  printArray(arr2, "排序前: ");
  radixSort(arr2);
  printArray(arr2, "排序后: ");

  std::cout << "\n测试单位数：" << std::endl;
  std::vector<int> arr3 = {5, 2, 9, 1, 7, 3, 8, 4, 6};
  printArray(arr3, "排序前: ");
  radixSort(arr3);
  printArray(arr3, "排序后: ");

  return 0;
}
