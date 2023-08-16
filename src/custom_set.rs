use std::collections::HashSet;
use std::hash::Hash;
#[derive(Debug, PartialEq)]
pub struct CustomSet<T: PartialEq + Eq + Hash> {
    set: HashSet<T>,
}
impl<T> CustomSet<T>
    where
        T: PartialEq + Eq + Hash + Clone,
{
    pub fn new(input: &[T]) -> Self {
        Self {
            set: input.into_iter().cloned().collect(),
        }
    }
    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }
    pub fn add(&mut self, element: T) {
        self.set.insert(element);
    }
    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.is_subset(&other.set)
    }
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.set.is_disjoint(&other.set)
    }
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            set: self.set.intersection(&other.set).cloned().collect(),
        }
    }
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            set: self.set.difference(&other.set).cloned().collect(),
        }
    }
    pub fn union(&self, other: &Self) -> Self {
        Self {
            set: self.set.union(&other.set).cloned().collect(),
        }
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct CustomSet<T> {
//     values: Vec<T>,
//     // We fake using T here, so the compiler does not complain that
//     // "parameter `T` is never used". Delete when no longer needed.
//     phantom: std::marker::PhantomData<T>,
// }
//
// impl<T: Clone + PartialEq + Ord> CustomSet<T> {
//     pub fn new(_input: &[T]) -> Self {
//         let mut data = _input.to_vec();
//         data.sort();
//
//         CustomSet {
//             values: data,
//             phantom: Default::default(),
//         }
//     }
//
//     pub fn contains(&self, _element: &T) -> bool {
//         self.values.contains(_element)
//     }
//
//     pub fn add(&mut self, _element: T) {
//         if !self.values.contains(&_element) {
//             self.values.push(_element);
//         }
//
//         self.values.sort()
//     }
//
//     pub fn is_subset(&self, _other: &Self) -> bool {
//         match (self.values.len(), _other.values.len()) {
//             (0, 0) | (0, _) => true,
//             (_, _) => {
//                 _other.values.windows(self.values.len()).any(|x| x == self.values)
//             }
//         }
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.values.is_empty()
//     }
//
//     pub fn is_disjoint(&self, _other: &Self) -> bool {
//         match (self.values.len(), _other.values.len()) {
//             (0, 0) | (_, 0) | (0, _) => true,
//             (_, _) => {
//                 let first = self.values.first().unwrap();
//                 let last = self.values.last().unwrap();
//
//                 !_other.values.iter().any(|x| x == first || x == last)
//             }
//         }
//     }
//
//     #[must_use]
//     pub fn intersection(&self, _other: &Self) -> Self {
//         let intersection: Vec<T> = self.values
//             .iter()
//             .filter(|&x| _other.contains(x))
//             .cloned()
//             .collect();
//
//         CustomSet::new(&intersection)
//     }
//
//     #[must_use]
//     pub fn difference(&self, _other: &Self) -> Self {
//         let difference: Vec<T> = self.values
//             .iter()
//             .filter(|&x| !_other.contains(x))
//             .cloned()
//             .collect();
//
//         CustomSet::new(&difference)
//     }
//
//     #[must_use]
//     pub fn union(&self, _other: &Self) -> Self {
//         let mut union: Vec<T> = self.values.to_vec();
//         for value in &_other.values {
//             if !self.values.contains(&value) {
//                 union.push(value.clone());
//             }
//         }
//
//         CustomSet::new(&union)
//     }
// }
