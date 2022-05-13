

pub fn if_while_loops() {
  
  // Flujos de control
  if true {
      println!("True");
  } else if false {
      println!("False");
  } else {
      println!("Nada");
  }

  let result = if 1 > 2 { 10 } else { 5 };
  println!("result {}", result);


  // Loops
  let mut number = 0;
  let rest = loop {
      println!("Loop");
      
      number += 1;

      if number == 10 {
          break number
      }
  };

  println!("loop return {}", rest);


  // While
  while number > 0 {
      println!("While");
      number -= 1;
  }


  // For
  for item in [1, 2, 3].iter() {
      println!("Array {}", item);
  }

  for item in 0..5 {
      println!("Array {}", item);
  }


}