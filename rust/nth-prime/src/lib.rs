pub fn nth(n: u32) -> u32 {
    let mut prime_list = vec![2];
    let mut num =3;
    let m =n+1;
    
while (prime_list.len() < m as usize) {
    let mut is_prime = true;
    for i in &prime_list{
        if (num % i == 0){
            is_prime = false;
            break;
        }
    }
    if is_prime {
        &prime_list.push(num);

    }
    num = num+2;
}
   let length = prime_list.len();
   //println!("last no is {:?} ",prime_list );

    prime_list[length-1]
}
