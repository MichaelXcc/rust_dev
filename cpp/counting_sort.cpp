/**
 * 计数排序 (Counting Sort)
 *
 * 算法原理：统计每个元素出现的次数，然后根据计数结果将元素放到正确位置。
 * 适用于整数排序，特别是范围较小的非负整数。
 *
 * 时间复杂度：O(n + k)，其中k是数据范围
 * 空间复杂度：O(n + k)
 * 稳定性：稳定
 */

#include <algorithm>
#include <iostream>
#include <vector>

/**
 * 计数排序（基本版本，只能处理非负整数）
 */
void countingSort(std::vector<int> &arr) {
  if (arr.empty())
    return;

  // 找到最大值
  int maxVal = *std::max_element(arr.begin(), arr.end());

  // 创建计数数组
  std::vector<int> count(maxVal + 1, 0);

  // 统计每个元素出现的次数
  for (int num : arr) {
    count[num]++;
  }

  // 根据计数重建数组
  int index = 0;
  for (int i = 0; i <= maxVal; i++) {
    while (count[i] > 0) {
      arr[index++] = i;
      count[i]--;
    }
  }
}

/**
 * 计数排序（稳定版本）
 */
void countingSortStable(std::vector<int> &arr) {
  if (arr.empty())
    return;

  int n = arr.size();
  int minVal = *std::min_element(arr.begin(), arr.end());
  int maxVal = *std::max_element(arr.begin(), arr.end());
  int range = maxVal - minVal + 1;

  std::vector<int> count(range, 0);
  std::vector<int> output(n);

  // 统计每个元素出现次数
  for (int num : arr) {
    count[num - minVal]++;
  }

  // 计算累积计数（确定每个元素的结束位置）
  for (int i = 1; i < range; i++) {
    count[i] += count[i - 1];
  }

  // 从后向前遍历（保证稳定性），将元素放入正确位置
  for (int i = n - 1; i >= 0; i--) {
    int pos = count[arr[i] - minVal] - 1;
    output[pos] = arr[i];
    count[arr[i] - minVal]--;
  }

  // 复制回原数组
  arr = output;
}

void printArray(const std::vector<int> &arr, const std::string &msg) {
  std::cout << msg;
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;
}

int main() {
  std::cout << "========== 计数排序演示 ==========" << std::endl;
  std::vector<int> arr = {4, 2, 2, 8, 3, 3, 1, 7, 5, 6};
  printArray(arr, "排序前: ");
  countingSort(arr);
  printArray(arr, "排序后: ");

  std::cout << "\n========== 稳定计数排序 ==========" << std::endl;
  std::vector<int> arr2 = {4, 2, 2, 8, 3, 3, 1, 7, 5, 6};
  printArray(arr2, "排序前: ");
  countingSortStable(arr2);
  printArray(arr2, "排序后: ");

  std::cout << "\n测试有重复元素的数组：" << std::endl;
  std::vector<int> arr3 = {1, 4, 1, 2, 7, 5, 2};
  printArray(arr3, "排序前: ");
  countingSort(arr3);
  printArray(arr3, "排序后: ");

  return 0;
}
