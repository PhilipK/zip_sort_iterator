pub struct ZipSortIterator<T, I>
where
    I: Iterator<Item = T>,
{
    a: I,
    b: I,

    a_next: Option<T>,
    b_next: Option<T>,
    initialized: bool,
}

impl<T, I> ZipSortIterator<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn new<In>(a: In, b: In) -> Self
    where
        In: IntoIterator<Item = T, IntoIter = I>,
    {
        ZipSortIterator {
            a: a.into_iter(),
            b: b.into_iter(),
            a_next: None,
            b_next: None,
            initialized: false,
        }
    }
}

impl<T, I> Iterator for ZipSortIterator<T, I>
where
    I: Iterator<Item = T>,
    T: PartialOrd,
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initialized {
            self.a_next = self.a.next();
            self.b_next = self.b.next();
            self.initialized = true;
        }

        let mut assign_a = false;
        let mut assign_b = false;
        let res = {
            match (self.a_next, self.b_next) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        assign_a = true;
                        // self.a_next = self.a.next();
                        Some(a)
                    } else {
                        assign_b = true;
                        // self.b_next = self.b.next();
                        Some(b)
                    }
                }
                (Some(a), None) => {
                    assign_a = true;
                    // self.a_next = self.a.next();
                    Some(a)
                }
                (None, Some(b)) => {
                    assign_b = true;
                    // self.b_next = self.b.next();
                    Some(b)
                }
                (None, None) => None,
            }
        };
        if assign_a {
            self.a_next = self.a.next();
        }
        if assign_b {
            self.b_next = self.b.next();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::ZipSortIterator;

    #[test]
    fn it_works() {
        let a = vec![1, 3, 5];
        let b = vec![2, 4];

        let mut zip = ZipSortIterator::new(a, b);
        assert_eq!(zip.next(), Some(1));
        assert_eq!(zip.next(), Some(2));
        assert_eq!(zip.next(), Some(3));
        assert_eq!(zip.next(), Some(4));
        assert_eq!(zip.next(), Some(5));
        assert_eq!(zip.next(), None);
    }

    #[test]
    fn it_works_doupes() {
        let a = vec![1, 3, 4, 5];
        let b = vec![2, 4, 4, 4, 4];

        let mut zip = ZipSortIterator::new(a, b);
        assert_eq!(zip.next(), Some(1));
        assert_eq!(zip.next(), Some(2));
        assert_eq!(zip.next(), Some(3));
        assert_eq!(zip.next(), Some(4));
        assert_eq!(zip.next(), Some(4));
        assert_eq!(zip.next(), Some(4));
        assert_eq!(zip.next(), Some(4));
        assert_eq!(zip.next(), Some(4));
        assert_eq!(zip.next(), Some(5));
        assert_eq!(zip.next(), None);
    }

    #[test]
    fn it_works_times() {
        let a = vec![
            TimeStamp { time: 0, data: "h" },
            TimeStamp { time: 2, data: "l" },
        ];
        let b = vec![
            TimeStamp { time: 1, data: "e" },
            TimeStamp { time: 2, data: "l" },
        ];

        let mut zip = ZipSortIterator::new(a, b);
        assert_eq!(zip.next().map(|x| x.data), Some("h"));
        assert_eq!(zip.next().map(|x| x.data), Some("e"));
        assert_eq!(zip.next().map(|x| x.data), Some("l"));
        assert_eq!(zip.next().map(|x| x.data), Some("l"));
        assert_eq!(zip.next(), None);
    }

    #[derive(Debug, Copy, Clone)]
    pub struct TimeStamp<T> {
        pub time: u64,
        pub data: T,
    }

    impl<T> PartialEq for TimeStamp<T> {
        fn eq(&self, other: &Self) -> bool {
            self.time == other.time
        }
    }

    impl<T> PartialOrd for TimeStamp<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.time.cmp(&other.time))
        }
    }
}
