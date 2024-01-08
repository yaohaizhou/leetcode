#include <iostream>
#include <numeric>
#include <ostream>
#include <vector>

class Solution {
public:
  int maxSumTwoNoOverlap(std::vector<int> &nums, int firstLen, int secondLen) {
    std::vector<int> s(nums.size() + 1, 0);
    std::partial_sum(nums.begin(), nums.end(),
                     s.begin() + 1); // Calculate prefix sums
    int ans = 0;

    auto help = [&](int L1, int L2) -> void {
      int maxSumA = 0;
      for (int i = L1 + L2; i < s.size(); ++i) {
        maxSumA = std::max(maxSumA, s[i - L2] - s[i - L2 - L1]);
        ans = std::max(ans, maxSumA + s[i] - s[i - L2]);
      }
    };

    help(firstLen, secondLen);
    help(secondLen, firstLen);
    return ans;
  }
};

int main() {
  std::vector<int> nums = {0, 6, 5, 2, 2, 5, 1, 9, 4};
  int firstLen = 1;
  int secondLen = 2;
  std::cout << Solution().maxSumTwoNoOverlap(nums, firstLen, secondLen)
            << std::endl;

  return 0;
}