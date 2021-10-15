use rand::seq::SliceRandom;

fn main() {

  let random_shit = vec!["hello" , "hello" , "sup" , "hello gay", "fuck off" , "nigga"]

  let gay: Vec<_> = random_shit
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();

  println!("{:?}" , gay)
  
}
