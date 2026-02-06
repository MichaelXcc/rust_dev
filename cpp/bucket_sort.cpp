/**
 * 桶排序 (Bucket Sort)
 *
 * 算法原理：将数据分到有限数量的桶里，每个桶再单独排序，最后合并。
 * 适用于数据分布均匀的情况。
 *
 * 时间复杂度：平均O(n + k)，最坏O(n²)
 * 空间复杂度：O(n + k)
 * 稳定性：取决于桶内排序算法
 */

#include <algorithm>
#include <iostream>
#include <vector>

/**
 * 桶排序（浮点数版本）
 */
void bucketSortFloat(std::vector<float> &arr) {
  int n = arr.size();
  if (n <= 0)
    return;

  // 创建n个空桶
  std::vector<std::vector<float>> buckets(n);

  // 将每个元素放入对应的桶（假设元素在[0, 1)范围内）
  for (float num : arr) {
    int bucketIndex = n * num;
    buckets[bucketIndex].push_back(num);
  }

  // 对每个桶内的元素排序
  for (auto &bucket : buckets) {
    std::sort(bucket.begin(), bucket.end());
  }

  // 合并所有桶
  int index = 0;
  for (auto &bucket : buckets) {
    for (float num : bucket) {
      arr[index++] = num;
    }
  }
}

/**
 * 桶排序（整数版本）
 */
void bucketSort(std::vector<int> &arr) {
  if (arr.empty())
    return;

  int n = arr.size();
  int minVal = *std::min_element(arr.begin(), arr.end());
  int maxVal = *std::max_element(arr.begin(), arr.end());

  // 计算桶的数量和大小
  int bucketCount = n;
  int bucketSize = (maxVal - minVal) / bucketCount + 1;

  std::vector<std::vector<int>> buckets(bucketCount);

  // 将元素分配到桶中
  for (int num : arr) {
    int bucketIndex = (num - minVal) / bucketSize;
    buckets[bucketIndex].push_back(num);
  }

  // 对每个桶排序并合并
  int index = 0;
  for (auto &bucket : buckets) {
    std::sort(bucket.begin(), bucket.end());
    for (int num : bucket) {
      arr[index++] = num;
    }
  }
}

void printArray(const std::vector<int> &arr, const std::string &msg) {
  std::cout << msg;
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;
}

void printArrayFloat(const std::vector<float> &arr, const std::string &msg) {
  std::cout << msg;
  for (float num : arr)
    std::cout << num << " ";
  std::cout << std::endl;
}

int main() {
  std::cout << "========== 桶排序（整数）==========" << std::endl;
  std::vector<int> arr = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr, "排序前: ");
  bucketSort(arr);
  printArray(arr, "排序后: ");

  std::cout << "\n========== 桶排序（浮点数）==========" << std::endl;
  std::vector<float> arrFloat = {0.42f, 0.32f, 0.33f, 0.52f,
                                 0.37f, 0.47f, 0.51f};
  printArrayFloat(arrFloat, "排序前: ");
  bucketSortFloat(arrFloat);
  printArrayFloat(arrFloat, "排序后: ");

  return 0;
}
