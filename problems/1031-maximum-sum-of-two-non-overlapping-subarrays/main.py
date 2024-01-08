from typing import List
from itertools import accumulate

# ref: https://leetcode.cn/problems/maximum-sum-of-two-non-overlapping-subarrays/solutions/2245647/tu-jie-mei-you-si-lu-yi-zhang-tu-miao-do-3lli/


class Solution:
    def maxSumTwoNoOverlap(self, nums: List[int], firstLen: int, secondLen: int) -> int:
        s = list(accumulate(nums, initial=0))
        ans = 0

        def help(firstLen: int, secondLen: int) -> None:
            nonlocal ans
            maxSumA = 0
            # traverse B index
            for i in range(firstLen+secondLen, len(s)):
                # compare sum of previous A with the array ahead of B
                maxSumA = max(maxSumA, s[i-secondLen]-s[i-secondLen-firstLen])
                # compare ans with sum of A and B
                ans = max(ans, maxSumA+s[i]-s[i-secondLen])
        # change the order of firstLen and secondLen to go through all cases
        help(firstLen, secondLen)
        help(secondLen, firstLen)

        return ans
