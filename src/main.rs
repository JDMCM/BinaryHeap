fn main() {
    println!("Hello, world!");
}

struct Bheap<T:Copy, F:Fn(T,T)->bool> {
    pub data: Vec<T>,
    pub length: usize,
    pub predicate: F,
    
}

impl<T:Copy, F:Fn(T,T)->bool> Bheap<T,F> {
    fn new(pred: F) -> Self {
        return Self {data: Vec::new(), length: 0, predicate: pred}
    }

    fn with_capacity(pred: F, size: usize) -> Self {
        return Self {data: Vec::with_capacity(size), length: size, predicate: pred}
    }

    fn top(child: usize) -> usize {
        return (child-1)/2
    }

    fn left(parent: usize) -> usize {
        return 2*parent+1
    }

    fn right(parent: usize) -> usize {
        return 2*parent+2
    }

    fn fixup(&mut self) {
        let rover: usize = self.length;
        let mut next: usize =Self::top(rover);
        let tmp = self.data[rover];
        self.data[next] = tmp
        

    }


}
