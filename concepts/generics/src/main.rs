struct Queue<T> {
    young: Vec<T>,
    old:Vec<T>
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len(){
        if slice[i] < *least {
            least = & slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema {
        greatest,
        least
    }
}

impl<T> Queue<T>{
    fn new() -> Queue<T> {
        Queue {
            young: Vec::new(),
            old: Vec::new()
        }
    }

    fn is_empty(&mut self) -> bool {
        self.old.is_empty() && self.old.is_empty()
    }

    fn push(&mut self, c: T) {
        self.young.push(c);
    }

    fn split(self) -> (Vec<T>, Vec<T>){
        (self.old, self.young)
    }
}

fn main() {

    let z = [-5, 5, 6,4,3];
    let ex = find_extrema(&z);
    assert_eq!(*ex.greatest, 6);
    assert_eq!(*ex.least, -5);

    let mut q = Queue::new();
    let mut e = Queue::new();
    q.push(5);
    e.push("hello");
    q.is_empty();
    q.split();

}
