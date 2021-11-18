use std::ops::Deref;

use yada::{DoubleArray, builder::DoubleArrayBuilder};

pub struct Map<K, V>
    where K: Deref<Target = [u8]> + std::convert::AsRef<[u8]>
{
    trie: DoubleArray<Vec<u8>>,
    data: Vec<V>,
    phantom: std::marker::PhantomData<K>
}

impl<K, V> Map<K, V> 
    where K: Deref<Target = [u8]> + std::convert::AsRef<[u8]>
{
    /// Creates a new map from a iterator of key, value pairs that has to be presorted
    pub fn build(pairs: impl Iterator<Item = (K, V)>, size_hint: usize) -> std::io::Result<Self> {
        let mut keys: Vec<(K, u32)> = Vec::with_capacity(size_hint);
        let mut data: Vec<V> = Vec::with_capacity(size_hint);
        for (index, (key, value)) in pairs.enumerate() {
            keys.push((key, index as u32));
            data.push(value);
        }
        match DoubleArrayBuilder::build(&keys) {
            Some(bytes) => Ok(Map {
                trie: DoubleArray::new(bytes),
                data,
                phantom: std::marker::PhantomData
            }),
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to build trie"))
        }
    }
    /// Returns the value associated with the key
    /// Returns None if the key is not found
    pub fn get(&self, key: &K) -> Option<&V> {
        self.trie.exact_match_search(&key).map(|index| &self.data[index as usize])
    }
}