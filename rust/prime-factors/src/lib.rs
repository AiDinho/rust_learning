pub fn factors(n: u64) -> Vec<u64> {

    let mut m = n  ;  
    let mut fact_list :Vec<u64> = Vec::new();
    
    let div_till = match n {
        2 => n,
        _ => n/2 as u64,
    };
    let mut start =2;
    println!("div till is {}",div_till  );
    while start <= div_till{
        if m%start==0{
            match is_prime_number(start) {
                true => fact_list.push(start),
                false => (),
            }
            m = m/start;
        }else {
            start= start+1;
        }

    }
    
   fact_list
}

fn is_prime_number(n: u64) -> bool {
    let d = n/2 as u64; 
  !(2..d ).any(|a| n % a == 0)
}
