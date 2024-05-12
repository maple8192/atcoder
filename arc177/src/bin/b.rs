#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::{max, Ordering, Reverse};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::fmt::{Debug, Display, Pointer};
use std::marker::PhantomData;
use std::mem::swap;
use std::ops::{BitXor, Deref};
use ac_library::{Additive, Dsu, FenwickTree, LazySegtree, Monoid, Segtree, suffix_array_arbitrary};
use ac_library::{convolution, floor_sum, ModInt998244353};
use ac_library::{Max, Min};
use easy_ext::ext;
use itertools::{Itertools};
use nalgebra::min;
use num_bigint::BigUint;
use num_integer::{gcd, gcd_lcm};
use num_traits::{abs, pow};
use proconio::{fastout, input};
use proconio::marker::{Bytes, Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: Bytes
    }

    let mut a = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'0' {
            i += 1;
            continue;
        }

        let mut j = i + 1;
        while j < s.len() && s[j] == b'1' {
            j += 1;
        }
        a.push(i..j);
        i = j;
    }

    a.sort_by(|x, y| y.end.cmp(&x.end));
    
    let mut ans = Vec::new();
    for a in a {
        ans.resize(ans.len() + a.end, b'A');
        ans.resize(ans.len() + a.start, b'B');
    }
    
    println!("{}", ans.len());
    println!("{}", String::from_utf8(ans).unwrap());
}

const INF: usize = 1_000_000_000_000_000_000;

const DIR4: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIR8: [(isize, isize); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

#[ext]
impl usize {
    fn is_prime(&self) -> bool {
        if *self == 0 || *self == 1 {
            return false;
        }
        for i in (2..).take_while(|&x| x * x <= *self) {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }

    fn divisors(&self) -> HashSet<usize> {
        let mut div  = HashSet::new();
        for i in (1..).take_while(|&x| x * x <= *self) {
            if *self % i == 0 {
                div.insert(i);
                div.insert(*self / i);
            }
        }
        div
    }

    fn factors(&self) -> HashMap<usize, usize> {
        let mut n = *self;
        let mut fact = HashMap::new();
        for i in (2..).take_while(|&x| x * x <= *self) {
            while n % i == 0 {
                *fact.entry(i).or_insert(0) += 1;
                n /= i;
            }
        }
        if n != 1 {
            fact.insert(n, 1);
        }
        fact
    }
}

#[ext]
impl bool {
    fn yes_no(&self) -> &str {
        if *self {
            "Yes"
        } else {
            "No"
        }
    }

    fn possible_impossible(&self) -> &str {
        if *self {
            "possible"
        } else {
            "impossible"
        }
    }
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut vec = vec![true; n + 1];
    vec[0] = false;
    vec[1] = false;
    for i in (2..).take_while(|&i| i * i <= n) {
        if vec[i] {
            for j in (2..).take_while(|&j| i * j <= n) {
                vec[i * j] = false;
            }
        }
    }
    vec
}

fn compress<T: Ord>(vec: &[T]) -> Vec<usize> {
    let set = vec.iter().collect::<BTreeSet<_>>();
    let mut map = BTreeMap::new();
    for (i, n) in set.into_iter().enumerate() {
        map.insert(n, i);
    }
    let mut ret = Vec::new();
    for n in vec {
        ret.push(*map.get(n).unwrap());
    }
    ret
}

fn to_undirected(n: usize, e: &Vec<(usize, usize)>) -> Vec<HashSet<usize>> {
    let mut g = vec![HashSet::new(); n];
    for &(x, y) in e {
        g[x].insert(y);
        g[y].insert(x);
    }
    g
}

fn to_directed(n: usize, e: &Vec<(usize, usize)>) -> Vec<HashSet<usize>> {
    let mut g = vec![HashSet::new(); n];
    for &(x, y) in e {
        g[x].insert(y);
    }
    g
}
