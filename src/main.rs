mod constants;

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
    "8" => problem8(),
    "9" => problem9(),
    "10" => problem10(),
    "11" => problem11(),
    "12" => problem12(),
    "13" => problem13(),
    "14" => problem14(),
    "15" => problem15(),
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

fn problem8() {
  const NUMBER: &'static str = "73167176531330624919225119674426574742355349194934\
                                96983520312774506326239578318016984801869478851843\
                                85861560789112949495459501737958331952853208805511\
                                12540698747158523863050715693290963295227443043557\
                                66896648950445244523161731856403098711121722383113\
                                62229893423380308135336276614282806444486645238749\
                                30358907296290491560440772390713810515859307960866\
                                70172427121883998797908792274921901699720888093776\
                                65727333001053367881220235421809751254540594752243\
                                52584907711670556013604839586446706324415722155397\
                                53697817977846174064955149290862569321978468622482\
                                83972241375657056057490261407972968652414535100474\
                                82166370484403199890008895243450658541227588666881\
                                16427171479924442928230863465674813919123162824586\
                                17866458359124566529476545682848912883142607690042\
                                24219022671055626321111109370544217506941658960408\
                                07198403850962455444362981230987879927244284909188\
                                84580156166097919133875499200524063689912560717606\
                                05886116467109405077541002256983155200055935729725\
                                71636269561882670428252483600823257530420752963450";

  let digits: Vec<u64> = NUMBER
    .chars()
    .map(|d| d.to_digit(10).unwrap() as u64)
    .collect();
  let windows = digits.windows(13);
  let solution = windows
    .map(|window| window.iter().product::<u64>())
    .max()
    .unwrap();

  println!("{}", solution);
}

fn problem9() {
  fn find_triple() -> Option<(u64, u64, u64)> {
    for i in 1u64..1001 {
      for j in (i + 1)..1001 {
        for k in (j + 1)..1001 {
          if i + j + k == 1000 && i.pow(2) + j.pow(2) == k.pow(2) {
            return Some((i, j, k));
          }
        }
      }
    }

    None
  }

  let (a, b, c) = find_triple().unwrap();
  println!("{}", a * b * c);
}

// TODO refactor problems 3 and 7 to use this.
// TODO optimize by skipping already nonprime indices
// Probly going to need to keep using this so here's one in a function.
fn prime_sieve(max: usize) -> Vec<usize> {
  let mut possible_primes = vec![true; max];

  for i in 2..=max {
    let mut factor = i;
    while i * factor <= max {
      let index = (i * factor) - 1 as usize;
      possible_primes[index] = false;
      factor += 1;
    }
  }

  possible_primes
    .iter()
    .enumerate()
    .filter(|x| *x.1)
    .skip(1)
    .map(|x| x.0 + 1)
    .collect::<Vec<usize>>()
}

fn problem10() {
  let primes = prime_sieve(2_000_000 - 1);
  let solution: usize = primes.iter().sum();
  println!("{}", solution);
}

fn problem11() {
  let grid = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

  let grid_matrix: Vec<Vec<u64>> = grid
    .split("\n")
    .map(|row| {
      row
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
    })
    .collect();

  let mut dirs = Vec::with_capacity(7);
  for i in -1..=1i64 {
    for j in -1..=1i64 {
      if i == 0 && j == 0 {
        continue;
      }
      dirs.push((i, j));
    }
  }

  // naive
  // TODO do by sliding windows and ignroe 00 special case
  let mut max_prod = 1u64;
  for i in 0..grid_matrix.len() {
    for j in 0..grid_matrix[0].len() {
      for dir in dirs.iter() {
        let y_end = i as i64 + 4 * dir.0;
        let x_end = j as i64 + 4 * dir.1;

        if y_end < grid_matrix[0].len() as i64
          && y_end >= 0
          && x_end < grid_matrix.len() as i64
          && x_end >= 0
        {
          let mut prod = 1;
          for step in 0..4 {
            prod *=
              grid_matrix[(i as i64 + dir.0 * step) as usize][(j as i64 + dir.1 * step) as usize];
          }
          max_prod = max_prod.max(prod);
        }
      }
    }
  }

  println!("{}", max_prod);
}

fn problem12() {
  fn triangle_number(n: u64) -> u64 {
    n * (n + 1) / 2
  }

  fn find_factors(n: u64) -> Vec<u64> {
    let max = (n as f64).sqrt().ceil() as u64;
    let mut solution = Vec::new();
    for i in 1..max {
      if n % i == 0 {
        solution.push(i);
        solution.push(n / i);
      }
    }

    solution
  }

  let mut i = 1;
  let solution;
  loop {
    let t = triangle_number(i);
    if find_factors(t).len() > 500 {
        solution = t;
      break;
    }
    i += 1;
  };

  println!("{}", solution);
}

use rug;

fn problem13() {
    let big_ints: Vec<rug::Integer> = constants::PROBLEM13_NUMBER_STRINGS.iter().map(|&num| rug::Integer::from(rug::Integer::parse_radix(num, 10).unwrap())).collect();

  let solution: rug::Integer = big_ints.iter().sum();
  println!("{}", solution.to_string_radix(10).split_at(10).0);
}

fn problem14() {
  fn collatz(mut num: u64) -> Option<Vec<u64>> {
    if num <= 0 {
      return None
    }

    let mut solution = Vec::new();

    while num != 1 {
      solution.push(num);
      if num % 2 == 0 {
        num = num / 2;
      } else {
        num = 3 * num + 1
      }
    }

    Some(solution)
  }

  let collatz_lengths: Vec<usize> = (1u64..1_000_000u64).map(|num| collatz(num).unwrap().len()).collect();
  let solution = collatz_lengths.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap().0 + 1;
  println!("{}", solution);
}

fn problem15() {
  fn num_paths_recursive_naive(vert: u64, horiz: u64) -> u64 {
    if vert == 0 && horiz == 0 {
      return 0;
    } else if vert == 0 {
      return 1 + num_paths_recursive_naive(vert, horiz - 1);
    } else if horiz == 0 {
      return 1 + num_paths_recursive_naive(vert - 1, horiz);
    }

    2 + num_paths_recursive_naive(vert - 1, horiz) + num_paths_recursive_naive(vert, horiz - 1)
  }

  // TODO Can be optimized by only storing previous row and column.
  fn num_paths_dp(vert: u64, horiz: u64) -> u64 {
    assert!(vert > 0 && horiz > 0);

    let mut memo = vec![vec![1; (vert + 1) as usize]; (horiz + 1) as usize];

    for i in 0usize..=vert as usize {
      for j in 0usize..=horiz as usize {
        if i > 0 && j > 0 {
          memo[i][j] = memo[i - 1][j] + memo[i][j - 1];
        }
      }
    }

    memo[20][20]
  }

  println!("{}", num_paths_dp(20, 20));
}