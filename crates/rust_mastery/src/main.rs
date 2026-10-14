// src/main.rs
// 這個檔案是程式的入口。我們將在這裡呼叫各個章節的練習函數。

// 宣告模組 (Modules)，告訴 Rust 我們有這些獨立的檔案
mod ch06_structs;
mod ch07_enums;
mod ch08_error;
mod ch09_traits;
mod ch10_lifetimes;

fn main() {
    println!("🚀 Welcome to Rust Mastery Learning Environment!");
    println!("-----------------------------------------------");

    // println!("\n--- Testing Chapter 6: Structs ---");
    // ch06_structs::run_exercises();

    // println!("\n--- Testing Chapter 7: Enums ---");
    // ch07_enums::run_exercises();

    // println!("\n--- Testing Chapter 8: Error Handling ---");
    // ch08_error::run_exercises();

    // println!("\n--- Testing Chapter 9: Traits ---");
    // ch09_traits::run_exercises();

    println!("\n--- Testing Chapter 10: Lifetimes & Generics ---");
    ch10_lifetimes::run_exercises();
}
