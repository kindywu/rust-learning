#![allow(unused)]

const LENGTH: usize = 8 * 1024 * 1024;

/*
   https://godbolt.org/
   const LENGTH: usize = 8 * 1024 * 1024;

   #[no_mangle]
   pub fn test_box(num: i32) {
       let buf:[u8;LENGTH] = [0_u8; LENGTH];
       let buf_box = Box::new(buf);

       println!("{:?}", buf_box);
   }
   -C opt-level=1 -C force-frame-pointers=yes
*/

// 必须是优化版本才可以成功执行，llvm优化会跳过在栈创建，直接在堆创建。
// cargo.exe run --release --package syntax --example box

fn main() {
    let buf = BigStruct {
        data: [0_u8; LENGTH],
    };
    let buf_box = Box::new(buf);
    println!("{}", buf_box.data.len());

    // debug 版本依然可以执行，方法1
    // let buf_box: Box<[u8; LENGTH]> = vec![0u8; LENGTH].try_into().unwrap();
    // println!("{}", buf_box.len());
}

struct BigStruct {
    data: [u8; LENGTH],
}
