pub fn is_leap_year(year: i32) -> bool {
  match (year % 4 == 0, year % 100 == 0, year % 400 == 0) {
    (true, false, false) => true,
    (true, true, false) => false,
    (_, _, true) => true,
    (false, _, _) => false,
  }
}
