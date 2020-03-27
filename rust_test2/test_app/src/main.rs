use submod;

///
/// 
#[derive(Debug)]
struct Counter {
    count: i32,
    _currentItem: i32
}

impl Counter {
    fn new(start_value: i32) -> Counter {
        Counter { 
            count: start_value,
            _currentItem: -1
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self._currentItem < 0 {
            self._currentItem = self.count
        }
        self._currentItem -= 1;

        if self._currentItem >= 0 {
            Some(self._currentItem)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self._currentItem += 1;

        if self._currentItem < self.count {
            Some(self._currentItem)
        } else {
            None
        }
    }
}

fn main() {

    let c = &mut Counter::new(5);

    c.for_each( & |count| {
        println!("Fwd, {:?}", count);
    });

    c.rev().for_each(|count| {
        println!("Rev, {:?}", count);
    });
    c.next_back();
    println!("forth and back, {:?}", c.next());

    println!("+1 {:?}", submod::add_one(2));
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum = Counter::new(6).zip(Counter::new(6).skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}