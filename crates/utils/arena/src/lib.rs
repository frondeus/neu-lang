use std::borrow::Borrow;
use std::fmt;
use std::collections::HashMap;
//use anymap::any::Any;
//use std::any::{Any, TypeId};
use mopa::Any;
use std::any::TypeId;
use std::collections::hash_map::Entry;

pub trait Node {
    fn parent(&self) -> Option<NodeId>;
    fn set_parent(&mut self, id: NodeId);
    fn children(&self) -> &[NodeId];
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(crate) usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}

pub struct Ancestors<'a, Node> {
    current: Option<NodeId>,
    arena: &'a Arena<Node>,
}

impl<'a, N: Node> Iterator for Ancestors<'a, N> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.map(|id| self.arena.get(id))?;

        let ancestor = current.parent();

        let current = self.current.take();
        self.current = ancestor;
        current
    }
}

pub struct Arena<Node> {
    nodes: Vec<Node>,
    components: Components
}

impl<N> Default for Arena<N> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            components: Default::default(),
        }
    }
}

impl<Node: fmt::Debug> fmt::Debug for Arena<Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arena")?;
        for (i, n) in self.nodes.iter().enumerate() {
            write!(f, "\tN{}: ", i)?;
            writeln!(f, "{:?}", n)?;
        }
        Ok(())
    }
}

impl<N: Node> Arena<N> {
    pub fn take(&mut self) -> Self {
        let nodes = std::mem::take(&mut self.nodes);
        let components = std::mem::take(&mut self.components);
        Self {
            nodes,
            components
        }
    }

    pub fn add(&mut self, node: N) -> NodeId {
        let len = self.nodes.len();
        let id = NodeId(len);
        for child_id in node.children().iter() {
            let child = self.get_mut(child_id);
            child.set_parent(id);
        }
        self.nodes.push(node);
        id
    }

    pub fn add_component(&mut self, id: NodeId, c: impl Component) {
        self.components.insert(id, c);
    }

    pub fn component<C: Component>(&self, id: NodeId) -> Option<&C> {
        self.components.get(id)
    }

    pub fn components<C: Component>(&self) -> impl Iterator<Item = (NodeId, &C)> {
        self.components.iter()
    }

    pub fn ancestors(&self, id: NodeId) -> Ancestors<N> {
        Ancestors {
            current: Some(id),
            arena: self,
        }
    }

    pub fn get(&self, id: impl Borrow<NodeId>) -> &N {
        let id = *id.borrow();
        &self.nodes[id.0]
    }

    pub fn get_mut(&mut self, id: impl Borrow<NodeId>) -> &mut N {
        let id = *id.borrow();
        &mut self.nodes[id.0]
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &N> {
        self.nodes.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (NodeId, &N)> {
        self.nodes.iter().enumerate().map(|(id, n)| (NodeId(id), n))
    }
}

use std::fmt::Debug;
pub trait Resource: Any + Send + Sync + 'static + Debug { }
impl<T> Resource for T where T: Any + Send + Sync + Debug {}

pub trait Component : Send + Sync + 'static + Debug {
}

mod __resource_mopafy_scope {
    #![allow(clippy::all)]

    use mopa::mopafy;
    use super::Resource;
    mopafy!(Resource);
}

#[derive(Default, Debug)]
struct Components {
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
        let c = self.storage.entry(type_id);
        c
    }

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
    impl Component for Type {}

    #[derive(Debug, PartialEq)]
    struct Error(&'static str);
    impl Component for Error {}

    #[test]
    fn component_get() {
        let mut components = Components::default();
        let node_id = NodeId(1234);
        let node_id_2 = NodeId(666);

        components.insert(node_id, Type("Foo"));
        components.insert(node_id_2, Type("Bar"));
        components.insert(node_id_2, Error("Bar"));

        dbg!(&components);
        assert_eq!(&Type("Foo"), components.get(node_id).expect("Some type"));
        assert_eq!(&Type("Bar"), components.get(node_id_2).expect("Some type"));
        assert_eq!(&Error("Bar"), components.get(node_id_2).expect("Some error"));
    }

    #[test]
    fn components_iter() {

        let mut components = Components::default();
        let node_id = NodeId(1234);
        let node_id_2 = NodeId(666);

        components.insert(node_id, Type("Foo"));
        components.insert(node_id_2, Type("Bar"));
        components.insert(node_id_2, Error("Bar"));

        dbg!(&components);
        let mut types = components.iter::<Type>()
            .collect::<Vec<_>>();
        types.sort_by_key(|c| c.0);

        assert_eq!(&[
            (node_id_2, &Type("Bar")),
            (node_id, &Type("Foo"))
        ], types.as_slice());
    }
}

