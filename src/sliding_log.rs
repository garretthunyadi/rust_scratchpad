// // use the linked list to implement a fixed-size log (sliding window)
// use super::linked_list::List;

// /// This is a l
// pub struct SlidingWindow<T> {
//     window_size: usize,
//     list: List<T>,
// }
// impl<T> SlidingWindow<T> {
//     pub fn new(window_size: usize) -> SlidingWindow<T> {
//         SlidingWindow {
//             window_size,
//             list: List::new(),
//         }
//     }
//     pub fn push(&mut self, val: T) {
//         self.list.push(val);
//     }
//     pub fn at(&self, i: usize) -> &T {
//         let i = self.window_size - i - 1;
//         self.list.at(i)
//     }
// }

// pub fn main() {
//     // println!("const log");
//     // let mut list: List<u32> = List::new();
//     // list.push(0);
//     // assert_eq!(*list.at(0), 0);
//     // list.push(1);
//     // assert_eq!(*list.at(1), 1);
//     // list.push(2);
//     // assert_eq!(*list.at(2), 2);
//     let mut window: SlidingWindow<u32> = SlidingWindow::new(4);
//     window.push(0);
//     window.push(1);
//     window.push(2);
//     assert_eq!(*window.at(0), 2);
//     assert_eq!(*window.at(1), 1);
//     assert_eq!(*window.at(2), 0);
// }
