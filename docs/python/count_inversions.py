# 使用归并排序计算逆序数
def count_inversions_merge_sort(arr):
    """
    使用归并排序计算逆序数
    时间复杂度: O(nlogn)
    """

    def merge_count(arr, temp, left, mid, right):
        i = left  # 左半部分起始索引
        j = mid + 1  # 右半部分起始索引
        k = left  # 临时数组索引
        inv_count = 0

        # 合并两个有序子数组并统计逆序数
        while i <= mid and j <= right:
            if arr[i] <= arr[j]:
                temp[k] = arr[i]
                i += 1
            else:
                temp[k] = arr[j]
                inv_count += (mid - i + 1)  # 关键：左半剩余元素都与右当前元素构成逆序
                j += 1
            k += 1

        # 复制剩余元素
        while i <= mid:
            temp[k] = arr[i]
            i += 1
            k += 1

        while j <= right:
            temp[k] = arr[j]
            j += 1
            k += 1

        # 将排序结果复制回原数组
        for i in range(left, right + 1):
            arr[i] = temp[i]

        return inv_count

    def merge_sort_count(arr, temp, left, right):
        inv_count = 0
        if left < right:
            mid = (left + right) // 2

            inv_count += merge_sort_count(arr, temp, left, mid)
            inv_count += merge_sort_count(arr, temp, mid + 1, right)
            inv_count += merge_count(arr, temp, left, mid, right)

        return inv_count

    # 创建临时数组并调用递归函数
    return merge_sort_count(arr.copy(), [0] * len(arr), 0, len(arr) - 1)

import numpy as np

level = np.random.randint(3, 10)
print(f"数组长度: {level}")
# 测试: 生成1-9的序数，然后随机打乱，计算逆序数
arr = [i for i in range(1, level + 1)]
for i in range(1, 10):
    np.random.shuffle(arr)
    print(f"数组{i}: {arr}")
    inversions_str = ''.join(str(x) for x in arr)
    inversions = count_inversions_merge_sort(arr)
    print(f"N({inversions_str})={inversions}")

ranks = ["列兵", "上等兵","下士","中士","二级上士","一级上士","三级军士长","二级军士长","一级军士长","少尉","中尉","上尉","少校","中校","上校","少将","中将","上将"]








