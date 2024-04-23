#[derive(Debug)]
struct Stack<T> {
    top : usize,  //栈大小
    data : Vec<T>, //用动态数组实现栈
}

impl<T> Stack<T> {
    fn new() -> Self {  //创建一个空栈
        Self {
            top : 0 ,
            data : Vec::new() ,
        }
    }
    
    fn push(&mut self , val : T) {  //将val入栈
        self.data.push(val);
        self.top+=1;
    }
    
    fn pop(&mut self) -> Option<T> {  //将栈顶元素出栈
        if self.top == 0 {
            return None
        }
        self.top -= 1;
        self.data.pop()
    }
    
    fn peek(&mut self) -> Option<&T> {  //返回栈顶元素，但不会使其出栈
           if self.top == 0  {
               return None;
            }
            self.data.get(self.top-1)
    }
    
    fn is_empty(&mut self) -> bool {  //检测栈是否为空
        if self.top == 0 {
            return true;
        }
        false
    }
    
    fn size(&mut self) -> usize {  //返回栈中元素数目
        self.data.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut stack = Stack::new();
        stack.push(1);stack.push(2);stack.push(3);
        println!("top : {?} , size : {}" , stack.peek().unwrap() , stack.size());
        println!("pop : {?} , size : {}" , stack.pop().unwrap() , stack.size());
        println!("is_empty : {:?} , stack:{:?}",stack.is_empty() , stack);
    }
}
