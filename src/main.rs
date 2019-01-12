fn main() {
  let problem: Vec<String> = std::env::args().collect();
  if problem.len() < 2 {
    println!("usage: euler [problem-number]");
    std::process::exit(1);
  }

  match problem[1].as_str() {
    "1" => problem1(),
    "2" => problem2(),
    _ => panic!("that euler problem is not implemented!")
  }
}

fn problem1() {
  let solution: u64 = (1u64..1000u64).into_iter()
    .filter(|number| number % 3 == 0 || number % 5 == 0).sum();
  println!("{}", solution);
}

fn problem2() {
  struct Fib {
    curr: u64,
    next: u64,
  }

  impl Fib {
    fn new() -> Fib {
      Fib {
        curr: 0,
        next: 1,
      }
    }
  }

  impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
      (self.curr, self.next) = (self.next, self.curr + self.next);
      Some(self.curr)
    }
  }

  let solution: u64 = Fib::new()
    .take_while(|x| *x < 4_000_000)
    .filter(|x| x % 2 == 0)
    .fold(0, |acc, curr| acc + curr);
  println!("{}", solution);
}
