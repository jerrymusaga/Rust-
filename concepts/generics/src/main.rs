struct Queue<T> {
    young: Vec<T>,
    old:Vec<T>
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
    let mut q = Queue::new();
    let mut e = Queue::new();
    q.push(5);
    e.push("hello");
    q.is_empty();
    q.split();

}
