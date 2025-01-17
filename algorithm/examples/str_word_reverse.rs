#![allow(unused)]

fn main() {
    utf8_str_word_reverse();
    ascii_str_word_reverse();
}

fn utf8_str_word_reverse() {
    // let mut str = String::from("你好");
    let mut str = String::from("你好 abc otto 中国");

    let mut cs: Vec<_> = str.chars().collect();
    let mut start = 0;
    let mut stop = 0;

    println!("{str}");

    let len = cs.len();

    // println!("start: {start}; stop: {stop}; len= {len}");

    for i in 0..len {
        let c = cs[i];
        if c == ' ' || i == (len - 1) {
            stop = if c == ' ' { i - 1 } else { i };

            let size = stop - start;
            let mid = size / 2;
            let remain = size % 2;

            // println!("start: {start}; stop: {stop}; size: {size}; mid: {mid}; remain: {remain}");

            cs[start..stop + 1].reverse();

            // for j in 0..mid + remain {
            //     let tmp = arr[start + j];
            //     arr[start + j] = arr[stop - j];
            //     arr[stop - j] = tmp;
            // }
            start = i + 1;
        }
    }

    let str: String = cs.iter().cloned().collect();
    println!("{}", str);
}

fn ascii_str_word_reverse() {
    // let mut arr = b"hello".to_vec();
    // let mut arr: Vec<u8> = b"toto".to_vec();
    // let mut arr = b"hello world toto".to_vec();
    let mut arr = b"hello world, nice to see you".to_vec();

    println!("{}", String::from_utf8_lossy(arr.as_slice()));

    let mut start = 0;
    let mut stop = 0;

    let len = arr.len();

    // println!("start: {start}; stop: {stop}; len= {len}");

    for i in 0..arr.len() {
        let c = arr[i];
        if c == 32 || i == (len - 1) {
            stop = if c == 32 { i - 1 } else { i };

            let size = stop - start;
            let mid = size / 2;
            let remain = size % 2;

            // println!("start: {start}; stop: {stop}; size: {size}; mid: {mid}; remain: {remain}");

            arr[start..stop + 1].reverse();

            // for j in 0..mid + remain {
            //     let tmp = arr[start + j];
            //     arr[start + j] = arr[stop - j];
            //     arr[stop - j] = tmp;
            // }
            start = i + 1;
        }
    }

    println!("{}", String::from_utf8_lossy(arr.as_slice()));
}
