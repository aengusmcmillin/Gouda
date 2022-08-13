#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use anymap::AnyMap;
use std::fmt::Debug;

use crate::genindex::{GenIndex, GenIndexArray, GenIndexAllocator};

pub type Entity = GenIndex;
type EntityMap<T> = GenIndexArray<T>;

pub trait Mutation {
    fn apply(&self, ecs: &mut ECS);
}

pub type Mutations = Vec<Box<dyn Mutation>>;
pub type System = dyn Fn(&ECS, f32) -> Mutations;
pub type GameStateId = u32;

pub struct ECS {
    entity_allocator: GenIndexAllocator,
    components: AnyMap,
    resources: AnyMap,
    systems: Vec<Box<System>>,
    queued_events: AnyMap,
    processing_events: AnyMap,
}

macro_rules! impl_read {
        ( $fn_name:ident, [$($r:ident),*] ) => {
            pub fn $fn_name<$($r: 'static),*>(&self) -> Vec<($(&$r),*, Entity)> {

            let mut minlen = 1000;
        $(
            let $r = self.components.get::<EntityMap<$r>>();
            if $r.is_none() {
                return Vec::new();
            }
            minlen = std::cmp::min(minlen, $r.unwrap().0.len());
        )*

            let mut res = Vec::new();
            let num_iter = minlen;
            for i in 0..num_iter {
                $(
                    let $r = $r.unwrap().0.get(i);
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

macro_rules! impl_get {
        ( $fn_name:ident, [$($r:ident),*] ) => {
            pub fn $fn_name<$($r: 'static),*>(&self) -> Vec<Entity> {

            let mut minlen = 1000;
        $(
            let $r = self.components.get::<EntityMap<$r>>();
            if $r.is_none() {
                return Vec::new();
            }
            minlen = std::cmp::min(minlen, $r.unwrap().0.len());
        )*

            let mut res = Vec::new();
            let num_iter = minlen;
            for i in 0..num_iter {
                $(
                    let $r = $r.unwrap().0.get(i);
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

    fn register_component_type<T: 'static + Debug>(&mut self) -> &mut EntityMap<T> {
        let e: EntityMap<T> = EntityMap::new();
        self.components.insert(e);
        return self.components.get_mut::<EntityMap<T>>().unwrap();
    }

    pub fn run_systems(&mut self, dt: f32) {
        let num_systems = self.systems.len();
        for i in 0..num_systems {
            let mutations = self.systems[i](&self, dt);
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
        let mut comps = self.components.get_mut::<EntityMap<T>>();
        if (comps.is_none()) {
            comps = Some(self.register_component_type::<T>())
        }
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
            self.remove_component::<T>(&e);
        }
    }

    pub fn register_event_type<T: 'static>(&mut self) {
        self.queued_events.insert(Vec::<T>::new());
        self.processing_events.insert(Vec::<T>::new());
    }

    pub fn migrate_events<T: 'static>(&mut self) {
        self.processing_events.get_mut::<Vec<T>>().unwrap().clear();
        let v = self.queued_events.get_mut::<Vec<T>>().unwrap();
        while !v.is_empty() {
            self.processing_events.get_mut::<Vec<T>>().unwrap().push(v.pop().unwrap());
        }
    }

    pub fn push_event<T: 'static>(&mut self, event: T) {
        self.queued_events.get_mut::<Vec<T>>().unwrap().push(event);
    }

    pub fn events<T: 'static>(&self) -> &Vec<T> {
        self.processing_events.get::<Vec<T>>().unwrap()
    }

    pub fn add_res<T: 'static>(&mut self, value: T) {
        self.resources.insert(value);
    }

    pub fn remove_res<T: 'static>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn read_res<T: 'static>(&self) -> &T {
        self.resources.get::<T>().unwrap()
    }

    pub fn write_res<T: 'static>(&mut self) -> &mut T {
        self.resources.get_mut::<T>().unwrap()
    }

    pub fn read<T: 'static + Debug>(&self, entity: &Entity) -> Option<&T>{
        if let Some(map) = self.components.get::<EntityMap<T>>() {
            if let Some(Some(i)) = map.0.get(entity.index) {
                return Some(&i.value);
            }
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
            queued_events: AnyMap::new(),
            processing_events: AnyMap::new(),
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
    pub fn add<T: 'static + Debug>(self, c: T) -> EntityBuilder<'a> {
        self.ecs.add_component(&self.entity, c);
        self
    }

    pub fn entity(&mut self) -> Entity {
        self.entity.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::ECS;

    #[derive(Debug)]
    struct TestComponent {}

    #[test]
    fn can_insert_component_and_access_entity_for_it() {
        let mut ecs = ECS::new();
        let entity = ecs.new_entity();
        ecs.add_component(&entity, TestComponent {});

        let res = ecs.get1::<TestComponent>();
        assert!(res.get(0) == Some(&entity));
    }
}
