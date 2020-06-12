use std::iter::{Fuse, FusedIterator};

pub trait SkipLastIterator: Iterator
where
    Self: std::marker::Sized,
{
    fn skip_last(self) -> SkipLast<Self>;
}

impl<I> SkipLastIterator for I
where
    I: Iterator,
{
    fn skip_last(self) -> SkipLast<I> {
        SkipLast::new(self)
    }
}

pub struct SkipLast<I>
where
    I: Iterator,
{
    iter_: Fuse<I>,
    lookahead: Option<I::Item>,
    first_backward_consumed: bool,
}

impl<I> SkipLast<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> SkipLast<I> {
        let mut iter = iter.fuse();
        let lookahead = iter.next();
        SkipLast {
            iter_: iter,
            lookahead: lookahead,
            first_backward_consumed: false,
        }
    }
}

impl<I> Iterator for SkipLast<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.iter_.next();

        if next_item.is_none() {
            return None;
        }

        let res = self.lookahead.take();
        self.lookahead = next_item;
        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // The iterator is already advanced.
        self.iter_.size_hint()
    }
}

impl<I> Clone for SkipLast<I>
where
    I: Iterator + Clone,
    <I as Iterator>::Item: Clone,
{
    fn clone(&self) -> SkipLast<I> {
        SkipLast {
            iter_: self.iter_.clone(),
            lookahead: self.lookahead.clone(),
            first_backward_consumed: self.first_backward_consumed,
        }
    }
}

impl<I> ExactSizeIterator for SkipLast<I> where I: ExactSizeIterator {}

impl<I> FusedIterator for SkipLast<I> where I: FusedIterator {}

impl<I> DoubleEndedIterator for SkipLast<I>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = if self.first_backward_consumed {
            self.iter_.next_back()
        } else {
            self.iter_.next_back();
            self.first_backward_consumed = true;
            self.iter_.next_back()
        };

        if item.is_some() {
            item
        } else {
            let last_back_item = self.lookahead.take();
            last_back_item
        }
    }
}

#[cfg(test)]
mod tests;
