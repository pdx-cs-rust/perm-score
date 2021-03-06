//! Produce a permutation of a given sequence with
//! an optimal score according some given scoring function.

/// Returns a permutation that achieves the maximum score
/// over all permutations of the given input sequence `seq`
/// as scored by the scoring function `score`.
///
/// # Examples
///
/// ```
/// let (s, p) = perm_score::max_perm(
///     &[1, 2, 3],
///     |s| s[0]*s[0]*s[0] + s[1]*s[1] + s[2],
/// );
/// assert_eq!((32, [3, 2, 1].as_ref()), (s, &*p));
/// ```
pub fn max_perm<T, F>(seq: &[T], mut score: F) -> (i64, Vec<T>)
where
    T: Clone,
    F: FnMut(&[T]) -> i64,
{
    let mut max_score = score(seq);
    let mut seq = seq.to_vec();
    let mut max_seq = seq.clone();
    for p in permutohedron::Heap::new(&mut seq) {
        let s = score(&p);
        if s > max_score {
            max_score = s;
            max_seq = p;
        }
    }
    (max_score, max_seq)
}
