/*! 
The trigram library computes the similarity of strings.
prints out the trigram similarity of two strings
using what appears to be the same algorithm used by Postgres.
https://www.postgresql.org/docs/9.1/pgtrgm.html
*/

use std::collections::HashSet;
use std::hash::Hash;

fn distance(a: &String, b: &String) -> f32 {
    1.0 - similarity(a, b)
}

fn similarity(a: &String, b: &String) -> f32 {
    let ta = trigrams(a);
    let tb = trigrams(b);
    return jaccard(ta, tb);
}

fn trigrams(s: &String) -> HashSet<String> {
    let mut ts = HashSet::new();
    let s = format!("{} ", s);
    let mut p1 = ' ';
    let mut p2 = ' ';
    for c in s.chars() {
        let v = vec![p1, p2, c];
        let t: String = v.into_iter().collect();
        ts.insert(t);
        p1 = p2;
        p2 = c;
    }
    ts
}

fn jaccard<T>(s1: HashSet<T>, s2: HashSet<T>) -> f32 where T: Hash+Eq {
    let i = s1.intersection(&s2).count() as f32;
    let u = s1.union(&s2).count() as f32;
    return i / u;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_string() {
        let strs = vec!["", "a", "ab", "abc", "abcd"];
        for a in strs {
            let a = a.to_string();
            assert_eq!(similarity(&a, &a), 1.0, "checking similarity of '{}' to itself", a);
        }
    }
}
