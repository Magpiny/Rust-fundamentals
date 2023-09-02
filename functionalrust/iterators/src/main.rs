/// # Iterators in Rust
/// ## Examples
///```
///fn main(){
///   let numbers: Vec<i32> = vec![2, 3, 4];
///   let copied_array: Vec<_> = vector1.iter().copied().map(|x| x.pow(2)).collect();
///   println!("New copied elements are: {:?}", copied_array);
/// }
/// ```
/// ### Output: New copied elements are: [4, 9, 16]
/// Documented by Wanjare S;
// END OF DOCUMentation
fn main() {
    let numbers: Vec<i32> = vec![2, 3, 4, 5, 6, 7, 8, 123];
    let names: Vec<&str> = vec!["John Doe", "Magpiny BO", "Graham Bell"];

    // creating an iterator
    let iter_v1 = numbers.iter();

    // using an iterator in a for loop
    for num in iter_v1 {
        println!("The square of {} is {}", num, num.pow(2));
    }

    for name in names.iter() {
        println!("{} likes Chapati", name);
    }

    // OFF topic
    let hello = String::from("Hello world");
    println!("{:?}", hello.replace("world", "Toto"));

    let num2: i32 = 100;
    println!("The log of {num2} to base 10 is {}", num2.ilog(10));
    println!("{}", usize::BITS);

    // # Methods that consume iterator
    //sum, count, product etc
    let vector1 = vec![1, 2, 3, 4];

    // get number of items in the vector1
    let num_items = vector1.iter().count();

    // sum all the items in the vector1
    let sum_iters: i32 = vector1.iter().sum();

    // multiply all the items in the vector1
    let prod: i32 = vector1.iter().product();

    // create a new collection copied from another
    let copied_array: Vec<_> = vector1.iter().copied().map(|x| x.pow(2)).collect();

    // using filter consumer
    // let filtered: Vec<_> = numbers.iter().filter(|x| x > &&6).collect();
    // let filtered: Vec<_> = numbers.iter().filter(|&&x| x > 6).collect();
    let filtered: Vec<_> = numbers.iter().filter(|x| **x > 6).collect();
    // let filtered: Vec<_> = numbers.iter().filter(|&x| *x > 6).collect();

    println!("There are {} items in vector1", num_items);
    println!("The summation of vector1 items is {}", sum_iters);
    println!("The product of all items in vector1 is {}", prod);
    println!("New copied elements are: {:?}", copied_array);
    println!("New filtered array: {:?}", filtered); // [7, 8,123]
}
