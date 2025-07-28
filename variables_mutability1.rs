// Rust notes 2 //

fn main() // main function
{
  let x = 5;   // declare a variable like this 
  println!("the value of x is: {}", x);  // output a variable like this, similar to C syntax, in newer rust compilers we can also do (the value of x is: {x}") instead. 
  
  // in rust we can't change variables unless they are declared as mutables
  // for example:
  let mut y = 6;
  println!("the value of y is {}", y);
  y = 7;
  println!("the value of y is {}", y);

  // if 'y' was not a "mut" variable, we would not be able to change it 
  // 'x' is not a "mut" variable so it cannot be changed
}
