// struct Coroutine<T> {
//     // 定义协程的数据结构
//     func: dyn FnOnce() -> T + 'static, // 函数
// }

// impl<T> Coroutine<T> {
//     // 实现协程的创建
//     fn new<F>(f: F) -> Self
//     where
//         F: FnOnce() -> T,
//     {
//         Self { func: f }
//     }

//     // 实现协程的运行
//     fn resume(self) -> T {
//         // 你的代码
//     }
// }

// // 实现协程调度器
// struct CoroutineScheduler {
//     // 定义调度器的数据结构
// }

// impl CoroutineScheduler {
//     // 实现协程的调度
//     fn schedule<F, T>(&self, coroutine: Coroutine<F>) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         // 你的代码
//     }
// }

fn main() {}
