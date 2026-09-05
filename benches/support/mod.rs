// Experimental adapter retained for reproducible benchmarks and behavioral tests.
// Its next() matches the library; only fold() adds underlying try_fold delegation.
pub struct Candidate<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> Candidate<I, P> {
    pub fn new(iter: I, predicate: P) -> Self {
        Self {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<I, P> Iterator for Candidate<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flag {
            return None;
        }
        let item = self.iter.next()?;
        self.flag = (self.predicate)(&item);
        Some(item)
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        use core::ops::ControlFlow;

        if self.flag {
            return init;
        }
        let result = self.iter.try_fold(init, |acc, item| {
            let stop = (self.predicate)(&item);
            let acc = f(acc, item);
            if stop {
                ControlFlow::Break(acc)
            } else {
                ControlFlow::Continue(acc)
            }
        });
        match result {
            ControlFlow::Break(acc) | ControlFlow::Continue(acc) => acc,
        }
    }
}
