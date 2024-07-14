#![allow(unused)]

use core::panic;
use std::cmp::max;

fn main() {
    // largest_rectangle_in_histogram();
    largest_rectangle_in_matrix();
}

fn largest_rectangle_in_matrix() {
    let test_cases = vec![
        (
            4,
            vec![
                vec!{0, 1, 1, 0, 0},
                vec![0, 1, 0, 1, 1],
                vec![1, 1, 1, 0, 1],
                vec![1, 1, 0, 1, 0],
            ],
        ),
        (
            5,
            vec![
                vec![1, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 1, 0],
            ],
        ),
        (
            6,
            vec![
                vec![1, 0, 1, 0, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 0, 1, 0],
            ],
        ),
    ];

    for (max_area, matrix) in test_cases.clone() {
        let area = max_area_in_matrix(matrix.clone());
        println!("max_area_in_matrix: ");
        print_matrix(&matrix);
        println!("max area: expected= {max_area}, got= {area}");
    }
}

fn largest_rectangle_in_histogram() {
    let test_cases = vec![
        (0, vec![]),
        (996, vec![996, 1]),
        (999, vec![996, 1, 999]),
        (999, vec![1, 2, 3, 2, 999]),
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

// 时间复杂度为O(n)，空间复杂度O(n)
fn max_area_in_histogram_v2(mut histogram: Vec<i32>) -> i32 {
    let mut max_area = 0;

    histogram.push(0);

    let mut stack = Vec::new();

    for i in 0..histogram.len() {
        while !stack.is_empty() && histogram[i] < histogram[stack[stack.len() - 1]] {
            let len = stack.len() - 1;
            let last = stack[len];
            let height = histogram[last];

            // 准备处理栈顶元素，前面元素的索引高度比它小，准备入栈前面的索引高度比它小，想减得到宽度
            let width = if stack.len() > 1 {
                i - stack[len - 1] - 1
            } else {
                i //前面没有元素
            };

            let area = height * (width as i32);
            max_area = max(max_area, area);

            stack.pop();
        }

        stack.push(i);
    }

    max_area
}

// 计算矩阵中最大的四边形
fn max_area_in_matrix(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.len() == 0 {
        return 0;
    }
    let len = matrix[0].len();
    let mut histogram = vec![0; len];

    let mut max_area = 0;

    for h in matrix {
        for i in 0..h.len() {
            if h[i] == 0 {
                histogram[i] = 0;
            } else {
                histogram[i] += 1;
            }
        }

        max_area = max(max_area, max_area_in_histogram(histogram.clone()));
        // println!("histogram: {histogram:?}, max_area: {max_area}");
    }

    max_area
}

// 打印矩阵
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for line in matrix {
        println!("{line:?}");
    }
}
