#[derive(Debug)]
struct queue<T> {
    cap : usize;
    data : vec<T>
}

impl<T> queue<T> {
    fn new(size : usize) -> Self {  //创建一个空队列
        Self {
            cap : size ,
            data : Vec::with_capacity(size) ,
        }
    }
    
    fn enqueue(&mut self , val : T) -> Result<() , String>{  //将val入队
        if self.size() == self.cap {
            return Err("No space available".to_string());
        }

        self.data.insert(0,val);
        Ok(())
    }
    
    fn dequeue(&mut self) -> Option<T> {  //将队头元素出队
        if self.size() <=0 {
            None
        }
        else {
            self.data.pop()
        }
    }

    fn is_empty(&mut self) -> bool {  //检测队列是否为空
        self.size() == 0
    }
    
    fn size(&mut self) -> usize {  //返回队列元素数目
        self.data.len()
    }
}

