// https://atcoder.jp/contests/APG4b/tasks/APG4b_bx

use fixedbitset::FixedBitSet;
use itertools::Itertools as _;

fn main() {
    // use std::io::{self, Read as _};
    //
    // let mut input = "".to_owned();
    // io::stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_whitespace();
    // macro_rules! read {
    //      ([$t:tt; $n:expr]) => {
    //          (0..$n).map(|_| read!($t)).collect::<Vec<_>>()
    //      };
    //      (($($t:tt),+)) => {
    //          ($(read!($t)),*)
    //      };
    //      (_1based) => {
    //          read!(usize) - 1
    //      };
    //      (_bytes) => {
    //          read!(String).into_bytes()
    //      };
    //      ($ty:ty) => {
    //          input.next().unwrap().parse::<$ty>().unwrap()
    //      };
    //  }
    //
    // let n = read!(usize);
    // let a = read!([usize; n]);
    // let m = read!(usize);
    // let b = read!([usize; m]);
    // let arg0 = read!(String);
    // let args = read!([usize; if arg0 == "subtract" { 1 } else { 0 }]);

    use proconio::input;

    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        arg0: String,
        args: [usize; if arg0 == "subtract" { 1 } else { 0 }],
    }

    let (a, b) = (a.into_iter().collect(), b.into_iter().collect());

    print_set(&match (&*arg0, &*args) {
        ("intersection", []) => intersection(&a, &b),
        ("union_set", []) => union_set(&a, &b),
        ("symmetric_diff", []) => symmetric_diff(&a, &b),
        ("subtract", &[x]) => subtract(a, x),
        ("increment", []) => increment(&a),
        ("decrement", []) => decrement(&a),
        _ => unreachable!(),
    });
}

fn print_set(set: &FixedBitSet) {
    println!("{}", (0..50).filter(|&i| set[i]).format(" "));
}

fn intersection(a: &FixedBitSet, b: &FixedBitSet) -> FixedBitSet {
    a & b
}

fn union_set(a: &FixedBitSet, b: &FixedBitSet) -> FixedBitSet {
    a | b
}

fn symmetric_diff(a: &FixedBitSet, b: &FixedBitSet) -> FixedBitSet {
    a ^ b
}

fn subtract(mut a: FixedBitSet, x: usize) -> FixedBitSet {
    // > xは存在することが保証される。
    a.set(x, false);
    a
}

fn increment(a: &FixedBitSet) -> FixedBitSet {
    a.ones().map(|x| (x + 1) % 50).collect()
}

fn decrement(a: &FixedBitSet) -> FixedBitSet {
    a.ones().map(|x| (x + 49) % 50).collect()
}