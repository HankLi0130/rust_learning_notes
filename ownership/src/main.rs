mod ownership_move;
mod ownership_borrow;
mod ownership_slice;

fn main() {
    ownership_move::main();
    ownership_borrow::main();
    ownership_slice::main();
}
