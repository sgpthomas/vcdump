use crate::errors::Error;
use crate::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use vcd::{Header, IdCode, ScopeItem, Var};

#[derive(Default)]
pub struct Map {
    pub paths: HashMap<Path<String>, Var>,
    pub items: HashMap<IdCode, Vec<u64>>,
}

impl Map {
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
        self.items.insert(item.code, vec![]);
        self.paths.insert(path, item);
    }

    pub fn change_value(
        &mut self,
        id: IdCode,
        value: u64,
    ) -> Result<(), Error> {
        self.items.get_mut(&id).and_then(|x| x.last_mut()).map_or(
            Err(Error::ChangeValue),
            |last| {
                *last = value;
                Ok(())
            },
        )
    }

    pub fn push_timestep(&mut self) {
        for (_, item) in self.items.iter_mut() {
            match item.last() {
                Some(last) => {
                    let el = *last;
                    item.push(el)
                }
                None => item.push(0),
            }
        }
    }
}
