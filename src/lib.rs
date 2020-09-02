use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn calc(num1: i32, num2: i32) -> i32 {
    let res = num1 * num2;
    res
}

#[wasm_bindgen]
pub fn primes() -> i32 {
    let mut n = 2;
    let mut p = Vec::new();
    p.push(2);

    while n < 10_000_000 {
      let mut prime = true;
      let mut i = 0;
      while p[i] * p[i] <= n {
        if n % p[i] == 0 {
          prime = false;
          break;
        }
        i += 1;
      }
      if prime {
        p.push(n);
      }
      n += 1;
    }
    n
}