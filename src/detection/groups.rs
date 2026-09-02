use std::collections::BTreeMap;

use super::{BlockId, CloneGroup, CloneKind, EligibleBlock, Pair};

pub(super) struct GroupBuilder {
    parent: Vec<usize>,
    size: Vec<usize>,
    kind: Vec<CloneKind>,
    minimum_similarity: Vec<f64>,
    active: Vec<bool>,
    relation_count: usize,
}

impl GroupBuilder {
    pub(super) fn new(block_count: usize) -> Self {
        Self {
            parent: (0..block_count).collect(),
            size: vec![1; block_count],
            kind: vec![CloneKind::Type1; block_count],
            minimum_similarity: vec![1.0; block_count],
            active: vec![false; block_count],
            relation_count: 0,
        }
    }

    pub(super) fn add(&mut self, pair: Pair, kind: CloneKind, similarity: f64) {
        self.relation_count += 1;
        let left = self.find(pair.0.0);
        let right = self.find(pair.1.0);
        if left == right {
            self.kind[left] = self.kind[left].max(kind);
            self.minimum_similarity[left] = self.minimum_similarity[left].min(similarity);
            return;
        }

        let (root, child) = if self.size[left] > self.size[right]
            || self.size[left] == self.size[right] && left < right
        {
            (left, right)
        } else {
            (right, left)
        };
        self.parent[child] = root;
        self.size[root] += self.size[child];
        self.kind[root] = self.kind[root].max(self.kind[child]).max(kind);
        self.minimum_similarity[root] = self.minimum_similarity[root]
            .min(self.minimum_similarity[child])
            .min(similarity);
        self.active[root] = true;
    }

    pub(super) fn finish(mut self, blocks: &[EligibleBlock<'_>]) -> Vec<CloneGroup> {
        let mut members: BTreeMap<usize, Vec<BlockId>> = BTreeMap::new();
        for index in 0..self.parent.len() {
            let root = self.find(index);
            if self.active[root] {
                members.entry(root).or_default().push(BlockId(index));
            }
        }

        let mut groups: Vec<_> = members
            .into_iter()
            .map(|(root, ids)| {
                let instances = ids
                    .into_iter()
                    .map(|id| blocks[id.0].block.location.clone())
                    .collect();
                CloneGroup::new(self.kind[root], self.minimum_similarity[root], instances)
            })
            .collect();
        groups.sort_unstable_by(|left, right| {
            left.instances
                .cmp(&right.instances)
                .then(left.kind.cmp(&right.kind))
        });
        groups
    }

    #[cfg(test)]
    pub(super) const fn relation_count(&self) -> usize {
        self.relation_count
    }

    fn find(&mut self, index: usize) -> usize {
        let mut root = index;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        let mut current = index;
        while self.parent[current] != current {
            let parent = self.parent[current];
            self.parent[current] = root;
            current = parent;
        }
        root
    }
}
