use wasm_bindgen::prelude::*;

#[wasm_bindgen]

pub fn left_pad(string:&str, length:usize, chara:char) -> String{
  let mut string = string.to_owned();
  while string.len() < length{
    string.insert(0, chara);
  }
  return string
}