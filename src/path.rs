use std::rc::Rc;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Path<T> {
    Empty,
    Segment(Rc<Path<T>>, T),
}

impl<T> Path<T> {
    pub fn to_vec(&self) -> Vec<&T> {
        match self {
            Path::Empty => vec![],
            Path::Segment(rest, item) => {
                let mut v = rest.to_vec();
                v.push(item);
                v
            }
        }
    }
}
