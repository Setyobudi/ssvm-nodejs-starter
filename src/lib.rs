use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(params: &str) -> String {
  let ps: (f32, f32,) = serde_json::from_str(&params).unwrap();
  let bmi=(ps.0*703)/(ps.1*ps.1);
  if(bmi>25)
  return serde_json::to_string(&"Overweight").unwrap();
  else if(bmi<25 && bmi>18.5)
  return serde_json::to_string(&"Optimal").unwrap();
  else
    return serde_json::to_string(&"Underweight").unwrap();
  } else {
    return String::from("Error");
  }
}
