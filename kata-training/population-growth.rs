/*
In a small town the population is p0 = 1000 at the beginning of a year. The population regularly increases by 2 percent per year and moreover 50 new inhabitants per year come to live in the town. How many years does the town need to see its population greater than or equal to p = 1200 inhabitants?
*/
fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut pop : i32 = p0;
    let mut years : i32 = 0;
    
    while pop < p {
        pop += ((pop as f64) * percent/100 as f64) as i32;
        pop += aug;
        years += 1;
    }
    
    return years;
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    std::iter::successors(Some(p0), |&p0|
        Some(p0 + (p0 as f64 * percent / 100.0) as i32 + aug)
    ).take_while(|&p0| p0 < p).count() as i32
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
  return if p0 >= p {0} 
         else {1 + nb_year(((p0 as f64) * (1.0 + percent/100.0)) as i32 + aug, percent, aug, p)}
}
