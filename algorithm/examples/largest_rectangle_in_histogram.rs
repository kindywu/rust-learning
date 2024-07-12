use std::cmp::max;

fn main() {
    let test_cases = vec![
        (9, vec![2, 1, 5, 4, 3]),
        (18, vec![2, 1, 5, 4, 9, 10]),
        (9, vec![1, 2, 3, 1, 2, 3, 1, 2, 3]),
        (15, vec![1, 2, 3, 4, 5, 4, 3, 2, 1]),
    ];

    for (max_area, histogram) in test_cases {
        let area = max_area_in_histogram(histogram);
        assert_eq!(max_area, area);
    }
}

// 时间复杂度为O(n^2)，空间复杂度O(1)
fn max_area_in_histogram(histogram: Vec<i32>) -> i32 {
    let mut max_area = 0;

    for i in 0..histogram.len() {
        // 计算高度
        let height = histogram[i];
        let mut width = 1;

        // 计算宽度
        if i > 0 {
            for j in (0..i).rev() {
                if height <= histogram[j] {
                    width += 1;
                } else {
                    break;
                }
            }
        }

        for j in (i + 1)..histogram.len() {
            if height <= histogram[j] {
                width += 1;
            } else {
                break;
            }
        }
        // 计算最大面积
        let area = height * width;
        max_area = max(max_area, area);
    }

    max_area
}
