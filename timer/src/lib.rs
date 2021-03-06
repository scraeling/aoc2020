#[allow(unused_macros)]
#[macro_export]
macro_rules! time {
    ($func:expr) => {{
        let now = std::time::Instant::now();
        let res = $func;
        let elapsed = now.elapsed();
        println!("{} took {:?}", stringify!($func), elapsed);
        res
    }}
}

#[cfg(test)]
mod test {
    use super::time;

    #[test]
    fn time_macro_without_args() {
        let func = || 42u32 * 69u32;
        let x = time!(func());
        assert_eq!(x, 42*69);
    }

    #[test]
    fn time_macro_with_args() {
        let arg1: u64 = 21;
        let arg2: u64 = 22;
        let func = |x:u64, y:u64| x*y;
        let x = time!(func(arg1, arg2));
        assert_eq!(x, func(arg1, arg2));
    }
}
