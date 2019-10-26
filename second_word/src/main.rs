fn main() {
    let  s = String::from("hello world hi there");
    
    println!("second word = {}",second_word(&s));
}

fn second_word(s:&String)->&str{
    let b = s.as_bytes();
    let  mut start_of_sec_word = 0;
    let mut end_of_sec_word = 0;
    for (i,&item) in b.iter().enumerate(){
        if item == b' '{
            if start_of_sec_word !=0 {
                end_of_sec_word = i;
                return &s[start_of_sec_word..end_of_sec_word] ;
            }
            start_of_sec_word = i;
        }

    }
  
   &s[start_of_sec_word..end_of_sec_word]

}
