pub trait Rgba<T: Sized + Copy> {
    fn rgba(&self) -> (T, T, T, T);
}
