// an arena (or arena allocator) is a memory management strategy where we:
// allocate a large chunk of memory up front, then hand out smaller pieces from it, and free everything at once.
// instead of allocating and freeing memory object by object, you allocate once, use it, and then drop the whole arena.
use crate::platform::{get_pagesize, mem_commit, mem_reserve};

pub struct Arena {
    base: *mut u8,
    reserve_size: usize,
    commit_size: usize,
    pos: usize,
    commit_pos: usize,
}

// The OS doesn't hand out memory byte-by-byte; it hands it out in Pages (usually 4096 bytes).
// If you ask for 10 bytes, the OS gives you a full page. If you ask for 5000 bytes, it gives you two pages.
fn allign_up(size: usize, allignment: usize) -> usize {
    let bump = size + (allignment - 1); 
    let mask =  !(allignment - 1);
    
    bump & mask // This bitwise trick "rounds up" a number to the nearest multiple of a power-of-two (like 4096).
}

impl Arena {
    // in C we return null but in rust we return None. this forces the person using your library to handle the error, making the code much safer:)
    pub fn new(reserve_size: usize, commit_size: usize) -> Option<Self> {
        let page_size = get_pagesize();

        let actual_reserve = allign_up(reserve_size, page_size);
        let actual_commit = allign_up(commit_size, page_size);

        let base_ptr = mem_reserve(actual_reserve);

        if base_ptr.is_null() {
            return None
        }     
        let mem_ptr = mem_commit(base_ptr, page_size);

        if !mem_ptr {
            return None
        }

        Some(Arena { // now we have our memory :)  we can create our arena
            base: base_ptr,
            reserve_size: actual_reserve,
            commit_size: actual_commit,
            pos: 0,
            commit_pos: actual_commit 
        })
    }

    pub fn push(&mut self, size: usize) -> *mut u8 {
        // we first align the current position to 8 bytes (standard for 64-bit CPUs)
        let position_aligned = allign_up(self.pos, 8);
        let new_position = position_aligned.wrapping_add(size);

        // out of total reserved address space?
        if new_position > self.reserve_size { 
            return std::ptr::null_mut();
        }
        // need more physical RAM?
        if new_position > self.commit_pos {
            let mut new_commit_pos = allign_up(new_position, self.commit_size);
            // don't go past the total reserve!
            if new_commit_pos > self.reserve_size {
                new_commit_pos = self.reserve_size;
            }
            // calculate how much MORE we need to ask for from daddy OS
            let commit_delta = new_commit_pos - self.commit_pos;

            unsafe {
                let commit_ptr = self.base.add(self.commit_pos);
                if !mem_commit(commit_ptr, commit_delta) {
                    return std::ptr::null_mut(); // daddy OS said no :(
                }
            }
            self.commit_pos = new_commit_pos;
        }
        // update the Arena's position and return the pointer
        self.pos = new_position;
        // we use .add() for pointer arithmetic. This is unsafe!
        unsafe { self.base.add(position_aligned) }
    }

    pub fn push_zeroed(&mut self, size: usize) -> *mut u8 {
        let ptr = self.push(size);
        if !ptr.is_null(){
            // ptr: where to start, 0: what byte to write, size: how many times
            unsafe {
                std::ptr::write_bytes(ptr, 0, size);
            }
        }
        ptr
    }

    // an Arena is just a pointer moving forward. to "free" memory, we don't actually erase the data; we just move the pos pointer backward.
    // the next time we call push, the arena will simply overwrite the "freed" space.
    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn pop_to(&mut self, target_pos: usize ) {
        if target_pos > self.pos || target_pos == 0 {
            panic!("Can not assign ptr pos that is ahead of the ptr or target pos can not be 0")
        }
        self.pos = target_pos
    }



}
