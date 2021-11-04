pub trait Container<T> {
    fn get_max_count(&self) -> T;
    fn set_max_count(&mut self, value: T);
    fn get_count(&self) -> T;
    fn set_count(&mut self, value: T);
}