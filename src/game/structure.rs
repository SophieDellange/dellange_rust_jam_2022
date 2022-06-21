use std::collections::{HashMap, HashSet};

pub struct BlobBody<T>(HashMap<(i8, i8), T>);

type BlockType = (i8, i8);

impl<T> BlobBody<T>
where
    T: Clone,
{
    pub fn new(hearth_item: T) -> Self {
        let mut blob = Self(HashMap::new());
        blob.insert((0, 0), hearth_item);
        blob
    }

    pub fn insert(&mut self, index: BlockType, element: T) {
        self.0.insert(index, element);
    }

    pub fn detach(&mut self, block: BlockType) -> Option<HashMap<(i8, i8), T>> {
        if block == (0, 0) {
            return Some(self.0.drain().collect());
        }

        if let Some(removed) = self.0.remove(&block) {
            return Some([(block, removed)].iter().cloned().collect());
        }

        None
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
    fn kill_a_leaf() {
        let mut blob = default_blob();
        let dropped = blob.detach((3, 0)).unwrap();
        assert_eq!(1, dropped.len());
        assert_eq!(3, blob.len());
    }

    #[test]
    fn test_connections() {
        let blob = default_blob();
        assert_eq!(4, blob.len());
        //assert_eq!(4, blob.find_connected(&(0, 0)).unwrap().len());
    }

    #[test]
    fn kill_a_bridge() {
        // We drop the middle-element near the heart, all three elements should fall
        let mut blob = default_blob();
        let dropped = blob.detach((1, 0)).unwrap();
        //assert_eq!(3, dropped.len());
        //assert_eq!(1, blob.len());
    }

    fn default_blob() -> BlobBody<i32> {
        let mut blob = BlobBody::new(0);

        blob.0.insert((1, 0), 1);
        blob.0.insert((2, 0), 2);
        blob.0.insert((3, 0), 3);

        blob
    }
}
