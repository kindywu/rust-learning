#![allow(unused)]

use std::cmp::max;

fn main() {
    let test_cases = vec![
        (8, vec![1, 2, 3, 2, 3]),
        (9, vec![1, 2, 5, 4, 3]),
        (18, vec![2, 1, 5, 4, 9, 10]),
        (9, vec![1, 2, 3, 1, 2, 3, 1, 2, 3]),
        (15, vec![1, 2, 3, 4, 5, 4, 3, 2, 1]),
    ];

    for (max_area, histogram) in test_cases.clone() {
        let area = max_area_in_histogram(histogram.clone());
        println!("max_area_in_histogram histogram: {histogram:?}, max area: expected= {max_area}, got= {area}");

        let area = max_area_in_histogram_v2(histogram.clone());
        println!("max_area_in_histogram_v2 histogram: {histogram:?}, max area: expected= {max_area}, got= {area}");
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

fn max_area_in_histogram_v2(mut histogram: Vec<i32>) -> i32 {
    let mut max_area = 0;

    histogram.push(0);

    let mut stack = Vec::new();

    for i in 0..histogram.len() {
        while !stack.is_empty() && histogram[i] < histogram[stack[stack.len() - 1]] {
            let len = stack.len() - 1;
            let last = stack[len];
            let height = histogram[last];

            let left = if stack.len() > 1 {
                last - stack[len - 1]
            } else {
                1
            };
            let right = i - last;

            let width = left + right - 1;

            // println!(
            //     "stack:{stack:?} i:{i} last:{last} left:{left} right:{right} height:{height} width:{width}"
            // );

            let area = height * (width as i32);
            max_area = max(max_area, area);

            stack.pop();
        }

        stack.push(i);
    }

    max_area
}
