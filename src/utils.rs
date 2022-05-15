#![allow(dead_code)]
use std::cmp::Ordering;

pub fn f32_cmp(a: &f32, b: &f32) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}
