
fn main() 
{
  let x = 5;
  
  let x = x + 1; // you can declare a variable again and use its previous value to declare a new value for it even if its not a "mut" variable
  
  {   // inner scope created with another pair of curly brackets
    
    let x = x + 2; // This logic will only apply inside the scope due to the curly brackets
    
    println!("The value of x in this scope: {}", x); // will output 8 due to x being equal to 8 in this scope
    
  }
  
  println!("The value of x outside the scope: {}", x); // will output 6 due to x being equal to 6 in this scope
}
