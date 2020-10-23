use std::marker;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    fn filter<P>(self, predicator: P) -> Filter<Self, P> 
    where
        Self: Sized,
        P: Fn(&Self::Item) -> bool,
    {
        Filter::new(self, predicator)
    }
}

pub struct Vec<T> {
    vec: std::vec::Vec<T>,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec::<T>{
            vec: std::vec::Vec::<T>::new(),
        }
    }

    pub fn add(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn iter(&self) -> VecIterator<'_, T> {
        VecIterator{
            vec: &self,
            idx: 0,
        }
    }

    pub fn iter_mut(&mut self) -> VecMutIterator<'_, T> {
        VecMutIterator::new(self)
    }
}

pub struct VecIterator<'a, T> {
    vec: &'a Vec<T>,
    idx: usize,
}

impl<'a, T> Iterator for VecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.vec.vec.len() {
            None
        } else {
            let result = Some(&self.vec.vec[self.idx]);
            self.idx += 1;
            result
        }
    }
}

pub struct VecMutIterator<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: marker::PhantomData<&'a mut T>,
}

impl<'a, T> VecMutIterator<'a, T> {
    fn new(v : &'a mut Vec<T>) -> Self {
        v.vec.as_mut_ptr();
        let ptr =  v.vec.as_mut_ptr();
        let end = unsafe { ptr.add(v.vec.len()) };

        println!("ptr: {:p}", ptr);

        VecMutIterator{
            ptr,
            end,
            _marker: marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for VecMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None
        } else {
            unsafe {
                let item = &mut *(self.ptr as *mut T);
                self.ptr = self.ptr.add(1);
                return Some(item);
            }
        }
    }
}

pub struct Filter<I, P> 
where 
    I: Iterator,
    P: Fn(&I::Item) -> bool,  
{
    iter: I,
    predicate: P,
}

impl<I, P> Filter<I, P> 
where 
    I: Iterator,
    P: Fn(&I::Item) -> bool,  
{
    pub fn new(iter: I, predicate: P) -> Self {
        Filter {
            iter,
            predicate,
        }
    }
}

impl<I, P> Iterator for Filter<I, P> 
where 
    I: Iterator,
    P: Fn(&I::Item) -> bool,    
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            if (&mut self.predicate)(&v) {
                return Some(v);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_to_iterator() {
        let mut v = Vec::<i32>::new();
        v.add(1);
        v.add(2);
        
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vec_to_iterator_mutable() {
        let mut v = Vec::new();
        v.add(1);
        v.add(2);

        let mut iter_mut = v.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), None);
    }

    #[test]
    fn vec_to_iterator_mutable_zero_sized() {
        let mut v = Vec::<i32>::new();
        let mut iter_mut = v.iter_mut();
        assert_eq!(iter_mut.next(), None);
    }


    #[test]
    fn vec_to_iterator_mutable_change_value() {
        let mut v = Vec::new();
        v.add(1);
        v.add(2);

        let mut iter_mut = v.iter_mut();
        let first = iter_mut.next();
        if let Some(value) = first {
            *value = 10;
        }

        let mut it = v.iter_mut();
        assert_eq!(it.next(), Some(&mut 10));
    }

    #[test]
    fn iterator_filter() {
        let mut v = Vec::new();
        v.add(1);
        v.add(2);
        v.add(3);

        let mut it = v.iter().filter(|&v| {v % 2 == 1});
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
    }
}