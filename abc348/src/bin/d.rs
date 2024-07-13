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
use itertools::Itertools;
use nalgebra::min;
use num_bigint::BigUint;
use num_integer::{gcd, gcd_lcm};
use num_traits::{abs, pow};
use proconio::{fastout, input};
use proconio::marker::{Bytes, Usize1};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

fn bfs(field: &[Vec<u8>], point: &FxHashSet<(usize, usize)>, start: (usize, usize), energy: usize) -> FxHashSet<(usize, usize)> {
    let h = field.len();
    let w = field[0].len();

    let mut visited = FxHashMap::from_iter([(start, 0)]);
    let mut queue = VecDeque::from_iter([start]);
    let mut ret = FxHashSet::default();
    while let Some((r, c)) = queue.pop_front() {
        if visited[&(r, c)] == energy { continue; }
        for (dx, dy) in DIR4 {
            let nr = r.wrapping_add_signed(dy);
            let nc = c.wrapping_add_signed(dx);
            if nr >= h || nc >= w || field[nr][nc] == b'#' || visited.contains_key(&(nr, nc)) { continue; }
            visited.insert((nr, nc), visited[&(r, c)] + 1);
            if point.contains(&(nr, nc)) {
                ret.insert((nr, nc));
                continue;
            }
            queue.push_back((nr, nc));
        }
    }
    ret
}

fn dfs() {

}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Bytes; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n]
    }

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (r, row) in a.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == b'S' {
                start = (r, c);
            }
            if ch == b'T' {
                end = (r, c);
            }
        }
    }

    let mut point = FxHashSet::from_iter([end]);
    for &(r, c, _) in &rce {
        point.insert((r, c));
    }
    if !point.contains(&start) {
        println!("No");
        return;
    }

    let mut reachable = vec![];
    for &(r, c, e) in &rce {
        reachable.push(bfs(&a, &point, (r, c), e));
    }

    let mut map = FxHashMap::default();
    for (i, &(r, c, _)) in rce.iter().enumerate() {
        map.insert((r, c), i);
    }

    let mut g = vec![vec![]; n];
    let st = map[&start];
    for (i, rc) in reachable.iter().enumerate() {
        for &(r, c) in rc {
            if (r, c) == end {
                g[i].push(INF);
                continue;
            }
            g[i].push(map[&(r, c)]);
        }
    }

    println!("{g:?}");
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
