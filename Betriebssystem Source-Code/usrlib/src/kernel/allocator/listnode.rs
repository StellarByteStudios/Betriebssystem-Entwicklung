/**
Description: Metadata of a free memory block in the list allocator
*/
#[derive(Debug)]
pub struct ListNode {
    // size of the memory block
    pub size: usize,

    // &'static mut type semantically describes an owned object behind
    // a pointer. Basically, it’s a Box without a destructor that frees
    // the object at the end of the scope.
    pub next: Option<&'static mut ListNode>,
}

impl ListNode {
    // Create new ListMode on Stack
    // (must be 'const')
    pub const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    // return start address of memory block
    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    // return end address of memory block
    pub fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}
