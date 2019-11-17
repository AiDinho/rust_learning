
pub fn factors(n: u64) -> Vec<u64> {

      
    let mut fact_list :Vec<u64> = Vec::new();
    //let mut non_fact_list :Vec<u64> = Vec::new();

    if n >1 {
    let mut m = n  ;
    let mut start =2;

    
    //println!("div till is {}",div_till  );
    while m !=1 {
        //println!("start is {}",start );

        if m%start==0 {
            //match is_prime_number(start) {
            //    true => fact_list.push(start),
            //    false => (),
            //}
            fact_list.push(start);
            m = m/start;
        }else {
            //non_fact_list.push(start);
            start= start+1;
        }

    }

    }
    
   fact_list//.iter().cloned().filter(|&x| is_prime_number(x)).collect::<Vec<u64>>()
}

