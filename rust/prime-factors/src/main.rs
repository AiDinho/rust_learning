use prime_factors as ps;

fn main()  {
     let d =93_819_012_551 ;

     let facs = ps::factors(d);

     println!("factors of {:?} is {:?}",d,facs );
}