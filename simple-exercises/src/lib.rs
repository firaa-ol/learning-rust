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
fn reverse_integer(number: i32) -> i32{
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn reverse_integer_test() {
        assert_eq!(reverse_integer(123), 321);
        assert_eq!(reverse_integer(208478933), 339874802);
        assert_eq!(reverse_integer(-73634), -43637);
    }
}
