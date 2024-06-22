mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // let message = format!("Hello, {}!", word);
    // alert(&message);
    alert("foobar");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[wasm_bindgen]
pub fn sort(arr: &mut [i32]) {
    arr.sort()
}

#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn heavy_computation(x: i64) -> i64 {
    let mut sum = 0;
    for _i in 0..x {
        sum += 1;
    }
    sum
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[wasm_bindgen]
pub fn sum(arr: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in arr {
        sum += i;
    }
    sum
}

#[wasm_bindgen]
pub fn sum_of_squares(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        sum += i * i;
    }
    sum
}