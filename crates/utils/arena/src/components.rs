use std::fmt::Debug;
use std::collections::HashMap;
use std::any::TypeId;
use crate::NodeId;
use std::collections::hash_map::Entry;
use mopa::Any;

pub trait Resource: Any + Send + Sync + 'static + Debug {}

impl<T> Resource for T where T: Any + Send + Sync + Debug {}

pub trait Component : Send + Sync + 'static + Debug {}
impl<T> Component for T where T: Any + Send + Sync + Debug {}

mod __resource_mopafy_scope {
    #![allow(clippy::all)]

    use mopa::mopafy;
    use super::Resource;
    mopafy!(Resource);
}

#[derive(Default, Debug)]
pub(crate) struct Components {
    storage: HashMap<TypeId, Box<dyn Resource>>
}

impl Components {
    pub fn insert<C: Component>(&mut self, id: NodeId, component: C) {
        let storage = self.entry::<HashMap<NodeId, C>>()
            .or_insert_with(move || {
                Box::new(HashMap::<NodeId, C>::new())
            });

        let storage = storage.as_mut();
        let storage: &mut HashMap::<NodeId, C> = unsafe { storage.downcast_mut_unchecked() };
        storage.insert(id, component);
    }

    pub fn get<C: Component>(&self, id: NodeId) -> Option<&C> {
        let storage = self.fetch::<HashMap<NodeId, C>>()?;
        storage.get(&id)
    }

    pub fn iter<C: Component>(&self) -> impl Iterator<Item=(NodeId, &C)> {
        let storage = self.fetch::<HashMap<NodeId, C>>();
        storage.into_iter()
            .flat_map(|s| {
                s.iter().map(|(k, v)| (*k, v))
            })
    }
}

impl Components {
    fn fetch<C: Resource>(&self) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        let c = self.storage.get(&type_id)?;
        let c = c.as_ref();
        unsafe {
            Some(c.downcast_ref_unchecked())
        }
    }

    fn entry<C: Resource>(&mut self) -> Entry<TypeId, Box<dyn Resource>> {
        let type_id = TypeId::of::<C>();
        self.storage.entry(type_id)
    }

    #[allow(dead_code)]
    fn fetch_mut<C: Resource>(&mut self) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        let c = self.storage.get_mut(&type_id)?;
        let c = c.as_mut();
        unsafe {
            Some(c.downcast_mut_unchecked())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Type(&'static str);

    #[derive(Debug, PartialEq)]
    struct Error(&'static str);

    trait Foo: Sync + Send + Debug {
        fn foo(&self) -> &'static str;
    }

    impl Foo for Error {
        fn foo(&self) -> &'static str { self.0 }
    }
    impl Foo for Type {
        fn foo(&self) -> &'static str { self.0 }
    }

    const IDS: [NodeId; 2] = [NodeId(1234), NodeId(666)];

    #[test]
    #[allow(clippy::borrowed_box)]
    fn dyn_component() {
        let mut components = Components::default();
        let error = Box::new(Error("Foo")) as Box<dyn Foo>;
        let t = Box::new(Type("Bar")) as Box<dyn Foo>;

        components.insert(IDS[0], error) ;
        components.insert(IDS[1], t) ;

        let res: &Box<dyn Foo> = components.get(IDS[0]).expect("dyn Foo");
        dbg!(&components);
        assert_eq!("Foo", res.foo());
    }

    #[test]
    fn component_get() {
        let mut components = Components::default();

        components.insert(IDS[0], Type("Foo"));
        components.insert(IDS[1], Type("Bar"));
        components.insert(IDS[1], Error("Bar"));

        dbg!(&components);
        assert_eq!(&Type("Foo"), components.get(IDS[0]).expect("Some type"));
        assert_eq!(&Type("Bar"), components.get(IDS[1]).expect("Some type"));
        assert_eq!(&Error("Bar"), components.get(IDS[1]).expect("Some error"));
    }

    #[test]
    fn components_iter() {
        let mut components = Components::default();

        components.insert(IDS[0], Type("Foo"));
        components.insert(IDS[1], Type("Bar"));
        components.insert(IDS[1], Error("Bar"));

        dbg!(&components);
        let mut types = components.iter::<Type>()
            .collect::<Vec<_>>();
        types.sort_by_key(|c| c.0);

        assert_eq!(&[
            (IDS[1], &Type("Bar")),
            (IDS[0], &Type("Foo"))
        ], types.as_slice());
    }
}


