extern crate libc;
use std::process;
// /home/sreysor-ath/POSIX syscall/rustc/target/debug 
//  cd '/home/sreysor-ath/POSIX syscall/rustc/target/debug'
// strace ./rustc
const SIZE: usize = 5;
//let mut nums: [i32; SIZE] = [0, 1, 2, 3, 4];
//static mut nums: &'static [i32] = &[0, 1, 2, 3, 4];
static mut NUMS: [i32; SIZE] = [0, 1, 2, 3, 4];
//cargo build
fn main() {

  
 //let pid = 30;
    let pid = unsafe { libc::fork() };

unsafe  {
    println!("rust : after fork  pid = {}", pid ); 

    if pid < 0 {
        println!("Fork failed");
        process::exit(1);
    } else if pid == 0 {
        // Child process
        for i in 0..SIZE {
            NUMS[i] *= -(i as i32);
            println!("CHILD: {}, pid = {}" , NUMS[i],pid);
        }
    } else {
        // Parent process
       // unsafe {
            let mut status: i32 = 0;
            libc::waitpid(pid, &mut status, 0);
            for i in 0..SIZE {
                println!("PARENT: {}, pid = {}", NUMS[i], pid);
            }
       // }
    }
}
}
