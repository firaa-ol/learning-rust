/*
    Write a program to reverse the digits of a given integer.
    Example:
    Input:
    i = 123
    i = 208478933
    i = -73634
    Output:
    Reverse integer: 321
    Reverse integer: 339874802
    Reverse integer: -43637
*/
pub fn reverse_integer(number: i32) -> i32{
    let mut num = number;
    let is_negative = num < 0;
    let mut res = 0;

    if is_negative {
        num = -1 * num;
    }

    while num != 0 {
        res = res * 10 + (num % 10);
        num = num / 10;
    }

    if is_negative {
        -1 * res
    } else {
        res
    }
}

/*
 Write a program to divide two integers (dividend and divisor) without using multiplication, division and mod operator
 */
pub fn divide(num1 : i32, num2 : i32) -> i32{
    let mut count : i32 = 0;
    let mut first_num = num1.abs();
    let second_num = num2.abs();

    while first_num >= second_num {
        count += 1;
        first_num -= second_num;
    }

    if (num1 < 0) ^ (num2 < 0) {
        -1 * count
    } else {
        count
    }
    
}

pub fn get_circle_area(radius : f64) -> f64{
    let area : f64 = (radius * radius) * std::f64::consts::PI;

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_integer_test() {
        assert_eq!(reverse_integer(123), 321);
        assert_eq!(reverse_integer(208478933), 339874802);
        assert_eq!(reverse_integer(-73634), -43637);
    }

    #[test]
    fn divide_test() {
        assert_eq!(divide(7, 2), 3);
        assert_eq!(divide(-17, 5), -3);
        assert_eq!(divide(35, 7), 5);
        assert_eq!(divide(-39, -4), 9);
    }
}
