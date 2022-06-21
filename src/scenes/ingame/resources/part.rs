use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

//as in monster part
#[derive(Component, PartialEq, Clone)]
pub struct Part {}

#[derive(Component)]
pub struct PartBlob(HashMap<(i8, i8), Part>);

impl Default for PartBlob {
    fn default() -> Self {
        let mut its_a_me = Self(HashMap::new());
        its_a_me.0.insert((0, 0), Part {});
        its_a_me
    }
}

impl PartBlob {
    /// Detach one of the blocks from the PartBlob,
    /// Results with None if the block does not exists
    /// Results with an HashMap of the lost Parts, including one removed
    pub fn detach(&mut self, block: (i8, i8)) -> Option<HashMap<(i8, i8), Part>> {
        // Remove the `heart` remove everything.
        if block == (0, 0) {
            return Some(self.0.drain().collect());
        }

        // Find out what is left after removing the single piece
        if let Some(removed) = self.0.remove(&block) {
            let to_be_retained = self.find_connected(&(0, 0));

            return Some([(block, removed)].iter().cloned().collect());
        }

        None
    }

    /// Considers the valid connection only on 4 sides, not diagonals.
    //   [_]
    //   [*]   << connected
    //      [_]   << disconnected
    //
    pub fn find_connected(&self, center: &(i8, i8)) -> Option<HashSet<(i8, i8)>> {
        self.0
            .iter()
            .fold(HashSet::new::<(i8, i8)>(), |mut p, entry| p)
            .collect()
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use super::Part;
    use super::PartBlob;

    #[test]
    fn drop_the_heart() {
        let mut blob = default_blob();

        let dropped = blob.detach((0, 0)).unwrap();
        assert_eq!(4, dropped.len());
    }

    #[test]
    fn kill_a_leaf() {
        let mut blob = default_blob();
        let dropped = blob.detach((3, 0)).unwrap();
        assert_eq!(1, dropped.len());
        assert_eq!(3, blob.0.len());
    }

    #[test]
    fn test_connections() {
        let blob = default_blob();
        assert_eq!(4, blob.0.len());
        assert_eq!(4, blob.find_connected(&(0, 0)).unwrap().len());
    }

    #[test]
    fn kill_a_bridge() {
        // We drop the middle-element near the heart, all three elements should fall
        let mut blob = default_blob();
        let dropped = blob.detach((1, 0)).unwrap();
        assert_eq!(3, dropped.len());
        assert_eq!(1, blob.0.len());
    }

    fn default_blob() -> PartBlob {
        let mut blob = PartBlob::default();

        blob.0.insert((1, 0), Part {});
        blob.0.insert((2, 0), Part {});
        blob.0.insert((3, 0), Part {});

        blob
    }
}
