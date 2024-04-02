// Generation Index
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GenIndex {
    pub index: usize,
    pub generation: u64,
}

#[derive(Eq, PartialEq, Clone)]
pub enum GenIndexAllocationStatus {
    Free,
    Live,
}

#[derive(Eq, PartialEq, Clone)]
struct GenIndexAllocationEntry {
    status: GenIndexAllocationStatus,
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
            self.entries.push(GenIndexAllocationEntry {
                status: GenIndexAllocationStatus::Live,
                generation: 1,
            });
            let index = self.entries.len() - 1;
            GenIndex {
                index,
                generation: 1,
            }
        } else {
            let e = self.free.pop().unwrap();
            let entry = self.entries.get_mut(e).unwrap();
            entry.status = GenIndexAllocationStatus::Live;
            entry.generation += 1;
            GenIndex {
                index: e,
                generation: entry.generation,
            }
        }
    }

    pub fn deallocate(&mut self, index: GenIndex) -> bool {
        if let Some(entry) = self.entries.get_mut(index.index) {
            if entry.status == GenIndexAllocationStatus::Free {
                return false;
            }
            entry.status = GenIndexAllocationStatus::Free;
            self.free.push(index.index);
            true
        } else {
            false
        }
    }

    pub fn is_live(&self, index: GenIndex) -> bool {
        let e = self.entries.get(index.index).unwrap();
        e.generation == index.generation && e.status == GenIndexAllocationStatus::Live
    }
}

impl Default for GenIndexAllocator {
    fn default() -> Self {
        Self::new()
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
        *entry = Some(ArrayEntry {
            value,
            generation: index.generation,
        })
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
            Some(Some(entry)) => {
                if entry.generation == index.generation {
                    Some(&entry.value)
                } else {
                    None
                }
            }
            _default => None,
        }
    }

    pub fn get_mut(&mut self, index: GenIndex) -> Option<&mut T> {
        let entry = self.0.get_mut(index.index);
        match entry {
            Some(Some(entry)) => {
                if entry.generation == index.generation {
                    Some(&mut entry.value)
                } else {
                    None
                }
            }
            _default => None,
        }
    }
}

impl<T> Default for GenIndexArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{GenIndexAllocator, GenIndexArray};

    #[test]
    fn test_allocate() {
        let mut allocator = GenIndexAllocator::new();
        let index = allocator.allocate();
        assert!(allocator.is_live(index));
    }

    #[test]
    fn test_deallocate() {
        let mut allocator = GenIndexAllocator::new();
        let index = allocator.allocate();
        assert!(allocator.is_live(index));
        assert!(allocator.deallocate(index));
        assert!(!allocator.is_live(index));
    }

    #[test]
    fn can_clear() {
        let mut allocator = GenIndexAllocator::new();
        let mut sut: GenIndexArray<i32> = GenIndexArray::new();
        let index = allocator.allocate();

        sut.set(index, 5);
        sut.clear(index);
        assert!(sut.get(index).is_none())
    }

    #[test]
    fn can_get_mut() {
        let mut allocator = GenIndexAllocator::new();
        let mut sut: GenIndexArray<i32> = GenIndexArray::new();
        let index = allocator.allocate();

        sut.set(index, 5);
        let value = sut.get_mut(index);
        assert_eq!(value, Some(&mut 5));
    }

    #[test]
    fn can_get_mut_modify() {
        let mut allocator = GenIndexAllocator::new();
        let mut sut: GenIndexArray<i32> = GenIndexArray::new();
        let index = allocator.allocate();

        sut.set(index, 5);
        if let Some(value) = sut.get_mut(index) {
            *value = 10;
        }
        assert_eq!(sut.get(index), Some(&10));
    }

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
        assert!(sut.get(index) == Some(&5))
    }
}
