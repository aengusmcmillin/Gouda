// Generation Index 
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GenIndex {
    pub index: usize,
    pub generation: u64,
}

#[derive(Eq, PartialEq, Clone)]
struct GenIndexAllocationEntry {
    is_free: bool,
    generation: u64,
}

#[derive(Eq, PartialEq, Clone)]
pub struct GenIndexAllocator {
    entries: Vec<GenIndexAllocationEntry>,
    free: Vec<usize>,
}

impl GenIndexAllocator {
    pub fn new() -> GenIndexAllocator {
        GenIndexAllocator {
            entries: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> GenIndex {
        if self.free.is_empty() {
            self.entries.push(GenIndexAllocationEntry {is_free: false, generation: 1});
            let index = self.entries.len() - 1;
            return GenIndex {index, generation: 1};
        } else {
            let e = self.free.pop().unwrap();
            let entry = self.entries.get_mut(e).unwrap();
            entry.is_free = false;
            entry.generation += 1;
            return GenIndex {index: e, generation: entry.generation};
        }
    }

    pub fn deallocate(&mut self, index: GenIndex) -> bool {
        let e = self.entries.get_mut(index.index);
        match e {
            Some(entry) => {
                if entry.is_free {
                    return false;
                }
                entry.is_free = true;
                self.free.push(index.index);
                return true;
            },
            None => {
                return false;
            }
        }
    }

    pub fn is_live(&self, index: GenIndex) -> bool {
        let e = self.entries.get(index.index).unwrap();
        return e.generation == index.generation && !e.is_free;
    }
}

#[derive(Debug)]
pub struct ArrayEntry<T> {
    pub value: T,
    pub generation: u64,
}

pub struct GenIndexArray<T>(pub Vec<Option<ArrayEntry<T>>>);

impl<T> GenIndexArray<T> {
    pub fn new() -> GenIndexArray<T> {
        GenIndexArray(Vec::new())
    }

    pub fn set(&mut self, index: GenIndex, value: T) {
        while index.index >= self.0.len() {
            self.0.push(None);
        }
        let entry = self.0.get_mut(index.index).unwrap();
        *entry = Some(ArrayEntry {value, generation: index.generation})
    }

    pub fn clear(&mut self, index: GenIndex) {
        if index.index >= self.0.len() {
            return;
        }
        let entry = self.0.get_mut(index.index).unwrap();
        *entry = None;
    }

    pub fn get(&self, index: GenIndex) -> Option<&T> {
        let entry = self.0.get(index.index);
        match entry {
            Some(entry) => {
                match entry {
                    Some(entry) => {
                        if entry.generation == index.generation {
                            Some(&entry.value)
                        } else {
                            None
                        }

                    },
                    None => { None }
                }
            },
            None => { None }
        }
    }

    pub fn get_mut(&mut self, index: GenIndex) -> Option<&mut T> {
        let entry = self.0.get_mut(index.index);
        match entry {
            Some(entry) => {
                match entry {
                    Some(entry) => {
                        if entry.generation == index.generation {
                            Some(&mut entry.value)
                        } else {
                            None
                        }

                    },
                    None => { None }
                }
            },
            None => { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GenIndexArray, GenIndexAllocator};

    #[test]
    fn can_set() {
        let mut allocator = GenIndexAllocator::new();
        let mut sut: GenIndexArray<i32> = GenIndexArray::new();
        let index = allocator.allocate();

        sut.set(index, 5);
        assert!(sut.get(index) == Some(&5))
    }

    #[test]
    fn can_deallocate() {
        let mut allocator = GenIndexAllocator::new();
        let mut sut: GenIndexArray<i32> = GenIndexArray::new();
        let index = allocator.allocate();

        sut.set(index, 5);
        allocator.deallocate(index);
        sut.clear(index);
        assert!(sut.get(index) == Some(&5))
    }
}