// options1.rs

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
     // 检查时间是否有效（0-23）
     if time_of_day > 23 {
        return None;
    }

    // 根据时间返回冰淇淋数量
    if time_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
       // 使用 unwrap() 或 expect() 从 Option 中获取值
       let icecreams = maybe_icecream(12).unwrap();
       assert_eq!(icecreams, 5);
    }
}
