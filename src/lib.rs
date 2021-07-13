fn solution(s: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut get_last = |in_str: &str| -> String {
        if in_str.len() <= 2 {
            return in_str.to_string();
        }
        let split = &in_str.split_at(2);
        result.push(split.0.to_string());

        split.1.to_string()
    };

    let mut last = get_last(s);
    loop {
        match last.len() {
            0 => break,
            1 => {
                last.push('_');
                result.push(last);
                break;
            }
            2 => {
                result.push(last);
                break;
            }
            _ => {
                last = get_last(&last);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
