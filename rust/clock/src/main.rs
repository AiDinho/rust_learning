use clock::Clock;

fn  main() {
    let clock = Clock::new(0, 3).add_minutes(-4);
    
    println!("clock value {}",clock);
}
