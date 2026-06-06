use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        let ptr = unsafe { MALLOC.malloc(size_of::<T>() * capacity) as *mut T };
        return FastVec {
            ptr_to_data: ptr,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        unsafe { MALLOC.free(self.ptr_to_data as *mut u8); }
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
        if i >= self.len {
            panic!("FastVec: get out of bounds");
}
        unsafe {
            &*self.ptr_to_data.add(i) 

        }
        
        

    
    }

    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {
            let new_capacity = self.capacity * 2;
            let new_ptr = unsafe { 
                MALLOC.malloc(new_capacity * size_of::<T>()) as *mut T 
            };
                for i in 0..self.len() {
                    unsafe {
                        ptr::write(new_ptr.add(i), ptr::read(self.ptr_to_data.add(i)));
                    }  
            }
            self.ptr_to_data = new_ptr;
            self.capacity = new_capacity;
        }
        else {
            unsafe {
                ptr::write(self.ptr_to_data.add(self.len), t);

            }
        }
         self.len += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        unsafe {
            let value = ptr::read(self.ptr_to_data.add(i));

            for current_slot in i..self.len-1 {
                ptr::write(self.ptr_to_data.add(current_slot), ptr::read(self.ptr_to_data.add(current_slot+1)));
            }
            self.len = self.len - 1;
            return value
        }
    }

    // This appears correct but with further testing, you will notice it has a bug!
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) {
        unsafe { MALLOC.free(self.ptr_to_data as *mut u8); }
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}

// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}