/// map a function over an array
///
/// Examples:
/// ```
/// let x = [1, 2, 3];
/// assert_eq!([2, 3, 4], x.map(|v| v + 1));
/// ```
pub trait ArrayMap<X, Y, T> {
    fn map<F: Fn(&X) -> Y>(&self, f: F) -> T;
}

macro_rules! map_inner {
    {$f:ident, $s:expr, []; $n:expr, $($ns:expr),*,} => {
        map_inner!($f, $s, [$f(&$s[$n])]; $($ns),*,)
    };
    {$f:ident, $s:expr, [$($t:tt)*]; $n:expr, $($ns:expr),*,} => {
        map_inner!($f, $s, [$f(&$s[$n]), $($t)*]; $($ns),*,)
    };
    {$f:ident, $s:expr, [$($t:tt)*]; $n:expr, } => {
        map_inner!($f, $s, [$f(&$s[$n]), $($t)*])
    };
    {$f:ident, $s:expr, $t:expr} => { $t };
}

macro_rules! map_impl {
    {$n:expr, $($ns:expr),*} => {
        impl<U, V> ArrayMap<U, V, [V; $n]> for [U; $n] {
            fn map<F: Fn(&U) -> V>(&self, f: F) -> [V; $n] {
                map_inner!(f, self, []; $($ns),*,)
            }
        }
        map_impl!{$($ns),*}
    };
    {$n:expr} => { /* nothing to do here */ }
}

impl<U, V> ArrayMap<U, V, [V; 0]> for [U; 0] {
    fn map<F: Fn(&U) -> V>(&self, _: F) -> [V; 0] { [] }
}

map_impl!{32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
          15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0}

#[cfg(test)]
mod tests {
    use ArrayMap;

    #[test]
    fn empty_array() {
        let x : [u8; 0] = [];
        assert_eq!(x, x.map(|_| unreachable!()));
    }
}
