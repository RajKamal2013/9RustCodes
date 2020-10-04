use std::mem;
fn main() {
    // Print Hello World
    println!("Hello, world!");

    // Playing with Variables 
    //------------Immutable Variables ---------------------------------
    let a:u8=123;  // Binding is Immutable : by default Rust bindings are immutable
    println!("a = {}", a);
    //a = 100; // Error -> trying to change immutable variable. -> so comment it out 

    //--------------Mutable Variables ----------------------------------
    let mut b:i8=0; // Binding is mutable 
    println!("b = {}", b);
    b = 100; // No issue as we have made variable mutable. 
    println!("b = {}", b);
    // Integers are -> i8, u8, i16, u16, u32, i32, i64, u64

    //----- Define variable without prototyping ------------------------
    let c = 234;
    println!("c = {}", c);

    let mut c1 = 245;
    println!("c1 = {}", c1);
    c1 = -1;
    println!("c1 = {}", c1);  // by default integers are signed 

    //--------------- size of variable --------------------------------
    let z:isize = 123; // -> size of pointers 
    let size_of_z = mem::size_of_val(&z);   // --> mem::size_of_val(&var) --> defines the variables size in rust 
    println!("z = {}. takes up {} bytes, {}-bit os ", z, size_of_z, size_of_z * 8);
    // pointer size -> usize, isize

    //------------------characters in Rust -----------------------------
    let d='x';
    let d1:char='y';
    println!("d = {}, size = {} bytes, d1= {}, size={}", d, mem::size_of_val(&d), d1, mem::size_of_val(&d1));

    //-----------------floating point ----------------------------------
    let e = 2.5;
    let e1:f32=3.5;
    println!("e = {}, size = {}, e1= {}, size = {}", e, mem::size_of_val(&e), e1, mem::size_of_val(&e1));
    // Floating point -> f32, f64


    //-------------------boolean values ----------------------------------
    //true and false
    let g = false;
    println!("g = {}, size = {}", g, mem::size_of_val(&g));
    let f = 4 > 0;
    println!("f = {}, size = {}", f, mem::size_of_val(&f));


    //-------------------Arithematic Operators -----------------------------
    let mut ar = 2 + 3 * 4;  // arithematic operators are +,-,*,/,%
    println!("ar = {}", ar);
    //ar++ // error in RUST !!!
    ar = ar + 1 ; // Rust does not support ++ and --, allthough rust support +=, -=, *=, /= ,%=
    println!("ar = {}", ar);
    ar -= 2;
    println!("ar = {}", ar);
    // Rust does not have power operator. we need to use pow(), powi(), powf()
    let ar_cubed = i32::pow(ar, 3);
    println!("{} cube = {}", ar, ar_cubed);

    let br = 2.5;
    let br_cubed = f64::powi(br, 3);
    let br_to_pi = f64::powf(br, std::f64::consts::PI);
    println!("{} cubed = {}, {}^ pi = {} ", br, br_cubed, br, br_to_pi);


    //--------------------Bitwise operator --------------------------------------
    //only available for integers
    let cb = 1 | 2;   // | => OR, & => AND, ^ => XOR, ! => NOR, >> or << => left and right shift
    println!("cb = {}", cb);
    let two_to_10 = 1 << 10;
    println!("two_to_10={}", two_to_10);


    //--------------------Logical Operators -------------------------------------
    // >, <, ==, !=, <=, >=, 
    let xl = 5;
    let xl_is_5 = xl == 5;
    println!("{} is 5= {}", xl, xl_is_5);


    //---------------SCOPE and shadowing -----------------------------------------
    // Rust support redeclaration all though it takes the last one 
    let al= 123;
    let al= 100;
    println!("al = {}", al);
    {
        let al= 456;
        println!("Inside , as = {}", al);
        let bs = 400;
        println!("Inside , bs = {}", bs);

    }

    //------------CONSTANTS ------------------------------------------------
    const MEANING_OF_LIFE:u8 = 42; // we need to give type 
    println!("{}", MEANING_OF_LIFE);
    println!("{}", 42);

    // no fixed address of constant
    // we can use static for that 
    
    static ZS:u8 = 142;
    static mut ZS1:u8=142;
   // println!("zs1 = {}", ZS1); //===?> error 
   unsafe
   {
    println!("zs1 = {}", ZS1);
   }


   //-----------------if constructs ---------------------------------------
   let mut temp = 33;
   if temp > 30 
   {
       println!("Hot Outside");
   }
   else if temp < 10
   {
       println!("Really cold");
   }
   else
   {
       println!("moderate");
   }

   temp = 5;
   // if statement is expression in RUST 
   let clay = if temp > 20 {"sunny"} else {"cloudy"};
   //println!("is it {}"), if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"okay"});
   println!("Clay is {}", clay);

   //-------------------while construct------------------------------------
   let mut xw = 1;
   while xw < 1000
   {
       xw *= 2;
       if xw == 64
       {
           continue;
       }
       println!("xw = {}", xw);
   }

   //---------------------loop construct ---------------------------------
   let mut yw = 1;
   loop         // while true --> inifinite loop 
   {
       yw *= 2;
       println!("yw = {}", yw);
       if yw == 1 << 10
       {
           break;
       }
   }

   //--------------------for construct ------------------------------------
   for xf in 1..11 // loop from 1 to 10
   {
       if xf == 3 {continue;}
       if xf == 8 {break;} 
       println!("xf = {}", xf);
   }

   for (pos, yf) in (30..41).enumerate()
   {
       println!("{}: {}", pos, yf);
   }


   //-------------------match construct --------------------------------
   let country_code = 44;
   let country = match country_code
   {
       44 => "UK",
       46 => "Sweden",
       7 => "Russia",
       1...999 => "Unknown",  // range 1 to 999  including 999
       _ => "invalid"
   };
   println!("country code {}: {}", country_code, country);
   //--------------------------Data Structure --------------------------
}