#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use anymap::AnyMap;
use std::fmt::Debug;
use std::collections::HashMap;

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

pub type Entity = GenIndex;
type EntityMap<T> = GenIndexArray<T>;

pub trait Mutation {
    fn apply(&self, ecs: &mut ECS);
}

pub type Mutations = Vec<Box<dyn Mutation>>;
pub type System = dyn Fn(&ECS) -> Mutations;
pub type GameStateId = u32;


pub struct ECS {
    entity_allocator: GenIndexAllocator,
    components: AnyMap,
    resources: AnyMap,
    systems: Vec<Box<System>>,
}

#[macro_use]
macro_rules! impl_read {
        ( $fn_name:ident, [$($r:ident),*] ) => {
            pub fn $fn_name<$($r: 'static),*>(&self) -> Vec<($(&$r),*, Entity)> {

            let mut minlen = 1000;
        $(
            let $r = self.components.get::<EntityMap<$r>>().unwrap();
            minlen = std::cmp::min(minlen, $r.0.len());
        )*

            let mut res = Vec::new();
            let num_iter = minlen;
            for i in 0..num_iter {
                $(
                    let $r = $r.0.get(i);
                )*
                match ($($r),*) {
                    ($(Some($r)),*) => {
                        match ($($r),*) {
                            ($(Some($r)),*) => {
                                let mut generation = 0;
                                $(
                                    if generation == 0 {
                                        generation = $r.generation;
                                    }
                                    if generation != $r.generation {
                                        continue;
                                    }
                                )*
                                let e = Entity {index: i, generation: generation };
                                res.push(($(&$r.value),*, e));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            return res;
        }
    }
}

#[macro_use]
macro_rules! impl_get {
        ( $fn_name:ident, [$($r:ident),*] ) => {
            pub fn $fn_name<$($r: 'static),*>(&self) -> Vec<Entity> {

            let mut minlen = 1000;
        $(
            let $r = self.components.get::<EntityMap<$r>>().unwrap();
            minlen = std::cmp::min(minlen, $r.0.len());
        )*

            let mut res = Vec::new();
            let num_iter = minlen;
            for i in 0..num_iter {
                $(
                    let $r = $r.0.get(i);
                )*
                match ($($r),*) {
                    ($(Some($r)),*) => {
                        match ($($r),*) {
                            ($(Some($r)),*) => {
                                let mut generation = 0;
                                $(
                                    if generation == 0 {
                                        generation = $r.generation;
                                    }
                                    if generation != $r.generation {
                                        continue;
                                    }
                                )*
                                let e = Entity {index: i, generation: generation };
                                res.push(e);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            return res;
        }
    }
}

struct MissingComponentError;

impl From<&str> for MissingComponentError {
    fn from(_: &str) -> MissingComponentError {
        return MissingComponentError;
    }
}

impl ECS {
    pub fn add_system(&mut self, system: Box<System>) {
        self.systems.push(system);
    }

    pub fn clear_systems(&mut self) {
        self.systems.clear();
    }

    pub fn register_component_type<T: 'static + Debug>(&mut self) {
        let e: EntityMap<T> = EntityMap::new();
        self.components.insert(e);
    }

    pub fn run_systems(&mut self) {
        let num_systems = self.systems.len();
        for i in 0..num_systems {
            let mutations = self.systems[i](&self);
            for mutation in mutations {
                mutation.apply(self);
            }
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        self.entity_allocator.allocate()
    }

    pub fn build_entity(&mut self) -> EntityBuilder {
        let e = self.entity_allocator.allocate();
        EntityBuilder {
            ecs: self,
            entity: e,
        }
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

    pub fn add_res<T: 'static>(&mut self, value: T) {
        self.resources.insert(value);
    }

    pub fn read_res<T: 'static>(&self) -> &T {
        self.resources.get::<T>().unwrap()
    }

    pub fn write_res<T: 'static>(&mut self) -> &mut T {
        self.resources.get_mut::<T>().unwrap()
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

    impl_read!(read1, [t1]);
    impl_read!(read2, [t1, t2]);
    impl_read!(read3, [t1, t2, t3]);
    impl_read!(read4, [t1, t2, t3, t4]);
    impl_read!(read5, [t1, t2, t3, t4, t5]);

    impl_get!(get1, [t1]);
    impl_get!(get2, [t1, t2]);
    impl_get!(get3, [t1, t2, t3]);
    impl_get!(get4, [t1, t2, t3, t4]);
    impl_get!(get5, [t1, t2, t3, t4, t5]);

    pub fn new() -> ECS  {
        ECS {
            entity_allocator: GenIndexAllocator::new(),
            components: AnyMap::new(),
            resources: AnyMap::new(),
            systems: Vec::new(),
        }
    }
}

impl Default for ECS {
    fn default() -> Self {
        ECS::new()
    }
}

pub struct EntityBuilder<'a> {
    ecs: &'a mut ECS,
    entity: Entity,
}

impl <'a> EntityBuilder<'a> {
    pub fn add<T: 'static + Debug>(mut self, c: T) -> EntityBuilder<'a> {
        self.ecs.add_component(&self.entity, c);
        self
    }

    pub fn entity(&mut self) -> Entity {
        self.entity.clone()
    }
}
