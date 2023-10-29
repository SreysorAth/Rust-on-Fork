
extern crate libc;
use std::process;

const SIZE: usize = 5;

fn main() {
    let mut nums: [i32; SIZE] = [0, 1, 2, 3, 4];
    let pid = unsafe { libc::fork() };

    if pid < 0 {
        println!("Fork failed");
        process::exit(1);
    } else if pid == 0 {
        // Child process
        for i in 0..SIZE {
            nums[i] *= -(i as i32);
            println!("CHILD: {}", nums[i]);
        }
    } else {
        // Parent process
        unsafe {
            let mut status: i32 = 0;
            //libc::waitpid(pid, &mut status, 0);
        }
        for i in 0..SIZE {
            println!("PARENT: {}", nums[i]);
        }
    }
}