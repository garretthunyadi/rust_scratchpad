#[macro_export]
macro_rules! s {
    ($e:expr) => {
        String::from($e)
    };
}

#[macro_export]
macro_rules! puts {
    ($e:expr) => {
        println!("{}", $e);
    };
}

#[macro_export]
macro_rules! puto {
    ($e:expr) => {
        println!("{:?}", $e);
    };
}

#[macro_export]
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
