// use and pub problem solutions
// 1st method:
use std::collections::*;  // the blank is filled with '*'

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}

// 2nd method:
use std::collections::{HashMap, BTreeMap, HashSet};  // the blank is filled with '{HashMap, BTreeMap, HashSet}'

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}