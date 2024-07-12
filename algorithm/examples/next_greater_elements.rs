// 单调栈是一种特殊的栈数据结构，其元素按单调递增或单调递减的顺序排列。
// 下面是用于查找数组中每个元素的下一个更大元素，如果没有为-1。

fn main() {
    let nums = vec![2, 1, 2, 4, 3];
    let result = next_greater_elements(nums);
    println!("{:?}", result); // Output: [4, 2, 4, -1, -1]
}

fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];
    let mut stack = Vec::new();

    for i in 0..nums.len() {
        while let Some(&last) = stack.last() {
            if nums[i] > nums[last] {
                result[last] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}
