//The type HashMap<k, v> stores a mapping of keys of type k to valuse of type v using hashing factor. which shows how it places these keys and values into memory.
// useful when you want to look up data not by using index like vectors.

use std ::collections::HashMap;
let scores = HashMap::new();
 scores.insert(String:: from("blue"),10); // adding element with "insert"
 scores.insert(String::from("yellow"), 50);

//Accessing values 
let team_name= String::from("blue");
let score = scores.get(&team_name).copied().unwraped_or(0); // "get" method returns an option<&v>
// "copied" get an option<i32> rather than option<&i32> , "unwrap_or" to set "score" to 0 if scores doesnt have anyy entry

//Iterating
 for (key, value) in &scores{
    println!("{key} : {value}");
 }

//Updating a HashMap
// if you insert a new value with the same key it just replaced.
    scores.insert(String::from("blue"), 25);
    //replaced with 25










fn main() {
    println!("Hello, world!");
}
