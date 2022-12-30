use std::{ cmp };
use crate::bubble_sort::print_bubble;
use crate::cocktail_sort::print_cocktail;
use crate::comb_sort::print_comb;
use crate::morse_code::print_word_morse;
use crate::caesar_crypt::print_caesar;
use crate::dijikstra_sort::print_dijkstra;

mod bubble_sort;
mod cocktail_sort;
mod comb_sort;
mod morse_code;
mod caesar_crypt;
mod dijikstra_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool where T: cmp::PartialOrd {
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}

fn main() {
    // print_bubble();
    // print_cocktail();
    // print_comb();
    // print_word_morse();
    // print_caesar();
    print_dijkstra();
}