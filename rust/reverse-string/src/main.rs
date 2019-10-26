extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
fn main(){
    let a = String::from("bbbc");
    println!("reverse of {} is  {}",a,reverse(&a));

    println!("a is {}",a);
}

fn reverse(input: &str) -> String {
    
    
    let rev:String = input.graphemes(true).rev().collect();

    rev   


}