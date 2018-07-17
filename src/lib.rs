/// map a function over an array
///
/// Examples:
/// ```
/// use arraymap::ArrayMap;
///
/// let x = [1, 2, 3];
/// assert_eq!([2, 3, 4], x.map(|v| v + 1));
/// ```
pub trait ArrayMap<X, Y, T> {
    fn map<F: Fn(X) -> Y>(self, f: F) -> T;
}

macro_rules! map_impl {
    {$n:tt, $v:ident, $($ns:tt, $vs:ident),*} => {
        impl<U, V> ArrayMap<U, V, [V; $n]> for [U; $n] {
            fn map<F: Fn(U) -> V>(self, f: F) -> [V; $n] {
                (|[$($vs),*]: [U; $n]| [$(f($vs)),*])(self)
            }
        }
        impl<'a, U, V> ArrayMap<&'a U, V, [V; $n]> for &'a [U; $n] {
            fn map<F: Fn(&'a U) -> V>(self, f: F) -> [V; $n] {
                (|&[$(ref $vs),*]: &'a [U; $n]| [$(f($vs)),*])(self)
            }
        }
        map_impl!{$($ns, $vs),*}
    };
    {$n:tt, $v:ident} => { /* nothing to do here */ }
}

impl<U, V> ArrayMap<U, V, [V; 0]> for [U; 0] {
    fn map<F: Fn(U) -> V>(self, _: F) -> [V; 0] { [] }
}
impl<'a, U, V> ArrayMap<&'a U, V, [V; 0]> for &'a [U; 0] {
    fn map<F: Fn(&'a U) -> V>(self, _: F) -> [V; 0] { [] }
}

map_impl!{32, x32, 31, x31, 30, x30, 29, x29, 28, x28, 27, x27, 26, x26, 25, x25,
          24, x24, 23, x23, 22, x22, 21, x21, 20, x20, 19, x19, 18, x18, 17, x17,
          16, x16, 15, x15, 14, x14, 13, x13, 12, x12, 11, x11, 10, x10, 9, x9,
          8, x8, 7, x7, 6, x6, 5, x5, 4, x4, 3, x3, 2, x2, 1, x1, 0, x0}

#[cfg(test)]
mod tests {
    use ArrayMap;

    #[test]
    fn empty_array() {
        let x : [u8; 0] = [];
        assert_eq!(x, x.map(|_| unreachable!()));
    }
}
