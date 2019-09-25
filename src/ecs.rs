use anymap::AnyMap;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GenIndex {
    index: usize,
    generation: u64,
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
                if !entry.is_free {
                    return false;
                }
                entry.is_free = false;
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
struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

pub struct GenIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenIndexArray<T> {
    pub fn new() -> GenIndexArray<T> {
        GenIndexArray(Vec::new())
    }

    pub fn set(&mut self, index: GenIndex, value: T) {
        while index.index >= self.0.len() {
            self.0.push(None);
        }
        let mut entry = self.0.get_mut(index.index).unwrap();
        *entry = Some(ArrayEntry {value, generation: index.generation})
    }

    pub fn clear(&mut self, index: GenIndex) {
        if index.index >= self.0.len() {
            return;
        }
        let mut entry = self.0.get_mut(index.index).unwrap();
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

pub type Entity = GenIndex;
type EntityMap<T> = GenIndexArray<T>;

pub struct ECS {
    entity_allocator: GenIndexAllocator,
    components: AnyMap,
}

struct MissingComponentError;

impl From<&str> for MissingComponentError {
    fn from(s: &str) -> MissingComponentError {
        return MissingComponentError;
    }
}

impl ECS {
    pub fn register_component_type<T: 'static + Debug>(&mut self) {
        let e: EntityMap<T> = EntityMap::new();
        self.components.insert(e);
    }

    pub fn new_entity(&mut self) -> Entity {
        self.entity_allocator.allocate()
    }

    pub fn delete_entity(&mut self, entity: &Entity) {
        self.entity_allocator.deallocate(entity.clone());
    }

    pub fn add_component<T: 'static + Debug>(&mut self, entity: &Entity, component: T) {
        let comps = self.components.get_mut::<EntityMap<T>>();
        match comps {
            Some(comps) => {
                comps.set(*entity, component);
            },
            None => {}
        }
    }

    pub fn remove_component<T: 'static + Debug>(&mut self, entity: &Entity) {
        let comps = self.components.get_mut::<EntityMap<T>>();
        match comps {
            Some(comps) => {
                comps.clear(*entity);
            },
            None => {}
        }
    }

    pub fn cleanup_components<T: 'static + Debug>(&mut self) {
        let mut comps_to_remove = vec![];
        if let Some(comps) = self.components.get::<EntityMap<T>>() {
            let l = comps.0.len();
            for i in 0..l {
                if let Some(e) = &comps.0[i] {
                    let e = GenIndex {index: i, generation: e.generation};
                    if !self.entity_allocator.is_live(e) {
                        comps_to_remove.push(e);
                    }
                }
            }
        }
        for e in comps_to_remove {
            println!("removing");
            self.remove_component::<T>(&e);
        }
    }

    pub fn read<T: 'static>(&self, entity: &Entity) -> Option<&T>{
        let i = self.components.get::<EntityMap<T>>().unwrap().0.get(entity.index);
        match i {
            Some(i) => {
                match i {
                    Some(i) => {
                        return Some(&i.value);
                    },
                    None => {}
                }
            }
            None => {}
        }
        return None;
    }

    pub fn write<T: 'static>(&mut self, entity: &Entity) -> Option<&mut T>{
        if let Some(map) = self.components.get_mut::<EntityMap<T>>() {
            if let Some(Some(val)) = map.0.get_mut(entity.index) {
                return Some(&mut val.value);
            }
        }
        return None;
    }

    pub fn new() -> ECS  {
        ECS {
            entity_allocator: GenIndexAllocator::new(),
            components: AnyMap::new(),
        }
    }
}

impl Default for ECS {
    fn default() -> Self {
        ECS::new()
    }
}
