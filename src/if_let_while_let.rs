
pub fn if_let_while_let() {

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

  
}