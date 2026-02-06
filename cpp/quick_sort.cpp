/**
 * 快速排序 (Quick Sort)
 *
 * 算法原理：选择基准元素，将数组分成两部分：小于基准的在左，大于基准的在右。
 * 时间复杂度：平均O(n log n)，最坏O(n²)
 * 空间复杂度：O(log n)
 * 稳定性：不稳定
 */

#include <iostream>
#include <random>
#include <vector>

// Lomuto分区
int partitionLomuto(std::vector<int> &arr, int low, int high) {
  int pivot = arr[high];
  int i = low - 1;

  for (int j = low; j < high; j++) {
    if (arr[j] < pivot) {
      i++;
      std::swap(arr[i], arr[j]);
    }
  }
  std::swap(arr[i + 1], arr[high]);
  return i + 1;
}

// Hoare分区
int partitionHoare(std::vector<int> &arr, int low, int high) {
  int pivot = arr[low];
  int i = low - 1, j = high + 1;

  while (true) {
    do {
      i++;
    } while (arr[i] < pivot);
    do {
      j--;
    } while (arr[j] > pivot);
    if (i >= j)
      return j;
    std::swap(arr[i], arr[j]);
  }
}

void quickSortLomuto(std::vector<int> &arr, int low, int high) {
  if (low < high) {
    int pi = partitionLomuto(arr, low, high);
    quickSortLomuto(arr, low, pi - 1);
    quickSortLomuto(arr, pi + 1, high);
  }
}

void quickSortHoare(std::vector<int> &arr, int low, int high) {
  if (low < high) {
    int pi = partitionHoare(arr, low, high);
    quickSortHoare(arr, low, pi);
    quickSortHoare(arr, pi + 1, high);
  }
}

void quickSort(std::vector<int> &arr) {
  if (arr.size() <= 1)
    return;
  quickSortLomuto(arr, 0, arr.size() - 1);
}

void printArray(const std::vector<int> &arr, const std::string &msg) {
  std::cout << msg;
  for (int num : arr)
    std::cout << num << " ";
  std::cout << std::endl;
}

int main() {
  std::cout << "========== 快速排序演示 ==========" << std::endl;
  std::vector<int> arr = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
  printArray(arr, "排序前: ");
  quickSort(arr);
  printArray(arr, "排序后: ");

  std::vector<int> arr2 = {10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
  printArray(arr2, "\n逆序数组: ");
  quickSort(arr2);
  printArray(arr2, "排序后: ");

  return 0;
}
