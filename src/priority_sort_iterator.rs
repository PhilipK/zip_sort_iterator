use std::{collections::BinaryHeap, iter::FromIterator};

struct State<T, I>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    iter: I,
    prio: T,
}

impl<T, I> PartialEq for State<T, I>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.prio == other.prio
    }
}

impl<T, I> Eq for State<T, I>
where
    I: Iterator<Item = T>,
    T: Ord,
{
}

impl<T, I> PartialOrd for State<T, I>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.prio.partial_cmp(&other.prio).map(|x| x.reverse())
    }
}

impl<T, I> Ord for State<T, I>
where
    I: Iterator<Item = T>,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prio.cmp(&other.prio).reverse()
    }
}

pub struct PrioritySortIterator<'a, T, I>
where
    I: Iterator<Item = &'a T>,
    T: Ord,
    T: 'a,
{
    queue: BinaryHeap<State<&'a T, I>>,
}

impl<'a, T, I> PrioritySortIterator<'a, T, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a + Ord,
{
    pub fn new<In>(iterators: &mut [In]) -> Self
    where
        In: IntoIterator<Item = &'a T, IntoIter = I> + Copy,
    {
        let iter = iterators.iter().filter_map(|iterator| {
            let mut iter = iterator.into_iter();
            let next = iter.next();
            next.map(|prio| State { iter, prio })
        });
        Self {
            queue: BinaryHeap::from_iter(iter),
        }
    }
}

impl<'a, T, I> Iterator for PrioritySortIterator<'a, T, I>
where
    I: Iterator<Item = &'a T>,
    T: Ord,
    T: 'a,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut needs_pop = false;
        if self.queue.is_empty() {
            return None;
        }
        let res = {
            let mut cur = self.queue.peek_mut().unwrap();
            let next = cur.iter.next();
            let res = cur.prio;
            if next.is_some() {
                cur.prio = next.unwrap();
            } else {
                needs_pop = true;
            }
            res
        };

        if needs_pop {
            self.queue.pop();
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::priority_sort_iterator::PrioritySortIterator;

    #[test]
    fn test_priority_sort_iterator() {
        let a = vec![
            TimeStampNoCopy { time: 0, data: "h" },
            TimeStampNoCopy { time: 2, data: "l" },
        ];
        let b = vec![
            TimeStampNoCopy { time: 1, data: "e" },
            TimeStampNoCopy { time: 2, data: "l" },
        ];
        let c = vec![TimeStampNoCopy { time: 8, data: "o" }];

        let d = vec![
            TimeStampNoCopy { time: 9, data: " " },
            TimeStampNoCopy {
                time: 10,
                data: "world",
            },
        ];

        let mut zip = PrioritySortIterator::new(vec![&a, &b, &c, &d].as_mut_slice());
        assert_eq!(zip.next().map(|x| x.data), Some("h"));
        assert_eq!(zip.next().map(|x| x.data), Some("e"));
        assert_eq!(zip.next().map(|x| x.data), Some("l"));
        assert_eq!(zip.next().map(|x| x.data), Some("l"));
        assert_eq!(zip.next().map(|x| x.data), Some("o"));
        assert_eq!(zip.next().map(|x| x.data), Some(" "));
        assert_eq!(zip.next().map(|x| x.data), Some("world"));
        assert_eq!(zip.next(), None);
    }

    #[test]
    fn test_priority_sort_integers_iterator() {
        let a = vec![1, 3, 5];
        let b = vec![2, 4];

        let mut zip = PrioritySortIterator::new(vec![&a, &b].as_mut_slice());
        assert_eq!(zip.next(), Some(&1));
        assert_eq!(zip.next(), Some(&2));
        assert_eq!(zip.next(), Some(&3));
        assert_eq!(zip.next(), Some(&4));
        assert_eq!(zip.next(), Some(&5));
        assert_eq!(zip.next(), None);
    }

    #[test]
    fn benchmark() {
        let mut a = vec![];
        let mut b = vec![];
        let mut c = vec![];

        for i in 0..1000000 {
            a.push(i);
        }
        for i in 0..1000000 {
            b.push(i);
        }
        for i in 0..1000000 {
            c.push(i);
        }
        let mut zip = PrioritySortIterator::new(vec![&a, &b, &c].as_mut_slice());

        let before = std::time::Instant::now();
        while zip.next().is_some() {
            zip.next();
        }
        let dur = before.elapsed();
        println!("{:?}", dur);

        let before = std::time::Instant::now();
        let mut x = 0;
        for i in 0..3000000{
            x += i %3 * 3 *2;
        }
        let dur = before.elapsed();
        println!("{:?} {}", dur,x);
        
    }

    impl<T> Ord for TimeStampNoCopy<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.time.cmp(&other.time)
        }
    }
    #[derive(Debug, Clone)]
    pub struct TimeStampNoCopy<T> {
        pub time: u64,
        pub data: T,
    }

    impl<T> PartialEq for TimeStampNoCopy<T> {
        fn eq(&self, other: &Self) -> bool {
            self.time == other.time
        }
    }

    impl<T> PartialOrd for TimeStampNoCopy<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.time.cmp(&other.time))
        }
    }

    impl<T> Eq for TimeStampNoCopy<T> {}
}
