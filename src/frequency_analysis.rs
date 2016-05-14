use std::collections::hash_map::HashMap;
use num_rational::Ratio;
use std::convert::From;
use std::hash::Hash;
use std::collections::hash_map;
use std::cmp;
use std::borrow::Borrow;
use std::iter::Iterator;


#[derive(Clone)]
pub struct Histogram<T> {
    size:  usize,
    inner: HashMap<T, usize>,
}

impl<T> Histogram<T> where T: Eq + Hash {
    pub fn new() -> Histogram<T> {
        Histogram {
            size: 0,
            inner: HashMap::new(),
        }
    }

    fn pack(size: usize, map: HashMap<T, usize>) -> Histogram<T> {
        Histogram {
            size: size,
            inner: map,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn transpose(&self, reference: &Histogram<T>) -> Histogram<T> 
        where T: Copy + Clone + Ord {

        let mut map = HashMap::new();
        let size    = cmp::min(self.len(), reference.len());

        let mut keys = Vec::new();
        for key in reference.inner.keys() {
            keys.push(key.clone());
        }
        keys.sort();

        let mut values = Vec::new();
        for value in self.inner.values() {
            values.push(value.clone());
        }
        values.sort();


        for i in 0..size {
            map.insert(keys[i], values[i]);
        }

        Histogram::pack(size, map)
    }

    pub fn get(&self, key: &T) -> Option<Ratio<usize>> {
        self.inner.get(key)
                  .and_then(|value| { Some(Ratio::new_raw(*value, self.size)) })
    }

    pub fn get_count(&self, key: &T) -> Option<usize> {
        self.inner.get(key).and_then(|value| { Some(*value) })
    }

    pub fn contains_key(&self, key: &T) -> bool {
        self.inner.contains_key(key)
    }

}
/*
pub struct IterRaw<T> {
    inner: hash_map::IntoIter<T, usize>,
}

impl<T> Iterator for IterRaw<T> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
*/
impl<'a, T> From<&'a [T]> for Histogram<T> where T: Eq + Hash + Copy {
    fn from(corpus: &'a [T]) -> Histogram<T> {
        let mut table = HashMap::new();
    
        for item in corpus {
            if table.contains_key(item) {
                *table.get_mut(item).unwrap() += 1;
            } else {
                table.insert(*item, 1);
            }
        }

        Histogram::pack(corpus.len(), table)
    }
}

impl<T> From<Vec<(T, usize)>> for Histogram<T> where T: Eq + Hash {
    fn from(corpus: Vec<(T, usize)>) -> Histogram<T> {
        let mut table        = HashMap::new();
        let mut total: usize = 0; 

        for (key, value) in corpus {
            table.insert(key, value);
            total += value;
        }

        Histogram::pack(total, table)
    }
}

impl<'a, T> From<&'a [(T, usize)]> for Histogram<T> where T: Eq + Hash + Copy {
    fn from(corpus: &'a [(T, usize)]) -> Histogram<T> {
        let mut table = HashMap::new();
        let mut total: usize = 0;

        for ref kv in corpus {
            table.insert(kv.0, kv.1);
            total += kv.1;
        }
        
        Histogram::pack(total, table)
    }
}

pub struct IntoIter<T> {
    total: usize,
    inner: hash_map::IntoIter<T, usize>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = (T, Ratio<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|(key, value)| { Some((key, Ratio::new_raw(value, self.total)))})
    }
}

pub struct IterRef<'a, T: 'a> {
    inner: hash_map::Iter<'a, T, usize>,
}

impl<'a, T> Iterator for IterRef<'a, T> {
    type Item = (&'a T, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct IterMut<'a, T: 'a> {
    inner: hash_map::IterMut<'a, T, usize>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (&'a T, &'a mut usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
} 

impl<T> IntoIterator for Histogram<T> where T: Eq + Hash {
    type Item = (T, Ratio<usize>);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            total: self.size,
            inner: self.inner.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Histogram<T> where T: Eq + Hash {
    type Item = (&'a T, &'a usize);
    type IntoIter = IterRef<'a, T>;

    fn into_iter(self) -> IterRef<'a, T> {
        IterRef {
            inner: self.inner.iter()
        }
    }
}

impl<'a, T> IntoIterator for &'a mut Histogram<T> where T: Eq + Hash {
    type Item = (&'a T, &'a mut usize);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        IterMut {
            inner: self.inner.iter_mut()
        }
    }
}
