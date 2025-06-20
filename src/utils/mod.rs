// utils
// use super::lodging::{Accommodation};
use crate::lodging::Accommodation;

pub fn mix_and_match<T: Accommodation,U: Accommodation>(first: &mut T, second: &mut U,guest: &str) { // here both first and second are of both are different types Now
    first.book(guest, 1);
    second.book(guest, 1);
}