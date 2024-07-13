#![allow(dead_code, unused_imports, unused_macros)]
use my_library::*;

fn main() {
    input! {
        s: Bytes
    }

    let mut abc = vec![];
    for i in 0..s.len().saturating_sub(2) {
        if s[i] == b'A' && s[i+1] == b'B' && s[i+2] == b'C' {
            let mut l = i;
            while l.wrapping_add_signed(-1) < s.len() && s[l - 1] == b'A' {
                l -= 1;
            }
            let mut r = i + 2;
            while r < s.len()-2 && s[r+1] == b'B' && s[r+2] == b'C' {
                r += 2;
            }
            abc.push(l..=r);
        }
    }

    let mut groups = vec![];
    for r in abc {
        let Some(last) = groups.last_mut() else {
            groups.push(r);
            continue;
        };
        if last.end() + 1 == *r.start() {
            *last = *last.start()..=*r.end();
        } else {
            groups.push(r);
        }
    }

    let mut ans = 0usize;
    for r in groups {
        let mut t = 0;
        let mut a = 0;
        for i in r {
            if s[i] == b'A' {
                a += 1;
            } else {
                t += a;
            }
        }
        ans += t / 2;
    }

    println!("{}", ans);
}

mod my_library {
    pub use std::iter::once;
    pub use std::collections::{BinaryHeap, BTreeMap, BTreeSet, VecDeque};
    pub use ac_library::*;
    pub use bstr::ByteSlice;
    pub use itertools::Itertools;
    pub use num_rational::Ratio;
    pub use num_traits::{FromPrimitive, ToPrimitive};
    pub use omniswap::swap;
    pub use proconio::{input, marker::{Bytes, Usize1}};
    pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
    pub use superslice::Ext;

    use std::ops::{AddAssign, MulAssign};
    use num::{One, Zero};
    use easy_ext::ext;
    use proconio::source::Readable;

    pub const INF: usize = 1_000_000_000_000_000_000;

    pub const DIR4: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    pub const DIR8: [(isize, isize); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

    #[ext]
    pub impl usize {
        fn is_prime(&self) -> bool {
            if *self == 0 || *self == 1 { return false }
            (2..).take_while(|&x| x * x <= *self).all(|x| *self % x != 0)
        }

        fn divisors(&self) -> Vec<usize> {
            let mut ret = vec![];
            for i in (1..).take_while(|&i| i * i <= *self).filter(|i| *self % i == 0) {
                ret.push(i);
                if *self / i != i { ret.push(*self / i) }
            }
            ret
        }

        fn factors(&self) -> Vec<(usize, usize)> {
            let mut ret = vec![];
            let mut n = *self;
            for i in 2..*self {
                if i * i > n { break }
                if n % i != 0 { continue }

                let mut e = 0;
                while n % i == 0 {
                    n /= i;
                    e += 1;
                }
                ret.push((i, e));
            }
            if n != 1 { ret.push((n, 1)) }
            ret
        }
    }

    #[ext]
    pub impl bool {
        fn yes_no(&self) -> &str {
            if *self { "Yes" } else { "No" }
        }
    }

    pub fn cumsum<T: Zero + AddAssign + Copy>(vec: &Vec<T>) -> Vec<T> {
        once(&T::zero()).chain(vec).scan(T::zero(), |acc, &x| { *acc += x; Some(*acc) }).collect_vec()
    }

    pub fn cumprod<T: One + MulAssign + Copy>(vec: &Vec<T>) -> Vec<T> {
        once(&T::one()).chain(vec).scan(T::one(), |acc, &x| { *acc *= x; Some(*acc) }).collect_vec()
    }

    pub fn eratosthenes(n: usize) -> Vec<bool> {
        if n == 0 { return vec![false] }
        let mut ret = vec![true; n + 1];
        ret[0] = false;
        ret[1] = false;
        for i in (2..).take_while(|i| i * i <= n) {
            if !ret[i] { continue }
            for j in (2..).take_while(|j| i * j <= n) {
                ret[i * j] = false;
            }
        }
        ret
    }

    pub fn compress<T: Ord>(vec: &[T]) -> Vec<usize> {
        let set = vec.iter().collect::<BTreeSet<_>>();
        let map = set.into_iter().enumerate().map(|(i, n)| (n, i)).collect::<BTreeMap<_, _>>();
        let mut ret = vec![];
        for n in vec {
            ret.push(map[n]);
        }
        ret
    }

    pub fn power_by<T, F: Fn(&T, &T) -> T>(mut a: T, mut n: usize, e: T, f: F) -> T {
        let mut ans = e;
        while n > 0 {
            if n % 2 != 0 { ans = f(&ans, &a) }
            a = f(&a, &a);
            n /= 2;
        }
        ans
    }

    #[macro_export]
    macro_rules! min {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::min($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::min($a, min!($($rest),+))
        }};
    }

    #[macro_export]
    macro_rules! max {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::max($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::max($a, max!($($rest),+))
        }}
    }

    #[macro_export]
    macro_rules! gcd {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            num_integer::gcd($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            num_integer::gcd($a, gcd!($($rest),+))
        }};
    }

    #[macro_export]
    macro_rules! lcm {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            num_integer::lcm($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            num_integer::lcm($a, lcm!($($rest),+))
        }};
    }

    fn find_furthest_vertex(tree: &[Vec<(usize, usize)>], start: usize) -> (usize, usize) {
        let mut dist = HashMap::from_iter([(start, 0)]);
        let mut queue = VecDeque::from_iter([start]);
        while let Some(v) = queue.pop_front() {
            for &(nv, cost) in &tree[v] {
                if dist.contains_key(&nv) { continue }
                dist.insert(nv, dist[&v] + cost);
                queue.push_back(nv);
            }
        }
        dist.into_iter().max_by_key(|&(_, d)| d).unwrap()
    }

    pub fn tree_diameter(tree: &[Vec<(usize, usize)>]) -> (usize, usize) {
        let (start, _) = find_furthest_vertex(tree, 0);
        find_furthest_vertex(tree, start)
    }

    pub enum Offset<const N: usize> {}
    impl<const N: usize> Readable for Offset<N> {
        type Output = usize;
        fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
            (isize::read(source) + N as isize) as usize
        }
    }
    pub type Offset10e9 = Offset<1_000_000_000>;
}
