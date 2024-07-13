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

fn bfs(tree: &[Vec<usize>], start: usize) -> (usize, usize) {
    let mut d = vec![INF; tree.len()];
    d[start] = 0;
    let mut que = VecDeque::new();
    que.push_back(start);
    while let Some(v) = que.pop_front() {
        for &nx in &tree[v] {
            if d[nx] != INF { continue }
            d[nx] = d[v] + 1;
            que.push_back(nx);
        }
    }
    let dist = *d.iter().max().unwrap();
    let e = d.iter().enumerate().find(|(_, &d)| d == dist).unwrap().0;
    (e, dist)
}

fn diam(tree: &[Vec<usize>]) -> (usize, usize) {
    let (st, _) = bfs(tree, 0);
    bfs(tree, st)
}

fn dfs(tree: &[Vec<usize>], st: usize) -> Vec<usize> {
    let mut visited = FxHashSet::from_iter([st]);
    let mut stack = VecDeque::from_iter([(st, 2)]);
    let mut ans = vec![];
    while let Some((e, s)) = stack.pop_back() {
        if s == 0 {
            ans.push(e);
        }

        for &nx in &tree[e] {
            if visited.contains(&nx) { continue }
            visited.insert(nx);
            stack.push_back((nx, (s + 1) % 3));
        }
    }
    ans
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n-1]
    }

    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }

    let (h, _) = diam(&g);

    let core = dfs(&g, h);

    println!("{}", core.iter().map(|&x| g[x].len()).sorted().join(" "));
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
