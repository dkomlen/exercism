//on every year that is evenly divisible by 4
//  except every year that is evenly divisible by 100
//    unless the year is also evenly divisible by 400
pub fn is_leap_year(year: i32) -> bool {
    let divisible_by = |num: i32| -> bool { year % num == 0 };
    if divisible_by(400) { true } else if divisible_by(100) { false } else { divisible_by(4) }
}
