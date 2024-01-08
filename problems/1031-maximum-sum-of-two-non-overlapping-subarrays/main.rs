use std::cmp;
use std::iter;

struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: usize, second_len: usize) -> i32 {
        let s: Vec<i32> = iter::once(0)
            .chain(nums.into_iter())
            .scan(0, |sum, x| {
                *sum += x;
                Some(*sum)
            })
            .collect();
        let mut ans = 0;

        {
            // Define a helper closure
            let mut helper = |first_len: usize, second_len: usize| {
                let mut max_sum_a = 0;
                // Traverse B index
                for i in (first_len + second_len)..s.len() {
                    // Compare sum of previous A with the array ahead of B
                    max_sum_a =
                        cmp::max(max_sum_a, s[i - second_len] - s[i - second_len - first_len]);
                    // Compare ans with sum of A and B
                    ans = cmp::max(ans, max_sum_a + s[i] - s[i - second_len]);
                }
            };
            // Change the order of first_len and second_len to go through all cases
            helper(first_len, second_len);
            helper(second_len, first_len);
        }

        ans
    }
}

fn main() {
    let nums = vec![1, 0, 3, 8, 4, 9, 1, 3];
    let first_len = 3;
    let second_len = 2;
    let result = Solution::max_sum_two_no_overlap(nums, first_len, second_len);
    println!("The result is {}", result);
}
