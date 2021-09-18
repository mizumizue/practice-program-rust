fn main() {
    println!("{}", "Hello, world!!!");

    // Byteから文字列に戻す
    let target = String::from("おはよう,世界！！！");
    println!("{}", String::from_utf8(target.into_bytes()).unwrap());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    // 文字列の切り出し
    fn test_cut_str() {
        let target = String::from("hello, world");
        assert_eq!("", "");
    }

    #[test]
    // 文字列分割
    fn test_split_str() {
        let target = String::from("hoge/foo/bar");
        let split: Vec<&str> = target.split("/").collect();
        assert_eq!(split, vec!["hoge", "foo", "bar"])
    }

    #[test]
    // 検索(文字列を含んでいるか)
    fn test_replace_str() {
        let target = String::from("abcdefg");
        assert_eq!( target.contains("bcd"), true);
        assert_eq!( target.contains("hij"), false);
    }

    #[test]
    // 文字列の結合
    fn test_append_str() {
        let mut target = String::from("abcdefg");
        target.push_str("hijklmn");
        assert_eq!("abcdefghijklmn", target)
    }

    #[test]
    // 文字列の長さ
    fn test_str_len() {
        let target = String::from("abcdefg");
        assert_eq!(target.len(), 7);
    }

    #[test]
    // 前後の空白除去
    fn test_trim_str() {
        let target = String::from(" WhiteSpace ");
        assert_eq!(target.trim(), "WhiteSpace");
    }

    #[test]
    // 全て大文字英字にする
    fn test_to_uppercase_str() {
        let target = String::from("abcdefg");
        assert_eq!(target.to_uppercase().as_str(), "ABCDEFG");
    }

    #[test]
    // 全て小文字英字にする
    fn test_to_lowercase_str() {
        let target = String::from("ABCDEFG");
        assert_eq!(target.to_lowercase().as_str(), "abcdefg");
    }

    #[test]
    // 文字列を数値にする
    fn test_replace() {
        let target = String::from("1");
        let res = target.parse::<i32>().unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    // 1文字ずつ取り出す
    fn test_get_char() {
        let v = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n"];
        let target = String::from("abcdefghijklmn");
        let mut i = 0;
        for c in target.chars() {
            assert_eq!(c.to_string(), v.get(i).unwrap().to_string());
            i = i + 1
        }
    }

    #[test]
    // 1文字ずつ取り出す（マルチバイト文字）
    fn test_get_char_multibyte() {
        let v = vec!["お", "は", "よ", "う", ",", "世", "界", "！", "！", "！"];
        let target = String::from("おはよう,世界！！！");
        let mut i = 0;
        for c in target.chars() {
            assert_eq!(c.to_string(), v.get(i).unwrap().to_string());
            i = i + 1
        }
    }
}