use wasm_rust::heavy_computation;

fn main() {
    let result = heavy_computation(2550000000);
    println!("{}", result);
}
