let mut s = String :: new();
let data = "intial data";
let s = data.to_string();

/// string :: from and to_string do the same thing 
 let s = String:: from ("initial data")

 //Updating a string
  s.push_str("now"); //or 
  let s2 ="now";
  s.push_str(s2);

  //Adding strings
   let s1 = String :: from ("hello");
   let s2 = String :: from ("world ");
   let s3 = s1 + &s2 ;
        // or we can use fromat! macro that works like println but return content of the Strings
        let s3 = format!({s1}-{s2});

    // Indexing String
    let a = "helloooo";
    let b = &a [0..5];
    
    // Iterating over strings
         //char method
        for c in "hello".char(){
        println("{c}");
    }
        //byte method
        for i in "hello".bytes(){
            println("{i}");
        }


 