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
        a: isize,
        b: isize,
        c: isize,
        d: isize
    }

    let ar = (((a % 4) + 4) % 4) as usize;
    let br = (((b % 2) + 2) % 2) as usize;
    let cr = (((c % 4) + 4) % 4) as usize;
    let dr = (((d % 2) + 2) % 2) as usize;

    let ra = if ar != 0 { a + (4 - ar) as isize } else { a };
    let rb = if br != 0 { b + (2 - br) as isize } else { b };
    let rc = if cr != 0 { c + (4 - cr) as isize } else { c };
    let rd = if dr != 0 { d + (2 - dr) as isize } else { d };
    let mut ans = (rc - ra) / 4 * (rd - rb) / 2 * 8;
    ans += (rd - rb) * match ar { 
        0 => 0,
        1 => 5,
        2 => 2,
        3 => 1,
        _ => panic!()
    };
    ans += (rd - rb) * match cr { 
        0 => 0,
        1 => 3,
        2 => 6,
        3 => 7,
        _ => panic!()
    };
    println!("{ans}");
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
