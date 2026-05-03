// Myers file diff algorithm, yoinked from: https://github.com/prafitradimas/diff-rs/blob/main/myers/src/lib.rs

use std::collections::HashMap;

use crate::core::fs::AbsPath;

type Graph = HashMap<isize, isize>;
type Script = Vec<Graph>;

pub enum Op<'a, T> {
    Equal(&'a T),
    Insert(&'a T),
    Delete(&'a T),
}

impl AbsPath {
    /// Myers algorithm to calculate fill difference
    pub fn calc_diff(&self) {}
}

pub fn linear_diff<'a, T>(src: &'a [T], target: &'a [T]) -> Vec<Op<'a, T>>
where
    T: Eq + PartialEq,
{
    let script = shortest_edit_script(src, target);
    backtrack(src, target, script)
}

fn shortest_edit_script<'a, T>(src: &'a [T], target: &'a [T]) -> Script
where
    T: Eq + PartialEq,
{
    let n = src.len() as isize;
    let m = target.len() as isize;
    let max = n + m;

    let mut script = Script::new();

    let mut last_v = &Graph::new();
    let mut curr_v: Graph;

    for d in 0..=max {
        curr_v = Graph::new();

        for k in (-d..=d).step_by(2) {
            // `left` and `right` might be `None`
            let left = *last_v.get(&(k - 1)).unwrap_or(&0);
            let right = *last_v.get(&(k + 1)).unwrap_or(&0);

            let mut x = if k == -d || (k != d && left < right) {
                right
            } else {
                left + 1
            };

            let mut y = x - k;

            while x < n && y < m && src[x as usize] == target[y as usize] {
                x += 1;
                y += 1;
            }

            curr_v.insert(k, x);
            if x >= n && y >= m {
                script.push(curr_v);
                return script;
            }
        }

        script.push(curr_v);
        last_v = script
            .last()
            .expect("`script` should not be empty after `curr_v` is pushed");
    }

    unreachable!("impossible")
}

fn backtrack<'a, T>(src: &'a [T], target: &'a [T], script: Script) -> Vec<Op<'a, T>>
where
    T: Eq + PartialEq,
{
    let mut x = src.len() as isize;
    let mut y = target.len() as isize;

    let mut ops: Vec<Op<'a, T>> = Vec::with_capacity(script.len());

    for d in (1..script.len() as isize).rev() {
        let k = x - y;

        let prev_v = &script[(d - 1) as usize];

        // `left` and `right` might be `None`
        let left = *prev_v.get(&(k - 1)).unwrap_or(&0);
        let right = *prev_v.get(&(k + 1)).unwrap_or(&0);

        let prev_k = if k == -d || (k != d && left < right) {
            k + 1
        } else {
            k - 1
        };

        let prev_x = *prev_v.get(&prev_k).unwrap_or(&0);
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            x -= 1;
            y -= 1;

            ops.push(Op::Equal(&src[x as usize]));
        }

        if x == prev_x {
            ops.push(Op::Insert(&target[prev_y as usize]));
        } else if y == prev_y {
            ops.push(Op::Delete(&src[prev_x as usize]));
        }

        x = prev_x;
        y = prev_y;
    }

    ops.reverse();
    ops
}
