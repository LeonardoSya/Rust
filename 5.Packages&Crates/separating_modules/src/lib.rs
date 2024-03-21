mod front;

pub use front::hosting;

pub fn eat() {
    hosting::add();
}
