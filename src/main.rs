fn main() {
  let problem: Vec<String> = std::env::args().collect();
  if problem.len() < 2 {
    println!("usage: euler [problem-number]");
    std::process::exit(1);
  }

  match problem[1].as_str() {
    "1" => problem1(),
    "2" => problem2(),
    "3" => problem3(),
    "4" => problem4(),
    "5" => problem5(),
    "6" => problem6(),
    "7" => problem7(),
    _ => panic!("that euler problem is not implemented!"),
  }
}

fn problem1() {
  let solution: u64 = (1u64..1000u64)
    .into_iter()
    .filter(|number| number % 3 == 0 || number % 5 == 0)
    .sum();
  println!("{}", solution);
}

fn problem2() {
  struct Fib {
    curr: u64,
    next: u64,
  }

  impl Fib {
    fn new() -> Fib {
      Fib { curr: 0, next: 1 }
    }
  }

  impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
      let temp = self.curr;
      self.curr = self.next;
      self.next = temp + self.next;
      Some(self.curr)
    }
  }

  let solution: u64 = Fib::new()
    .take_while(|x| *x < 4_000_000)
    .filter(|x| x % 2 == 0)
    .fold(0, |acc, curr| acc + curr);
  println!("{}", solution);
}

fn problem3() {
  // Have to find largest prime factor of 600851475143
  // make a sieve of all numbers up to sqrt(600851475143).  Largest number in there is the answer.
  // If I make an array of bools that size it is less than a megabyte so...lets do that then filter
  // out things from the tail until we find one that is a factor of 600851475143.
  const NUMBER: u64 = 600851475143u64;
  let number_sqrt: usize = (NUMBER as f64).sqrt().ceil() as usize;

  println!("The sqrt is {}", number_sqrt);

  let mut possible_primes = vec![true; number_sqrt];

  for i in 2..number_sqrt / 2 {
    let mut factor = i;
    while i * factor < number_sqrt {
      let index = (i * factor) - 1 as usize;
      possible_primes[index] = false;
      factor += 1;
    }
  }

  let prime_factors: Vec<(usize)> = possible_primes
    .iter()
    .enumerate()
    .filter(|x| NUMBER % (x.0 as u64 + 1) == 0 && *x.1)
    .map(|x| x.0 + 1)
    .collect();

  println!("{}", prime_factors.last().unwrap());
}

fn problem4() {
  fn is_palindrome_num(num: u64) -> bool {
    let string = num.to_string();
    is_palindrome(&string)
  }

  fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() / 2 {
      if chars[i] != chars[chars.len() - 1 - i] {
        return false;
      }
    }

    true
  }

  let mut largest_palindrome = std::u64::MIN;

  for i in 100u64..999 {
    for j in i..999 {
      let prod = i * j;
      if is_palindrome_num(prod) {
        largest_palindrome = prod.max(largest_palindrome);
      }
    }
  }

  println!("{}", largest_palindrome);
}

fn problem5() {
  let required_divisors: Vec<u64> = (1u64..20u64).collect();
  let max = required_divisors.iter().product();

  fn is_divisible_by_values(values: &[u64], number: u64) -> bool {
    for value in values {
      if number % value != 0 {
        return false;
      }
    }

    return true;
  }

  for i in (20u64..max).step_by(20) {
    if is_divisible_by_values(&required_divisors, i) {
      println!("{}", i);
      break;
    }
  }
}

fn problem6() {
  let range = 1u64..101u64;

  let sum_of_squares: u64 = range.clone().map(|x| x.pow(2)).sum();
  let square_of_sums: u64 = range.clone().sum::<u64>().pow(2);

  println!("{}", square_of_sums - sum_of_squares);
}

fn problem7() {
  // copy pasted the solution for problem 3.
  const NUMBER: u64 = 600851475143u64;
  let number_sqrt: usize = (NUMBER as f64).sqrt().ceil() as usize;

  let mut possible_primes = vec![true; number_sqrt];

  for i in 2..1_000_000 {
    let mut factor = i;
    while i * factor < number_sqrt {
      let index = (i * factor) - 1 as usize;
      possible_primes[index] = false;
      factor += 1;
    }
  }

  println!(
    "{}",
    possible_primes
      .iter()
      .enumerate()
      .filter(|x| *x.1)
      .nth(10_001)
      .unwrap()
      .0
      + 1
  );
}
