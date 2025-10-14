use crate::lazy_cell::lazy_cell_init_demo;
use crate::lazy_lock::lazy_lock_init_demo;

mod lazy_cell;
mod lazy_lock;

fn main() {
    lazy_cell_init_demo();
    lazy_lock_init_demo();
}
