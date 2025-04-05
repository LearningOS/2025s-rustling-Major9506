// strings3.rs

fn trim_me(input: &str) -> String {
    // 移除字符串两端的空格
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 字符串末尾添加 "world!"
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // 替换字符串中的 "cars" 为 "balloons"
    input.replace("cars", "balloons").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
