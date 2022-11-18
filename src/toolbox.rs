
pub fn is_bool_true(s: &str) -> bool {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "ok" => true,
        _ => false
    }
}

pub fn is_bool_false(s: &str) -> bool {
    !is_bool_true(s)
}




#[cfg(test)]
mod test {
    use crate::is_bool_true;

    #[test]
    fn test_bool_str() {
        for s in ["true", "1", "yes", "ok"] {
            assert!(is_bool_true(s))
        }
        for s in ["false", "0", "no", "else"] {
            assert!(!is_bool_true(s))
        }

    }
}

use std::str::Chars;

// show max_n chars, if longer, use ellipsis to replace it
pub fn max_n_chars(input: Chars, max_n: usize, ellipsis: &str) -> String {
    let max_chars: Vec<_> = input.take(max_n + 1).collect();

    // if > max_n, represented as: "text..."
    if max_chars.len() > max_n {
        max_chars[0..(max_n - ellipsis.chars().count())].iter().collect::<String>() + ellipsis
    } else {
        max_chars.iter().collect()
    }
}

#[cfg(test)]
mod test2 {
    use crate::chars::max_n_chars;
    use std::iter::repeat;


    #[test]
    fn test_max_n_chars() {
        let max_n = 16;
        let ellipsis = "...";
        let might_be_n = max_n - ellipsis.chars().count();
        println!("max_n: {}, ellipsis: {}, might_be_n: {}", max_n, ellipsis, might_be_n);

        let s1 = max_n_chars(repeat("a").take(20).collect::<String>().chars(), max_n, ellipsis);
        println!("{}", s1);
        assert_eq!(s1, repeat("a").take(might_be_n).collect::<String>() + ellipsis);

        let s1 = max_n_chars(repeat("a").take(10).collect::<String>().chars(), max_n, ellipsis);
        println!("{}", s1);
        assert_eq!(s1, repeat("a").take(10).collect::<String>());

        let s1 = max_n_chars(repeat("a").take(max_n).collect::<String>().chars(), max_n, ellipsis);
        println!("{}", s1);
        assert_eq!(s1, repeat("a").take(max_n).collect::<String>());


        let max_n = 16;
        let ellipsis = " 等等";
        let might_be_n = max_n - ellipsis.chars().count();
        println!("max_n: {}, ellipsis: {}, might_be_n: {}", max_n, ellipsis, might_be_n);

        let s1 = max_n_chars(repeat("字").take(20).collect::<String>().chars(), max_n, ellipsis);
        println!("{}", s1);
        assert_eq!(s1, repeat("字").take(might_be_n).collect::<String>() + ellipsis);

        let s1 = max_n_chars(repeat("字").take(10).collect::<String>().chars(), max_n, ellipsis);
        println!("{}", s1);
        assert_eq!(s1, repeat("字").take(10).collect::<String>());

        let s1 = max_n_chars(repeat("字").take(max_n).collect::<String>().chars(), max_n, ellipsis);
        println!("{}", s1);
        assert_eq!(s1, repeat("字").take(max_n).collect::<String>());


        let max_n = 16;
        let ellipsis = "...";
        let might_be_n = max_n - ellipsis.chars().count();
        let text = "今晚九点直播‼️\n我们今晚的内容，不仅面向"; // !!Warning!!, '‼' is a single char!!!, different from '!'.
        for (i, v) in text.chars().enumerate() {
            println!("{}: {:?}, {}",i, v, v);
            //     0: '今', 今
            // 1: '晚', 晚
            // 2: '九', 九
            // 3: '点', 点
            // 4: '直', 直
            // 5: '播', 播
            // 6: '‼', ‼     <-------
            // 7: '\u{fe0f}', ️
            // 8: '\n',      <------- \n 换行了
            //
            // 9: '我', 我
            // 10: '们', 们
            // 11: '今', 今
            // 12: '晚', 晚
            // 13: '的', 的
            // 14: '内', 内
            // 15: '容', 容
            // 16: '，', ，
            // 17: '不', 不
            // 18: '仅', 仅
            // 19: '面', 面
            // 20: '向', 向
        }
        println!("\n");
        for (i, v) in "播‼️\n播".chars().enumerate() {
            println!("{}: {:?}, {}",i, v, v);
            // 0: '播', 播
            // 1: '‼', ‼
            // 2: '\u{fe0f}', ️
            // 3: '\n',
            //
            // 4: '播', 播
        }
        println!("\n");
        for (i, v) in "‼️cd".chars().enumerate() {
            println!("{}: {:?}, {}",i, v, v);
            // 0: '‼', ‼
            // 1: '\u{fe0f}', ️
            // 2: 'c', c
            // 3: 'd', d
        }

        println!("\n");
        for (i, v) in "‼️".chars().enumerate() { // here, "‼️" has 2 chars, including a invisible char at tail, put mouse cursor at tail of "‼️" and move right of the cursor to see its existence.
            println!("{}: {:?}, {}",i, v, v);
            // 0: '‼', ‼
            // 1: '\u{fe0f}', ️
        }

        println!("\n");
        for (i, v) in "‼".chars().enumerate() { // here, "‼️" has only 1 char, no invisible char. Can only be copied.
            println!("{}: {:?}, {}",i, v, v);
            // 0: '‼', ‼
        }

        println!("max_n: {}, ellipsis: {}, might_be_n: {}", max_n, ellipsis, might_be_n);

        println!("text: {:?}, len: {}, chars count: {}", text,  text.len(), text.chars().count());

        let s1 = max_n_chars(text.chars(), max_n, ellipsis);
        println!("{:?}", s1);
        assert_eq!(s1, text.chars().take(might_be_n).collect::<String>() + ellipsis);
    }
}