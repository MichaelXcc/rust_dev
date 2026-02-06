/**
 * 冒泡排序 (Bubble Sort)
 * 
 * 算法原理：
 * 冒泡排序是一种简单的排序算法。它重复地遍历要排序的数列，一次比较两个元素，
 * 如果它们的顺序错误就把它们交换过来。遍历数列的工作是重复进行的，直到没有
 * 再需要交换的元素为止，说明数列已经排序完成。
 * 
 * 算法步骤：
 * 1. 比较相邻的元素。如果第一个比第二个大，就交换它们。
 * 2. 对每一对相邻元素做同样的工作，从开始第一对到结尾的最后一对。
 *    这步做完后，最后的元素会是最大的数。
 * 3. 针对所有的元素重复以上的步骤，除了最后一个。
 * 4. 持续每次对越来越少的元素重复上面的步骤，直到没有任何一对数字需要比较。
 * 
 * 时间复杂度：
 * - 最好情况：O(n) - 当数组已经有序时
 * - 最坏情况：O(n²) - 当数组逆序时
 * - 平均情况：O(n²)
 * 
 * 空间复杂度：O(1) - 只需要常数级别的额外空间
 * 
 * 稳定性：稳定 - 相等元素的相对顺序不会改变
 */

#include <iostream>
#include <vector>

/**
 * 冒泡排序函数
 * @param arr 待排序的数组（引用传递，直接修改原数组）
 */
void bubbleSort(std::vector<int>& arr) {
    int n = arr.size();
    
    // 外层循环：控制排序的轮数
    // 每完成一轮，就有一个最大元素被"冒泡"到正确位置
    for (int i = 0; i < n - 1; i++) {
        // 优化：设置一个标志位，如果某一轮没有发生交换，说明已经有序
        bool swapped = false;
        
        // 内层循环：进行相邻元素的比较和交换
        // 注意：每轮排序后，最后i个元素已经有序，无需再比较
        for (int j = 0; j < n - 1 - i; j++) {
            // 如果前一个元素大于后一个元素，则交换它们
            if (arr[j] > arr[j + 1]) {
                std::swap(arr[j], arr[j + 1]);
                swapped = true;  // 标记发生了交换
            }
        }
        
        // 如果这一轮没有发生任何交换，说明数组已经有序，提前退出
        if (!swapped) {
            break;
        }
    }
}

/**
 * 打印数组的辅助函数
 * @param arr 要打印的数组
 * @param message 打印前的提示信息
 */
void printArray(const std::vector<int>& arr, const std::string& message) {
    std::cout << message;
    for (int num : arr) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
}

/**
 * 主函数 - 演示冒泡排序
 */
int main() {
    // 创建测试数组
    std::vector<int> arr = {64, 34, 25, 12, 22, 11, 90, 5, 77, 30};
    
    std::cout << "========== 冒泡排序演示 ==========" << std::endl;
    printArray(arr, "排序前: ");
    
    // 执行冒泡排序
    bubbleSort(arr);
    
    printArray(arr, "排序后: ");
    
    // 测试已排序数组（展示优化效果）
    std::cout << "\n测试已排序数组：" << std::endl;
    std::vector<int> sorted = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    printArray(sorted, "排序前: ");
    bubbleSort(sorted);
    printArray(sorted, "排序后: ");
    
    // 测试逆序数组
    std::cout << "\n测试逆序数组：" << std::endl;
    std::vector<int> reversed = {10, 9, 8, 7, 6, 5, 4, 3, 2, 1};
    printArray(reversed, "排序前: ");
    bubbleSort(reversed);
    printArray(reversed, "排序后: ");
    
    return 0;
}
