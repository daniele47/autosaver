// Myers file diff algorithm, yoinked from: https://github.com/prafitradimas/diff-rs/blob/main/myers/src/lib.rs

use std::collections::HashMap;

use crate::core::{error::Result, fs::AbsPath};

type Graph = HashMap<isize, isize>;
type Script = Vec<Graph>;

enum Op<'a, T> {
    Equal(&'a T),
    Insert(&'a T),
    Delete(&'a T),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineDiff {
    Equal(String),
    Insert(String),
    Delete(String),
}

impl AbsPath {
    /// Myers algorithm to calculate fill difference
    pub fn calc_diff(&self, other: &AbsPath) -> Result<Vec<LineDiff>> {
        let line_reader1 = self.line_reader()?;
        let line_reader2 = other.line_reader()?;

        let mut lines1 = vec![];
        for line in line_reader1.into_iter() {
            lines1.push(line?);
        }
        let mut lines2 = vec![];
        for line in line_reader2.into_iter() {
            lines2.push(line?);
        }

        let res = linear_diff(&lines1, &lines2);

        let mut linediff = vec![];
        for l in res {
            let lowned = match l {
                Op::Equal(l) => LineDiff::Equal(l.clone()),
                Op::Insert(l) => LineDiff::Insert(l.clone()),
                Op::Delete(l) => LineDiff::Delete(l.clone()),
            };
            linediff.push(lowned);
        }
        Ok(linediff)
    }
}

fn linear_diff<'a, T>(src: &'a [T], target: &'a [T]) -> Vec<Op<'a, T>>
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

#[cfg(test)]
mod tests {
    use crate::core::fs::LineWriter;

    use super::*;

    fn purge_path_even_on_panic(tmpdir: &AbsPath) -> impl Drop {
        struct Guard(AbsPath);
        impl Drop for Guard {
            fn drop(&mut self) {
                let _ = self.0.purge_path(true);
            }
        }
        Guard(tmpdir.clone())
    }

    #[test]
    fn test_calc_diff() -> Result<()> {
        let tmp = AbsPath::new_tmp("test_read_write_lines");
        tmp.create_dir()?;
        let _guard = purge_path_even_on_panic(&tmp);

        let test_file1 = tmp.joins(&["test1.txt"]);
        let lines_in1 = vec!["first line", "second line", "third line"];
        let test_file2 = tmp.joins(&["test2.txt"]);
        let lines_in2 = vec!["first line", "diff line", "second line", "added line"];

        // Write lines
        let mut writer = test_file1.line_writer()?;
        writer.write_all_lines(lines_in1.iter())?;
        let mut writer = test_file2.line_writer()?;
        writer.write_all_lines(lines_in2.iter())?;

        // calculate diff
        let diff = test_file1.calc_diff(&test_file2)?;

        assert_eq!(diff.len(), 4);
        assert!(matches!(&diff[0], LineDiff::Insert(x) if x == "diff line"));
        assert!(matches!(&diff[1], LineDiff::Equal(x) if x == "second line"));
        assert!(matches!(&diff[2], LineDiff::Delete(x) if x == "third line"));
        assert!(matches!(&diff[3], LineDiff::Insert(x) if x == "added line"));

        Ok(())
    }
}
