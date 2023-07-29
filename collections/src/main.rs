/*
* -> COLLECTIONS
*Colections are stored in the heap and not in the stack like other data types, giving them
*the ability to grow and shrink as need be
*Collections are poped out the memory once they get out of scope
*
*There are THREE Colletion types in Rust
*   -> Vectors
*   -> Strings
*   -> HashMaps
*
*/

// Vectors<T> T for type
// Vectors can store any type of values including ones defined in enums
// Values in a vector 'MUST' be of the same type
// With enums, we can make vectors store different values
use std::collections::HashMap;

#[allow(unused)]
fn main() {
    println!("Hello, world!");

    //vectors
    my_vectors();

    // HashMap
    my_hashmap();
}

#[allow(unused)]
// Vectors<T>
fn my_vectors() {
    //declaring a vector
    let mut vec1: Vec<i32> = Vec::new(); //1
    vec1.push(23);
    vec1.push(13);

    let vec2 = vec![12, 13, 14]; //2

    // Accessng values in a vector
    let newvec = vec![1, 2, 3, 4, 5];
    let third_el = &newvec[3];
    let sec_el = &newvec.get(2);
    println!("Third element: {third_el}");

    match (sec_el) {
        Some(sec_el) => println!("Second Element: {sec_el}"),
        None => println!("No second element in the vector!"),
    }

    // Iterating over vectors
    let fruits = vec!["mango", "oranges", "avocado"];
    for fruit in &fruits {
        println!("I like {fruit}");
    }

    // Iterating over mutable references
    let mut nums = vec![2, 3, 4, 5, 6];
    for num in &mut nums {
        let sqr = *num * *num;
        println!("The square of {num} is {sqr}");
    }

    // Using enums to Store Multiple Types in a Vector
    #[derive(Debug)]
    enum Student {
        AdmNo(i16),
        Name(String),
        Weight(f32),
    }
    let student = vec![
        Student::AdmNo(4050),
        Student::Name(String::from("Samuel Wanjare")),
        Student::Weight(59.89),
    ];

    let stud_name: Option<&Student> = student.get(1);
    match (stud_name) {
        Some(stud_name) => println!("{:#?}", stud_name),
        None => println!("This student has no name!"),
    }
}

#[allow(unused)]
//String::is a growable, mutable, owned, UTF-8 encoded string type.
fn my_string() {
    // Creating a string
    let mut s = String::new(); // creates a new empty string s
    let new_s = "Hello world";
    let data = new_s.to_string();
    let s2: String = String::from("Hello July");
    let s3: String = String::from("Good morning");

    // Updating a string
    s.push_str(new_s);

    // Concatenation with the + Operator or the format! Macro
    // we can only add a &str to a String; we can’t add two String values together
    let s1 = s2 + &s3;

    // Using format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // format returns a string with contents passed to it!
    println!("{s}");

    // Rust Strings do not support indexing so s3[0] is invalid
}

#[allow(unused)]
// hashmap
/*
Many programming languages support this kind of data structure, but they often use
a different name, such as hash, map, object, hash table, dictionary, or associative array

The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing
function, which determines how it places these keys and values into memory.

*/
fn my_hashmap() {
    // Creating a hash map
    let mut scores = HashMap::new();

    let sub1 = String::from("Maths");
    let sub1grade = String::from("A");
    let sub2 = String::from("Computer");
    let sub2grade = String::from("A");

    scores.insert(sub1, sub1grade);
    scores.insert(sub2, sub2grade);

    // Accessing values in a HashMap
    // We can get a value out of the hash map by providing its key to the get method
    let nothing = String::from("Not there!");

    let subj1 = String::from("Blue");
    let subj2 = String::from("Computer");

    let math_grade = scores.get(&subj1).unwrap_or(&nothing);
    let comp_grade = scores.get(&subj2).unwrap_or(&nothing);

    println!("Maths grade is {}", math_grade);
    println!("Computer grade is {}", comp_grade);

    // Iterating over a Hashmap
    // Using for ... in loop
    println!("------------Looping in HashMaps----------------");

    for (subject, grade) in &scores {
        println!("{subject}: {grade}");
    }

    // Overwriting a value
    scores.insert(String::from("Eng"), String::from("A-"));
    scores.insert(String::from("Eng"), String::from("B+"));
    println!("Scores: {:?}", scores); //->{"Computer":"A", "Maths":"A", "Eng":"B+"}

    // Updating a Value based on Old value
    //
    // counts how many times each word appears in some text
    let text = "Hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

// THE END
