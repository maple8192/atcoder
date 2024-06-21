#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, VecDeque};
use std::iter::once;
use ac_library::{Additive, Dsu, DynamicModInt, Max, Min, ModInt1000000007, ModInt998244353, Monoid, Multiplicative, Segtree};
use bstr::ByteSlice;
use easy_ext::ext;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use num_integer::{gcd, gcd_lcm};
use omniswap::swap;
use proconio::{fastout, input};
use proconio::marker::{Bytes, Usize1};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

fn main() {
    input! {
        s: Bytes,
        t: Bytes
    }

    let mut ss = FxHashMap::default();
    let mut sa = 0;
    for ch in s {
        if ch != b'@' {
            *ss.entry(ch).or_insert(0) += 1;
        } else {
            sa += 1;
        }
    }

    let mut st = FxHashMap::default();
    let mut ta = 0;
    for ch in t {
        if ch != b'@' {
            *st.entry(ch).or_insert(0) += 1;
        } else {
            ta += 1;
        }
    }

    let mut diff_s = FxHashMap::default();
    let mut diff_t = FxHashMap::default();
    for (k, &v) in &ss {
        if st.contains_key(k) {
            if v > st[k] {
                diff_t.insert(k, v - st[k]);
            } else if st[k] > v {
                diff_s.insert(k, st[k] - v);
            }
        } else {
            diff_t.insert(k, v);
        }
    }
    for (k, &v) in &st {
        if !ss.contains_key(k) {
            diff_s.insert(k, v);
        }
    }

    if !diff_s.keys().all(|&&ch| ch == b'a' || ch == b't' || ch == b'c' || ch == b'o' || ch == b'd' || ch == b'e' || ch == b'r') {
        println!("No");
        return;
    }
    if !diff_t.keys().all(|&&ch| ch == b'a' || ch == b't' || ch == b'c' || ch == b'o' || ch == b'd' || ch == b'e' || ch == b'r') {
        println!("No");
        return;
    }
    if diff_s.iter().map(|(_, v)| v).sum::<usize>() > sa {
        println!("No");
        return;
    }
    if diff_t.iter().map(|(_, v)| v).sum::<usize>() > ta {
        println!("No");
        return;
    }

    println!("Yes");
}

const INF: usize = 1_000_000_000_000_000_000;

const DIR4: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIR8: [(isize, isize); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

#[ext]
impl usize {
    fn is_prime(&self) -> bool {
        if *self == 0 || *self == 1 { return false }
        (2..).take_while(|&x| x * x <= *self).all(|x| *self % x == 0)
    }

    fn divisors(&self) -> Vec<usize> {
        let mut div = vec![];
        for i in (1..).take_while(|&x| x * x <= *self) {
            if *self % i != 0 { continue }
            div.push(i);
            if *self / i != i { div.push(*self / i) }
        }
        div
    }

    fn factors(&self) -> Vec<(usize, usize)> {
        let mut fact = vec![];
        let mut n = *self;
        for i in 2..=*self {
            if i * i > n { break }
            if n % i != 0 { continue }
            let mut e = 0;
            while n % i == 0 {
                e += 1;
                n /= i;
            }
            fact.push((i, e));
        }
        if n != 1 { fact.push((n, 1)) }
        fact
    }
}

#[ext]
impl bool {
    fn yes_no(&self) -> &str {
        if *self { "Yes" } else { "No" }
    }
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut vec = vec![true; n + 1];
    vec[0] = false;
    vec[1] = false;
    for i in (2..).take_while(|&x| x * x <= n) {
        if !vec[i] { continue }
        for j in (2..).take_while(|&j| i * j <= n) {
            vec[i * j] = false;
        }
    }
    vec
}

fn compress<T: Ord>(vec: &[T]) -> Vec<usize> {
    let set = vec.iter().collect::<BTreeSet<_>>();
    let map = set.into_iter().enumerate().map(|(i, n)| (n, i)).collect::<BTreeMap<_, _>>();
    let mut ret = vec![];
    for n in vec {
        ret.push(map[n]);
    }
    ret
}

fn power_by<T, F: Fn(&T, &T) -> T>(mut a: T, mut n: usize, e: T, f: F) -> T {
    let mut ans = e;
    while n > 0 {
        if n % 2 != 0 { ans = f(&ans, &a) }
        a = f(&a, &a);
        n /= 2;
    }
    ans
}
