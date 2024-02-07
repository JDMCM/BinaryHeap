fn main() {
    println!("Hello, world!");
}

struct Bheap<T:Copy, F:Fn(T,T)->bool> {
    pub data: Vec<T>,
    pub predicate: F,
    
}

impl<T:Copy, F: Fn(T,T)->bool> Bheap<T,F> {
    fn new(pred: F) -> Self {
        return Self {data: Vec::new(), predicate: pred}
    }

    fn top(child: usize) -> usize {
        return ((child-1)/2)
    }

    fn left(parent: usize) -> usize {
        return 2*parent+1
    }

    fn right(parent: usize) -> usize {
        return 2*parent+2
    }

    fn fixup(&mut self) {
        let mut rover: usize = self.data.len()-1;
        
        if rover != 0 {
        
         while rover != 0 && (self.predicate)(self.data[rover],self.data[Self::top(rover)]) {
             let mut next: usize =Self::top(rover);
             let mut tmp = self.data[rover];
             self.data[rover] = self.data[next];
             self.data[next] = tmp;
             rover = next;
    
            
         }
        }
    }

    fn fixdown(&mut self) {
        let mut rover:usize = 0;
        let mut tmp = self.data[rover];
        while Self::left(rover) < self.data.len() &&
         ((self.predicate)(self.data[Self::left(rover)],self.data[rover]) || 
         (self.predicate)(self.data[Self::right(rover)],self.data[rover])) {
            tmp = self.data[rover];
            if (self.predicate)(self.data[Self::left(rover)],self.data[Self::right(rover)]) {
                self.data[rover] = self.data[Self::left(rover)];
                self.data[Self::left(rover)] = tmp;
                rover = Self::left(rover);
            } else {
                self.data[rover] = self.data[Self::right(rover)];
                self.data[Self::right(rover)] = tmp;
                rover = Self::right(rover);
            }
        }
    }

    fn enqueue(&mut self, elem: T) {
        self.data.push(elem);
        Self::fixup(self);
    }

    fn dequeue(&mut self) -> T{
        let tmp:T = self.data[0].clone();
        self.data[0] = self.data.remove(self.data.len()-1);
        Self::fixdown(self);
        return tmp
    }

    fn peek(self) -> T {
        return self.data[0];
    }

    fn is_empty(self) -> bool {
        return self.data.is_empty();
    }

    fn size(self) -> usize {
        return self.data.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::Bheap;

    #[test]
    fn it_works() {
        let mut alabama: Bheap<i32,Box<dyn Fn(i32,i32)->bool> > = Bheap::new(Box::new(|x:i32,y:i32| { x > y}));
        alabama.enqueue(5);
        alabama.enqueue(6);
        alabama.enqueue(88);
        alabama.enqueue(12);
        let  t = alabama.peek();
        //let  line = alabama.size();
        assert_eq!(t,88);
        //assert_eq!(line,2);
        
    }

}