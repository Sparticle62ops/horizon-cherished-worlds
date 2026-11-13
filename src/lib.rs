//! Cherished Worlds for Horizon (behavioral clean-room port of the LGPL-3.0 mod).
//!
//! Upstream lets players favorite (pin/bookmark) worlds so they always sort to
//! the top of the world list and are protected from deletion. The world-list
//! screen and its buttons are host seams (see README); this crate carries the
//! policy logic: the favorite set, the sort order, and the delete guard.

use std::collections::BTreeSet;

/// The favorite set, keyed by the world's unique key (folder name + level
/// name as the launcher addresses it). BTreeSet keeps the stored order stable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Favorites {
    set: BTreeSet<String>,
}

impl Favorites {
    pub fn load(entries: impl IntoIterator<Item = String>) -> Self {
        Self { set: entries.into_iter().collect() }
    }

    pub fn is_favorite(&self, key: &str) -> bool {
        self.set.contains(key)
    }

    /// Toggle and report the new state (true = now a favorite).
    pub fn toggle(&mut self, key: &str) -> bool {
        if self.set.remove(key) { false } else {
            self.set.insert(key.to_string());
            true
        }
    }

    pub fn unfavorite(&mut self, key: &str) {
        self.set.remove(key);
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Sort worlds for the list: favorites first, then most recently played,
    /// stable within equal keys so the order is deterministic.
    pub fn sort_worlds(&self, worlds: &mut [(String, u64)]) {
        worlds.sort_by(|a, b| {
            let fa = self.is_favorite(&a.0);
            let fb = self.is_favorite(&b.0);
            fb.cmp(&fa).then(b.1.cmp(&a.1))
        });
    }

    /// Delete guard: a favorite world cannot be deleted until it is unfavored.
    pub fn can_delete(&self, key: &str) -> bool {
        !self.set.contains(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worlds() -> Vec<(String, u64)> {
        vec![
            ("alpha".into(), 100),
            ("beta".into(), 900),
            ("gamma".into(), 500),
        ]
    }

    #[test]
    fn toggle_flips_state() {
        let mut f = Favorites::default();
        assert!(!f.is_favorite("alpha"));
        assert!(f.toggle("alpha"));
        assert!(f.is_favorite("alpha"));
        assert!(!f.toggle("alpha"));
        assert!(!f.is_favorite("alpha"));
    }

    #[test]
    fn favorites_sort_first_then_by_recency() {
        let mut f = Favorites::default();
        f.toggle("gamma");
        let mut ws = worlds();
        f.sort_worlds(&mut ws);
        // gamma (favorite) first, then beta (900) then alpha (100).
        assert_eq!(ws[0].0, "gamma");
        assert_eq!(ws[1].0, "beta");
        assert_eq!(ws[2].0, "alpha");
    }

    #[test]
    fn two_favorites_keep_recency_order() {
        let mut f = Favorites::default();
        f.toggle("alpha");
        f.toggle("beta");
        let mut ws = worlds();
        f.sort_worlds(&mut ws);
        assert_eq!(ws[0].0, "beta");
        assert_eq!(ws[1].0, "alpha");
        assert_eq!(ws[2].0, "gamma");
    }

    #[test]
    fn no_favorites_sorts_by_recency_only() {
        let mut ws = worlds();
        Favorites::default().sort_worlds(&mut ws);
        assert_eq!(ws.iter().map(|w| w.0.clone()).collect::<Vec<_>>(), ["beta", "gamma", "alpha"]);
    }

    #[test]
    fn favorites_cannot_be_deleted() {
        let mut f = Favorites::default();
        f.toggle("beta");
        assert!(!f.can_delete("beta"));
        f.unfavorite("beta");
        assert!(f.can_delete("beta"));
    }

    #[test]
    fn load_and_count() {
        let f = Favorites::load(["a".into(), "b".into()]);
        assert_eq!(f.len(), 2);
        assert!(!f.is_empty());
        assert!(Favorites::default().is_empty());
    }

    #[test]
    fn unknown_keys_do_not_panic() {
        let f = Favorites::load(["a".into()]);
        assert!(!f.is_favorite("zzz"));
        assert!(f.can_delete("zzz"));
    }
}
