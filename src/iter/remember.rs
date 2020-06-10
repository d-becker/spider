use std::collections::VecDeque;
use std::iter::{Fuse, FusedIterator};

// TODO: Test it.
// TODO: Specialised implementation for DoubleEndedIterator.
// TODO: Implement iterator traits.

pub trait RememberIterator: Iterator
where
    Self: std::marker::Sized,
    <Self as Iterator>::Item: Clone,
{
    fn remember(self, from_front: usize, from_back: usize) -> Remember<Self>;
}

pub struct Remember<I>
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
{
    iter: Fuse<I>,
    from_front: usize,
    from_back: usize,

    front_elements: Vec<<I as Iterator>::Item>,
    back_elements: VecDeque<<I as Iterator>::Item>,
}

impl<I> Remember<I>
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
{
    pub fn new(iter: I, from_front: usize, from_back: usize) -> Remember<I> {
        Remember {
            iter: iter.fuse(),
            from_front: from_front,
            from_back: from_back,
            front_elements: Vec::with_capacity(from_front),
            back_elements: VecDeque::with_capacity(from_back),
        }
    }

    pub fn get_remembered(self) -> (Box<[<I as Iterator>::Item]>, Box<[<I as Iterator>::Item]>) {
        let front = self.front_elements.into_boxed_slice();
        let back = self
            .back_elements
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        (front, back)
    }
}

impl<I> Iterator for Remember<I>
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;

        if self.front_elements.len() < self.from_front {
            self.front_elements.push(item.clone());
        }

        self.back_elements.push_back(item.clone());
        if self.back_elements.len() > self.from_back {
            self.back_elements.pop_front();
        }

        Some(item)
    }
}

#[cfg(test)]
mod tests;
