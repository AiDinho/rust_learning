use std::f64;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    
    

    let m = factors.iter().fold(u32::max_value(), |a, &b| a.min(b));
    let l = m ;
    let mut total  = 0;
    let mut trav: Vec<u32> = Vec::new();
    for n in ( l..limit){
        for i in factors{

            if *i==0 {
                continue;
            }
            
            if n%i==0{
                if (trav.contains(&n) ==false)
                {
                total+=n;
                trav.push(n);
                }else {
                trav.push(n)
                }

            }
        //println!("total is {}",total);
        }
}
total  as u32

}
