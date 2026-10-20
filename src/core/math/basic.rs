use crate::core::subleq::Computer;

trait Arithmetic {
    fn add(&mut self,a:usize,b:usize)->usize;
}
impl Arithmetic for Computer {
    fn add(&mut self,a:usize,b:usize)->usize {
        let v1=self.fetch(a);
        let v2=self.fetch(b);
        let sum = v1+v2;
        self.load(a, sum);
        return a;

        
    }
    
}