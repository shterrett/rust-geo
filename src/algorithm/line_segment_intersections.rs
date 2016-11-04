use num::Float;
use std::collections::{ BinaryHeap, HashSet };
use std::collections::binary_heap::Iter;
use std::mem;
use std::hash::{ Hash, Hasher };
use std::cmp::Ordering;
use types::{ LineSegment, Point };

impl<T> Ord for Point<T>
    where T: Float
{
    // Order line segments by upper-left-ness
    fn cmp(&self, other: &Point<T>) -> Ordering {
        match self.y().partial_cmp(&other.y()).unwrap() {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                other.x().partial_cmp(&self.x()).unwrap()
            }
        }
    }
}

impl<T> PartialOrd for Point<T>
    where T: Float
{
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        self.y().partial_cmp(&other.y())
    }
}

impl<T> Eq for Point<T> where T: Float {}

impl<T> Hash for Point<T> where T: Float {
    fn hash<H: Hasher>(&self, state: &mut H){
        let x = self.x().integer_decode();
        let y = self.y().integer_decode();
        x.hash(state);
        y.hash(state);
    }
}

#[derive(Debug)]
struct EventQueue<'a, T>
    where T: Float, T: 'a {
        set: HashSet<&'a Point<T>>,
        queue: BinaryHeap<&'a Point<T>>
}

impl<'a, T> EventQueue<'a, T>
    where T: Float{
    pub fn new() -> Self {
        EventQueue {
            set: HashSet::new(),
            queue: BinaryHeap::new()
        }
    }

    pub fn push(&mut self, event: &'a Point<T>) {
        if !self.set.contains(event) {
            self.set.insert(event);
            self.queue.push(event);
        }
    }

    pub fn pop(&mut self) -> Option<&'a Point<T>> {
        if let Some(point) = self.queue.pop() {
            self.set.remove(&point);
            Some(point)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<&'a Point<T>> {
        self.queue.iter()
    }
}

#[cfg(test)]
mod test {
    use types::{ LineSegment, Point };
    use std::cmp::Ordering;
    use self::{ EventQueue };

    #[test]
    fn order_by_y_coordinate() {
        let p1 = Point::new(1., 5.);
        let p2 = Point::new(6., 0.);


        assert_eq!(point1.cmp(&point2), Ordering::Greater)
    }

    #[test]
    fn order_by_x_coordinate_if_y_equal() {
        let p1 = Point::new(1., 5.);
        let p2 = Point::new(3., 5.);


        assert_eq!(point1.cmp(&point2), Ordering::Greater)
    }

    #[test]
    fn event_queue_does_not_duplicate() {
        let p1 = Point::new(1., 5.);
        let p2 = Point::new(4., 3.);
        let p3 = Point::new(3., 5.);

        let mut queue = EventQueue::new();

        queue.push(p1);
        queue.push(p2);
        queue.push(p3);
        queue.push(p2);


    }
}
