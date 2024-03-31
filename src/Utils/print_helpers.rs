pub fn print_hashmap<K, V>(hashmap: &HashMap<K, V>)
where
    K: std::fmt::Debug + std::hash::Hash + Ord,
    V: std::fmt::Debug,
{
    let mut sorted_keys: Vec<_> = hashmap.keys().collect();
    sorted_keys.sort(); // Sort keys

    for key in sorted_keys {
        if let Some(value) = hashmap.get(key) {
            println!("{:?}: {:?}", key, value);
        }
    }
}

pub fn print_vector<T>(vector: &[T], message: &str)
where
    T: std::fmt::Debug,
{
    println!("{}", message);
    for (index, value) in vector.iter().enumerate() {
        println!("{}: {:?}", index, value);
    }
}
