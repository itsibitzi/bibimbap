pub fn decompose(s: &str) -> String {
    hangeul::decompose(s).into_iter().zip(s.chars()).fold(
        String::new(),
        |mut acc, (decomposed_char, original)| match decomposed_char {
            Ok((a, b, Some(c))) => {
                acc.push(a);
                acc.push(b);
                acc.push(c);
                acc
            }
            Ok((a, b, None)) => {
                acc.push(a);
                acc.push(b);
                acc
            }
            // First and second glyph aren't valid jamo.
            // We're probably decomposing a mix of Korean and non-Korean
            // Just push the original character
            Err(_) => {
                acc.push(original);
                acc
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::hangul::decompose;

    #[test]
    fn can_decompose_mixed() {
        let input = "english 한국어 francais";
        let output = decompose(input);

        assert_eq!(output, "english ㅎㅏㄴㄱㅜㄱㅇㅓ francais");
    }
}
