use bevy::prelude::*;
use std::collections::HashMap;

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
            // here goes the recursive connection-finding part

            /// *suggestion*: consider the valid connection only on 4 sides, not diagonals.
            ///   [_]
            ///   [*]   << connected
            ///      [_]   << disconnected
            ///
            return Some([(block, removed)].iter().cloned().collect());
        }

        None
    }
}
/*
pub fn check_valid_init(part_core: Part, parts: &mut Vec<Part>) -> Vec<Part> {
    let mut connected = Vec::new();

    //then check for connected parts
    check_valid(part_core, parts, &mut connected);

    connected
}

pub fn check_valid(current: Part, remaining: &mut Vec<Part>, connecteds: &mut Vec<Part>) {
    for part in current.connected_parts.iter() {
        match remaining.iter().position(|element| element == part) {
            Some(pos) => Some(connecteds.push(remaining.remove(pos))),
            None => None,
        };

        check_valid(part.clone(), remaining, connecteds);
    }
}*/

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
    }

    #[test]
    fn kill_a_bridge() {
        // We drop the middle-element near the heart, all three elements should fall
        let mut blob = default_blob();
        let dropped = blob.detach((1, 0)).unwrap();
        assert_eq!(3, dropped.len());
    }

    fn default_blob() -> PartBlob {
        let mut blob = PartBlob::default();

        blob.0.insert((1, 0), Part {});
        blob.0.insert((2, 0), Part {});
        blob.0.insert((3, 0), Part {});

        blob
    }
}
