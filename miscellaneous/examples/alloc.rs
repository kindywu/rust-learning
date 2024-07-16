#![allow(unused)]

use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::{ptr, usize};

fn main() {
    const N: usize = 3;
    let mut arr = alloc_arr::<N>();
    for i in 0..N {
        unsafe {
            (*arr)[i] = i as i32; // 给数组元素赋值
        }
    }

    // 打印数组以验证赋值是否成功
    unsafe {
        println!("{:?}", *arr);
    }

    let mut arr2 = alloc_arr_copy::<N, 6>(arr);
    unsafe {
        println!("{:?}", *arr2);
    }
}

fn alloc_arr<const N: usize>() -> *mut [i32; N] {
    unsafe {
        let layout = Layout::new::<[i32; N]>();
        let pointer = alloc_zeroed(layout) as *mut [i32; N];
        if pointer.is_null() {
            panic!("Memory allocation failed");
        }
        pointer
    }
}

fn alloc_arr_copy<const N: usize, const M: usize>(arr: *mut [i32; N]) -> *mut [i32; M] {
    unsafe {
        let layout = Layout::new::<[i32; M]>();
        let pointer = alloc_zeroed(layout) as *mut [i32; M];
        if pointer.is_null() {
            panic!("Memory allocation failed");
        }

        let p1: *mut i32 = pointer as *mut i32;
        let p2: *mut i32 = arr as *mut i32;
        ptr::copy(p2, p1, N);

        pointer
    }
}

fn alloc_and_expand() {
    let arr = unsafe {
        const N: usize = 3;
        let layout = Layout::new::<[i32; N]>();
        let pointer = alloc_zeroed(layout) as *mut i32;
        if pointer.is_null() {
            panic!("Memory allocation failed");
        }

        // 初始化数组元素
        for i in 0..N {
            *pointer.add(i) = i as i32 + 1; // 赋值 1, 2, 3
        }

        println!("{:?}", std::slice::from_raw_parts(pointer, N)); // 打印预期的数组值

        // 为更大的数组分配内存
        let layout2 = Layout::new::<[i32; N * 2]>();
        let pointer2 = alloc_zeroed(layout2) as *mut i32;
        if pointer2.is_null() {
            panic!("Memory allocation failed");
        }

        // 将 pointer 的数据拷贝到 pointer2
        ptr::copy(pointer, pointer2, N);
        // 释放内存
        dealloc(pointer as *mut u8, layout);

        // 打印拷贝后的数组
        let arr = std::slice::from_raw_parts(pointer2, N * 2);

        arr
    };

    println!("{:?}", arr);
}
