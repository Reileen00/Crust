#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use core::mem::size_of;
use core::ffi::{c_int, c_void};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod libc {
    use core::ffi::{c_int, c_void};
    extern "C" {
        #[link_name="printf"]
        pub fn printf_raw(fmt: *const u8, ...) -> c_int;

        #[link_name="free"]
        fn free_raw(ptr: *mut c_void);

        #[link_name="realloc"]
        fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    }
    
    #[macro_export]
    macro_rules! printf{
        ($fmt:literal)=>{
            libc::printf_raw($fmt.as_ptr);
        };
    }

    pub unsafe fn realloc_mut<T>(ptr: *mut T, count: usize) -> *mut T {
        realloc(ptr as *mut c_void, core::mem::size_of::<T>() * count) as *mut T
    }

    pub unsafe fn free<T>(ptr: *mut T) {
        free_raw(ptr as *mut c_void);
    }

    
}

#[repr(C)]
struct Array<T> {
    items: *mut T,
    count: usize,
    capacity: usize,
}

impl<T> Array<T> {
    const fn new() -> Self {
        Self {
            items: ptr::null_mut(),
            count: 0,
            capacity: 0,
        }
    }
}

unsafe fn array_destroy<T>(xs: *mut Array<T>) {
    libc::free_mut((*xs).items);
    (*xs).items = core::ptr::null_mut();
    (*xs).count = 0;
    (*xs).capacity = 0;
}

unsafe fn array_push<T: Copy>(xs: *mut Array<T>, item: T) {
    if (*xs).count >= (*xs).capacity {
        (*xs).capacity = if (*xs).capacity == 0 { 256 } else { (*xs).capacity * 2 };
        (*xs).items = libc::realloc_mut((*xs).items, (*xs).capacity);
    }

    *((*xs).items.add((*xs).count)) = item;
    (*xs).count += 1;
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    libc::printf_raw(b"Hello world".as_ptr());
    let mut xs: Array<i32> = Array::new();

    array_push(&mut xs, 69);
    array_push(&mut xs, 639);
    array_push(&mut xs, 629);
    array_push(&mut xs, 6923);
    array_push(&mut xs, 692);

    for i in 0..xs.count {
        libc::printf(b"%d : %d\n\0".as_ptr(), i as i32, *xs.items.add(i));
    }

    libc::printf(b"hello from rust\n\0".as_ptr(), xs.count);
    0
}
