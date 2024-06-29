#[allow(unused_variables)]
//Pracise Excercise 1: Variable

// Fix the error below with least amount of modification to the code
fn main() {
  //Excercise 1
    let x: i32=5; // Uninitialized but used, ERROR ! sol : intinalise by 5
    assert_eq!(x, 5);
    println!("Ex 1: Success!");
  //Excercise 2

  // Fill the blanks in the code to make it compile
      let mut y = 1;//sol : added mut for mutation
      y += 2; 

      assert_eq!(y, 3);
      println!("Ex 2: Success!");

  //Excercise 3
  let x: i32 = 10;
   let y: i32 = 5; //sol: improved the scope
  {

      println!("Ex-3: The value of x is {} and value of y is {}", x, y);
  }
  println!("Ex-3: The value of x is {} and value of y is {}", x, y); 

  //Excercise -4
  define_x();//sol: It was used in main but declared in this function, so fixed the error : function scope
  //Excercise -5
  let x: i32 = 5;
  {
      let x = 12;
      assert_eq!(x, 12);//sol : Shadowing in rust
  }

  assert_eq!(x, 5);

  let x = 42;
  println!("Excercise 5 : {}", x); // Prints "42".
  //Excercise -6
  let mut z: i32 = 1;
  z = 7;
  // Shadowing and re-binding
  let mut z = z; //sol : adding mut, this line can be removed too!
  z += 3;
  let y = 4;
  // Shadowing
  let y = "I can also be bound to text!"; 

  println!("Excercise 6 : Success!");
  //Excercise -7
  let a = 1; //sol : use _a or the allow line written on top
  //Excercise -8
  let (mut x, y) = (1, 2);//since we are mutating x, added mut 
  x += 2;

  assert_eq!(x, 3);
  assert_eq!(y, 2);

  println!("Excercise 8 : Success!");
  //Excercise -9
  
  let (x, y);
  (x,..) = (3, 4);//... means we dont care what it is , just focus on x
  [.., y] = [1, 2];//same with y
  // Fill the blank to make the code work
  assert_eq!([x,y], [3,2]);//sol : added the values of x & y

  println!("Excercise 9 : Success!");
  
}
fn define_x() {
    let x = "Excercise";
     println!("{}, 4", x); 
}
