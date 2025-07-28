// shadowing in Rust allows one value to be shadowed by another, meaning the compiler will see the non-shadowed variable instead

fn main() 
{
  let x = 5; // shadowed variable
  
  let x = x + 1; // you can declare a variable again and use its previous value, effecetivly shadowing it
  
  {   // inner scope created with another pair of curly brackets
    
    let x = x + 2; // This logic will only apply inside the scope due to the curly brackets, but we shadow again, the variable that shadowed the first variable is now shadowed itself
    
    println!("The value of x in this scope: {}", x); // will output 8 due to x being equal to 8 in this scope, due to being the value that overshadowed the previous variables
    
  }
  
  println!("The value of x outside the scope: {}", x); // will output 6 due to x being equal to 6 in this scope, due to having only overshadowed the first variable
}
