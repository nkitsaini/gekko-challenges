use zzz::ProgressBar;
use zzz::ProgressBarIterExt as _;

pub struct FixedMultiProductIterator<I>
where
    I: Sized + Clone,
{
    values: Vec<I>,
    idxs: Vec<usize>,
    count: usize,
}

impl<I> FixedMultiProductIterator<I>
where
    I: Sized + Clone,
{
    #[inline]
    fn build(&self) -> Vec<I> {
        let mut rv = Vec::with_capacity(self.idxs.len());
        for idx in self.idxs.iter() {
            rv.push(unsafe { self.values.get_unchecked(*idx) }.clone());
        }
        rv
    }

    fn permuts(&self) -> usize {
        return self.values.len();
    }

    fn letters(&self) -> usize {
        return self.idxs.len();
    }

    pub fn forward(&mut self, mut count: usize) {
        assert!(count <= self.count);
        self.count -= count;
        let max = self.letters() - 1;
        let mut carry = 0;
        let permuts = self.permuts();

        for i in (0..self.letters()).rev() {
            let part = count % self.values.len();
            self.idxs[i] += part + carry;
            carry = 0;
            if self.idxs[i] > max {
                self.idxs[i] %= permuts;
                carry += 1;
            }
            count = count / permuts;
            if (count == 0) {
                break;
            }
        }
    }
}

pub trait FixedMultiProductIter: Iterator {
    fn fixed_product(self, width: usize) -> FixedMultiProductIterator<Self::Item>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let values: Vec<Self::Item> = self.collect();
        let length = values.len();
        FixedMultiProductIterator {
            values,
            idxs: vec![0; width],
            count: length.pow(width as u32),
        }
    }
}

impl<T: ?Sized> FixedMultiProductIter for T where T: Iterator {}

impl<I> Iterator for FixedMultiProductIterator<I>
where
    I: Clone,
{
    type Item = Vec<I>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.count, Some(self.count));
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        let rv = self.build();
        let max = self.values.len() - 1;
        for idx in (0..self.idxs.len()).rev() {
            if self.idxs[idx] != max {
                self.idxs[idx] += 1;
                self.count -= 1;
                return Some(rv);
            } else {
                self.idxs[idx] = 0;
            }
        }
        self.count -= 1;
        return Some(rv);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut iter = (0..3).fixed_product(2);
        assert_eq!(iter.size_hint().0, 9);
        let val = iter.next();
        assert_eq!(val, Some(vec![0, 0]));
        assert_eq!(iter.size_hint().0, 8);
        let val = iter.next();
        assert_eq!(val, Some(vec![0, 1]));
        assert_eq!(iter.size_hint().0, 7);
        let val = iter.next();
        assert_eq!(val, Some(vec![0, 2]));
        assert_eq!(iter.size_hint().0, 6);
        let val = iter.next();
        assert_eq!(val, Some(vec![1, 0]));
        assert_eq!(iter.size_hint().0, 5);
        let val = iter.next();
        assert_eq!(val, Some(vec![1, 1]));
        assert_eq!(iter.size_hint().0, 4);
        let val = iter.next();
        assert_eq!(val, Some(vec![1, 2]));
        assert_eq!(iter.size_hint().0, 3);
        let val = iter.next();
        assert_eq!(val, Some(vec![2, 0]));
        assert_eq!(iter.size_hint().0, 2);
        let val = iter.next();
        assert_eq!(val, Some(vec![2, 1]));
        assert_eq!(iter.size_hint().0, 1);
        let val = iter.next();
        assert_eq!(val, Some(vec![2, 2]));
        assert_eq!(iter.size_hint().0, 0);
        let val = iter.next();
        assert_eq!(val, None);
        assert_eq!(val, None);
        assert_eq!(val, None);
    }

    #[test]
    fn test_forward() {
        let mut iter = (0..3).fixed_product(2);
        assert_eq!(iter.size_hint().0, 9);
        let val = iter.next();
        assert_eq!(val, Some(vec![0, 0]));
        assert_eq!(iter.size_hint().0, 8);
        iter.forward(3);
        assert_eq!(iter.size_hint().0, 5);
        let val = iter.next();
        assert_eq!(val, Some(vec![1, 1]));
        assert_eq!(iter.size_hint().0, 4);
        let val = iter.next();
        assert_eq!(val, Some(vec![1, 2]));
        assert_eq!(iter.size_hint().0, 3);
        let val = iter.next();
        assert_eq!(val, Some(vec![2, 0]));
        assert_eq!(iter.size_hint().0, 2);
        let val = iter.next();
        assert_eq!(val, Some(vec![2, 1]));
        assert_eq!(iter.size_hint().0, 1);
        let val = iter.next();
        assert_eq!(val, Some(vec![2, 2]));
        assert_eq!(iter.size_hint().0, 0);
        let val = iter.next();
        assert_eq!(val, None);
        assert_eq!(val, None);
        assert_eq!(val, None);
    }
}
