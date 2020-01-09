macro_rules! loop_n {
    ($count:expr, $blk:block) => {
        for _ in 1..$count {
            $blk
        }
    };
}
pub fn main() {
    loop_n!(5, { println!("here!") });
}
