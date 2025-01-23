pub trait Mode{
    fn run(&mut self);

    fn update(&self);
}