use std::collections::BTreeMap;

use crate::analysis::SourceLocation;

use super::{CloneGroup, CloneKind};

/// Connects duplicate relations between dense member indices into groups.
pub(super) struct GroupBuilder {
    parent: Vec<usize>,
    size: Vec<usize>,
    kind: Vec<CloneKind>,
    minimum_similarity: Vec<f64>,
    active: Vec<bool>,
    relation_count: usize,
}

impl GroupBuilder {
    pub(super) fn new(member_count: usize) -> Self {
        Self {
            parent: (0..member_count).collect(),
            size: vec![1; member_count],
            kind: vec![CloneKind::Type1; member_count],
            minimum_similarity: vec![1.0; member_count],
            active: vec![false; member_count],
            relation_count: 0,
        }
    }

    pub(super) fn add(&mut self, left: usize, right: usize, kind: CloneKind, similarity: f64) {
        self.relation_count += 1;
        let left = self.find(left);
        let right = self.find(right);
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

    /// Builds one group for each connected component, in unspecified order.
    pub(super) fn finish<F>(mut self, location: F) -> Vec<CloneGroup>
    where
        F: Fn(usize) -> SourceLocation,
    {
        let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for index in 0..self.parent.len() {
            let root = self.find(index);
            if self.active[root] {
                members.entry(root).or_default().push(index);
            }
        }

        members
            .into_iter()
            .map(|(root, indices)| {
                let instances = indices.into_iter().map(&location).collect();
                CloneGroup::new(self.kind[root], self.minimum_similarity[root], instances)
            })
            .collect()
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
