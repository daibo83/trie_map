use yada::{DoubleArray, builder::DoubleArrayBuilder};

pub struct Map<V>
{
    pub trie: DoubleArray<Vec<u8>>,
    pub data: Vec<V>,
}

impl<V> Map<V> 
{
    /// Creates a new map from a iterator of key, value pairs that has to be presorted
    pub fn build(pairs: impl Iterator<Item = (String, V)>, size_hint: usize) -> Option<Self> {
        let mut keys: Vec<(String, u32)> = Vec::with_capacity(size_hint);
        let mut data: Vec<V> = Vec::with_capacity(size_hint);
        for (index, (key, value)) in pairs.enumerate() {
            keys.push((key, index as u32));
            data.push(value);
        }
        Some(Map {
            trie: DoubleArray::new(DoubleArrayBuilder::build(&keys)?),
            data,
        })
    }
    /// Returns the value associated with the key
    /// Returns None if the key is not found
    pub fn get(&self, key: &str) -> Option<&V> {
        self.trie.exact_match_search(key.as_bytes()).map(|index| &self.data[index as usize])
    }
}