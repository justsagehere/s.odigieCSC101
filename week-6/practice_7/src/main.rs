fn main(){

    // Array with data types (explicit integer datatype)
    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("array is{:?}", arr1);
    println!("Array size is :{}", arr1.len());

    // Array without data types (explicit integer datatype)
    let arr2= [10.4,20.7,30.4,40.9,51.2,72.2];
     println!("\nArray without data type");
    println!("array is{:?}", arr2);
    println!("Array size is :{}", arr2.len());

    //array with default value that creates and
    //initializes all its element with a default value of -1.
    let arr3:[i32;8] = [-1;8];
     println!("\nArray with default values");
    println!("array is{:?}", arr3);
    println!("Array size is :{}", arr3.len());
}