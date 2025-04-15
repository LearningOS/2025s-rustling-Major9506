// options2.rs

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用 if let 匹配 Some 类型
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // 使用 while let 处理 Option 层次
        while let Some(integer) = optional_integers.pop() {
            if let Some(value) = integer {
                assert_eq!(value, cursor);
                cursor -= 1;
            } else {
                // 如果弹出的是 None，直接跳过
                continue;
            }
        }

        assert_eq!(cursor, 0);
    }
}
