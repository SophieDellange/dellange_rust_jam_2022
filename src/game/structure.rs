use std::collections::{HashMap, HashSet};

pub struct BlobBody<T>(HashMap<(i8, i8), T>);

type Coordinates = (i8, i8);

impl<T> BlobBody<T>
where
    T: Clone,
{
    pub fn new(hearth_item: T) -> Self {
        let mut blob = Self(HashMap::new());
        blob.insert((0, 0), hearth_item);
        blob
    }

    pub fn insert(&mut self, index: Coordinates, element: T) {
        self.0.insert(index, element);
    }

    pub fn detach(&mut self, block: Coordinates) -> Option<HashMap<Coordinates, T>> {
        if block == (0, 0) {
            return Some(self.0.drain().collect());
        }

        if self.0.contains_key(&block) {
            let removed = self.0.remove_entry(&block).unwrap();

            let conserve = self.find_alive_blocks();

            let dead_blocks = self
                .0
                .iter()
                .filter(|&p| !conserve.contains(&p.0))
                .map(|x| x.0.clone())
                .collect::<HashSet<Coordinates>>();

            let mut dropped = dead_blocks
                .iter()
                .flat_map(|r| self.0.remove_entry(r))
                .collect::<HashMap<Coordinates, T>>();

            dropped.insert(removed.0, removed.1);
            return Some(dropped);
        }

        None
    }

    pub fn find_alive_blocks(&self) -> HashSet<Coordinates> {
        let mut connected = HashSet::<Coordinates>::new();
        let mut find_neighboor = self.has_neighboor(&(0, 0));

        loop {
            let found = find_neighboor
                .iter()
                .map(|p| self.has_neighboor(p))
                .flatten()
                .collect::<HashSet<Coordinates>>();

            let new_dots = found
                .difference(&connected)
                .cloned()
                .collect::<HashSet<Coordinates>>();

            if new_dots.len() < 1 {
                break;
            }

            find_neighboor.clear();
            new_dots.iter().for_each(|p| {
                find_neighboor.insert(*p);
                connected.insert(*p);
            });
        }

        connected
    }

    /// returns neighboor coordinates
    pub fn has_neighboor(&self, block: &Coordinates) -> HashSet<Coordinates> {
        let directions: [Coordinates; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut found = HashSet::<Coordinates>::new();

        for p in directions.iter() {
            let check = (block.0 + p.0, block.1 + p.1);

            if self.0.contains_key(&check) {
                found.insert(check);
            }
        }
        found
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod test {

    use super::BlobBody;

    #[test]
    fn drop_the_heart() {
        let mut blob = default_blob();

        let dropped = blob.detach((0, 0)).unwrap();
        assert_eq!(4, dropped.len());
    }
    #[test]
    fn find_alive_blocks() {
        let blob = default_blob();
        let all = blob.find_alive_blocks();
        assert_eq!(all.len(), blob.len());
    }

    #[test]
    fn kill_a_leaf() {
        let mut blob = default_blob();
        let dropped = blob.detach((3, 0)).unwrap();
        assert_eq!(1, dropped.len());
        assert_eq!(3, blob.len());
    }

    #[test]
    fn test_neighboor_finder() {
        let blob = default_blob();
        let part = blob.has_neighboor(&(0, 0));
        assert_eq!(1, part.len());
        assert_eq!(&(1, 0), part.iter().next().unwrap());

        let part = blob.has_neighboor(&(1, 0));
        assert_eq!(2, part.len());
        assert!(part.contains(&(0, 0)));
        assert!(part.contains(&(2, 0)));
    }

    #[test]
    fn kill_a_bridge() {
        // We drop the middle-element near the heart, all three elements should fall
        let mut blob = default_blob();
        let dropped = blob.detach((1, 0)).unwrap();
        assert_eq!(3, dropped.len());
        assert_eq!(1, blob.len());
    }

    fn default_blob() -> BlobBody<i32> {
        let mut blob = BlobBody::new(0);

        blob.0.insert((1, 0), 1);
        blob.0.insert((2, 0), 2);
        blob.0.insert((3, 0), 3);

        blob
    }
}
