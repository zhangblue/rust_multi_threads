mod once_cell;
mod once_lock;

use crate::once_cell::{once_cell_example_1, once_cell_example_2};
use crate::once_lock::{once_lock_example_1, once_lock_example_2};

fn main() {
    once_cell_example_1();
    once_cell_example_2();
    once_lock_example_1();
    once_lock_example_2();
}
