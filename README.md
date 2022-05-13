## Controls Flow

### if - loop - while - for
```
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

```

### if let - while let

```
   let edad: Option<i32> = Some(23);

  // match
  match edad {
    Some(value) => println!("Edad: {} ", value),
    _ => ()
  }

  // if let
  if let Some(value) = edad {
      println!("Edad: {}", value);
  }

  // while let 

  let mut mss = Some(10);
  
  loop {
      match mss {
        Some(value) => {
          if value > 0 {
            println!("Mensaje sin leer");
            mss = Some(value - 1);
          } else{
            println!("No hay mensajes");
            mss = None;
          }
        },
        _ => { break; }
      };
  }

  while let Some(value) = mss {
    if value > 0 {
      println!("Mensaje sin leer");
      mss = Some(value - 1);
    } else{
      println!("No hay mensajes");
      mss = None;
    }
  }

```
