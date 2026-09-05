use std::cell::{Cell, RefCell};
use take_until::TakeUntilExt;

#[test]
fn inclusive_stops_and_tail() {
    for stop in [0, 2, 4, 9] {
        let calls = Cell::new(0);
        let mut source = 0..5;
        let result: Vec<_> = source
            .by_ref()
            .take_until(|x| {
                calls.set(calls.get() + 1);
                *x == stop
            })
            .collect();
        let count = (stop + 1).min(5);
        assert_eq!(result, (0..count).collect::<Vec<_>>());
        assert_eq!(calls.get(), count);
        assert_eq!(source.next(), (count < 5).then_some(count));
    }
}

#[test]
fn empty_and_size_hints() {
    let mut empty = core::iter::empty::<u8>().take_until(|_| panic!("empty predicate"));
    assert_eq!(empty.size_hint(), (0, Some(0)));
    assert_eq!(empty.next(), None);
    let values = [0, 1, 2];
    let mut iter = values.iter().take_until(|x| **x == 1);
    assert_eq!(iter.size_hint(), (1, Some(3)));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.size_hint(), (1, Some(2)));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(
        (0..3).filter(|_| true).take_until(|_| false).size_hint(),
        (0, Some(3))
    );
    assert_eq!((0..).take_until(|_| true).size_hint(), (1, None));
}

#[test]
fn fold_matches_next_order_and_tail() {
    for stop in [0, 2, 4, 9] {
        for prefix in 0..=5 {
            let run = |fold: bool| {
                let events = RefCell::new(Vec::new());
                let mut source = 0..5;
                let mut iter = source.by_ref().take_until(|x| {
                    events.borrow_mut().push(('p', *x));
                    *x == stop
                });
                for _ in 0..prefix {
                    iter.next();
                }
                let consume = |acc, x| {
                    events.borrow_mut().push(('c', x));
                    acc + x
                };
                let result = if fold {
                    iter.fold(10, consume)
                } else {
                    let mut acc = 10;
                    for x in iter {
                        acc = consume(acc, x);
                    }
                    acc
                };
                (result, events.into_inner(), source.next())
            };
            assert_eq!(run(true), run(false), "stop={stop}, prefix={prefix}");
        }
    }
}

fn non_fused() -> impl Iterator<Item = u8> {
    let mut values = [Some(1), None, Some(2), Some(3), None].into_iter();
    core::iter::from_fn(move || values.next().flatten())
}

#[test]
fn non_fused_resumes_until_predicate_matches() {
    let mut iter = non_fused().take_until(|x| *x == 2);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let mut iter = non_fused().take_until(|_| false);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.fold(10, |acc, x| acc + x), 15);

    let mut source = non_fused();
    assert_eq!(
        source
            .by_ref()
            .take_until(|_| false)
            .fold(10, |acc, x| acc + x),
        11
    );
    assert_eq!(source.next(), Some(2));
}

#[test]
#[allow(clippy::unnecessary_fold)] // Exercise fold explicitly.
fn fused_bound_and_borrowed_fold() {
    fn assert_fused(_: impl core::iter::FusedIterator) {}
    assert_fused((0..5).take_until(|x| *x == 2));
    let calls = Cell::new(0);
    let mut iter = (0..5).take_until(|x| {
        calls.set(calls.get() + 1);
        *x == 2
    });
    assert_eq!(iter.by_ref().fold(0, |acc, x| acc + x), 3);
    assert_eq!(iter.next(), None);
    assert_eq!(calls.get(), 3);
}
