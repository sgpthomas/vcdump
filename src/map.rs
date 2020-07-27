use crate::errors::Error;
use crate::path::Path;
use crate::ValueType;
use std::collections::HashMap;
use std::rc::Rc;
use vcd::{Header, IdCode, ScopeItem, Var};

pub struct MapItemList<V> {
    pub values: Vec<V>,
    pub item: Rc<Var>,
}

pub struct Map<V> {
    pub paths: HashMap<Path<String>, Rc<Var>>,
    pub items: HashMap<IdCode, MapItemList<V>>,
}

impl<V> Default for Map<V> {
    fn default() -> Self {
        Self {
            paths: Default::default(),
            items: Default::default(),
        }
    }
}

impl<V: ValueType> Map<V> {
    pub fn new(header: Header) -> Self {
        let mut map = Self::default();
        for item in header.items {
            map.construct_map(Rc::new(Path::Empty), item)
        }
        map
    }

    fn construct_map(&mut self, prefix: Rc<Path<String>>, item: ScopeItem) {
        match item {
            ScopeItem::Scope(data) => {
                for child in data.children {
                    self.construct_map(
                        Rc::new(Path::Segment(
                            Rc::clone(&prefix),
                            data.identifier.clone(),
                        )),
                        child,
                    );
                }
            }
            ScopeItem::Var(data) => {
                let name = data.reference.clone();
                self.insert(Path::Segment(prefix, name), data);
            }
        }
    }

    fn insert(&mut self, path: Path<String>, item: Var) {
        if self.paths.contains_key(&path) {
            panic!("oh no!")
        }
        let item = Rc::new(item);
        self.items.insert(
            item.code,
            MapItemList {
                values: vec![],
                item: Rc::clone(&item),
            },
        );
        self.paths.insert(path, item);
    }

    pub fn change_value(&mut self, id: IdCode, value: V) -> Result<(), Error> {
        self.items
            .get_mut(&id)
            .and_then(|x| x.values.last_mut())
            .map_or(Err(Error::ChangeValue), |last| {
                *last = value;
                Ok(())
            })
    }

    pub fn push_timestep(&mut self) {
        for (_, item_list) in self.items.iter_mut() {
            match item_list.values.last() {
                Some(last) => {
                    let el = last.clone();
                    item_list.values.push(el);
                }
                None => {
                    item_list.values.push(V::empty(&item_list.item));
                }
            }
        }
    }
}
