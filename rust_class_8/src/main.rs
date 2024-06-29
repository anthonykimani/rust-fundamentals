mod sync;
mod thread_file;
mod vec_thread;

use sync::sync_main;
use thread_file::thread_main;
use vec_thread::vec_thread;

fn main() {
    // thread_main();
    //
    // vec_thread();

    sync_main();
}
