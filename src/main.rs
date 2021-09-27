use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess must be between 1 and 100, got {}.", value);
    }
    Guess {value}
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

fn main() {
  println!("数を当ててごらん！");
  let secret_number = rand::thread_rng().gen_range(1..101); // 1..=100
  // println!("秘密の数字は{}", secret_number);
  loop {
    println!("ご予想を入力してください");
    let mut guess = String::new();
    std::io::stdin()
      .read_line(&mut guess)
      .expect("行の読み込みに失敗しました。");

    // let guess: u32 = guess.trim().parse().expect("無効な数字");
    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue
    };

    let guess = Guess::new(guess).value();

    // print!("推測は{}", guess);
    match guess.cmp(&secret_number) {
      Ordering::Greater => println!("大きすぎます！"),
      Ordering::Equal => {
        println!("正解です！");
        break;
      }
      Ordering::Less => println!("小さすぎます！"),
    }
  }
  println!("お疲れ様でした ^_^");
}
