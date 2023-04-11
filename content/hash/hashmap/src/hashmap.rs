use std::mem;

#[derive(Debug)]
struct SlotNode<K, V> {
    key: K,
    data: V,
    next: Option<Box<SlotNode<K, V>>>,
}

#[derive(Debug)]
pub struct HashMap<K, V> {
    table: Vec<Option<Box<SlotNode<K, V>>>>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq
{
    pub fn new(capacity: usize) -> Self {
        let mut table = Vec::new();
        table.resize_with(capacity, || None);
        HashMap { table }
    }

    pub fn push(&mut self, key: K, data: V) -> Result<(), String> {
        let index = self.hash(&key);
        let head_node_ref = self.table.get_mut(index);
        if head_node_ref.is_none() {
            return Err("Map Overflow".to_string());
        }
        let head_node = head_node_ref.unwrap();

        let new_node = Box::new(SlotNode {
            key,
            data,
            next: None,
        });

        if head_node.is_none() {
            *head_node = Some(new_node);

            return Ok(());
        }

        let mut last_node = &mut head_node.as_mut().unwrap().next;
        while last_node.is_some() {
            last_node = &mut last_node.as_mut().unwrap().next;
        }

        *last_node = Some(new_node);

        Ok(())
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let index = self.hash(&key);
        let head_node = &self.table[index];
        if head_node.is_none() {
            return None
        }

        let mut current_node = head_node;
        while let Some(cur_node) = current_node {
            if cur_node.key == key {
                return Some(&cur_node.data)
            }

            current_node = &cur_node.next;
        }

        None
    }

    // Simple hashing algorithm that sums each character
    // byte, and mods the result based on the capacity.
    // Returns the hashed slot position.
    fn hash(&self, data: &K) -> usize {
        let size = mem::size_of::<K>();
        let mut bytes: Vec<u8> = vec![0, size.try_into().unwrap()];
        unsafe {
            let ptr = data as *const K as *const u8;
            ptr.copy_to_nonoverlapping(bytes.as_mut_ptr(), size);
        }

        let byte_sum: usize = bytes.iter().map(|&x| x as usize).sum();
        
        byte_sum % self.table.capacity()
    }
}
