use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 12);
    scores.insert(String::from("Yellow"), 6);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //get gives Option<&V>, copied changes it to Option<V>, unwrap_or(0) sets to 0 if none
    scores.entry(String::from("Yellow")).or_insert(50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){//returns iterator over the slices
        let count = map.entry(word).or_insert(0); //o insert returns mutable reference
        //count point to value of word if word exists, elsse add as word 0 and point to that 0
        *count +=1;
    }

    println!("{map:?}");
}
