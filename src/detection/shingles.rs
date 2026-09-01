use xxhash_rust::xxh3::Xxh3;

pub(crate) const SHINGLE_WIDTH: usize = 5;

#[must_use]
pub(crate) fn build_shingles(tokens: &[u64]) -> Vec<u64> {
    if tokens.is_empty() {
        return Vec::new();
    }

    let mut shingles = Vec::with_capacity(tokens.len().saturating_sub(SHINGLE_WIDTH) + 1);
    if tokens.len() < SHINGLE_WIDTH {
        shingles.push(hash_window(tokens));
    } else {
        for window in tokens.windows(SHINGLE_WIDTH) {
            shingles.push(hash_window(window));
        }
    }

    shingles.sort_unstable();
    shingles.dedup();
    shingles
}

#[must_use]
pub(crate) fn jaccard(left: &[u64], right: &[u64]) -> f64 {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut intersection = 0usize;

    while left_index < left.len() && right_index < right.len() {
        match left[left_index].cmp(&right[right_index]) {
            std::cmp::Ordering::Less => left_index += 1,
            std::cmp::Ordering::Greater => right_index += 1,
            std::cmp::Ordering::Equal => {
                intersection += 1;
                left_index += 1;
                right_index += 1;
            }
        }
    }

    let union = left.len() + right.len() - intersection;
    if union == 0 {
        return 1.0;
    }
    intersection as f64 / union as f64
}

fn hash_window(values: &[u64]) -> u64 {
    let mut hash = Xxh3::new();
    for value in values {
        hash.update(&value.to_le_bytes());
    }
    hash.digest()
}
