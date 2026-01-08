# CodeWars-The-Hashtag-Generator-5-kyu---Passed-
The marketing team is spending way too much time typing in hashtags.
Let's help them with our own Hashtag Generator!

Here's the deal:

It must start with a hashtag (#).
All words must have their first letter capitalized, and remaining letters lowercased.
If the final result is longer than 140 chars it must return None.
If the input or the result is an empty string it must return None.
Examples
" Hello there thanks for trying my Kata"  =>  Some("#HelloThereThanksForTryingMyKata")
"    Hello     World   "                  =>  Some("#HelloWorld")
""                                        =>  None

TEST CASES:
#[cfg(test)]
mod tests {
    use super::generate_hashtag;
    use rand::{distributions::DistString, prelude::*};

    fn dotest(s: &str, expected: Option<String>) {
        let actual = generate_hashtag(s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) when testing with s = {s:?}"
        );
    }

    #[test]
    fn _1_fixed_tests() {
        dotest("", None);
        dotest("Codewars", Some("#Codewars".to_owned()));
        dotest("Codewars      ", Some("#Codewars".to_owned()));
        dotest("Codewars Is Nice", Some("#CodewarsIsNice".to_owned()));
        dotest("codewars is nice", Some("#CodewarsIsNice".to_owned()));
        dotest("CodeWars is nice", Some("#CodewarsIsNice".to_owned()));
        dotest("c i n", Some("#CIN".to_owned()));
        dotest("codewars  is  nice", Some("#CodewarsIsNice".to_owned()));
        dotest("Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong Cat",
            None);
    }

    #[test]
    fn _2_random_tests() {
        let mut rng = thread_rng();
        for _ in 0..200 {
            let mut s = String::new();
            for _ in 0..rng.gen_range(1..15) {
                let word_len = rng.gen_range(1..20);
                ASCIILettersAndSpace.append_string(&mut rng, &mut s, word_len);
                s.push(' ')
            }
            s.pop();
            dotest(&s, reference_solution(&s));
        }
    }

    struct ASCIILettersAndSpace;

    impl Distribution<u8> for ASCIILettersAndSpace {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
            const ASCII_LETTERS_AND_SPACE: &[u8] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz";
            const RANGE: std::ops::Range<usize> = 0..ASCII_LETTERS_AND_SPACE.len();

            ASCII_LETTERS_AND_SPACE[rng.gen_range(RANGE)]
        }
    }

    impl DistString for ASCIILettersAndSpace {
        fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
            string.reserve(len);
            unsafe {
                // Safety: generates valid ASCII codepoints
                let v = string.as_mut_vec();
                v.extend(self.sample_iter(rng).take(len));
            }
        }
    }

    fn reference_solution(s: &str) -> Option<String> {
        let res = format!(
            "#{}",
            s.split_whitespace()
                .map(|w| capitalize(w))
                .collect::<String>()
        );
        if res.len() < 141 && res.len() > 1 {
            Some(res)
        } else {
            None
        }
    }

    fn capitalize(s: &str) -> String {
        format!(
            "{}{}",
            s.chars().next().unwrap().to_uppercase(),
            s[1..].to_lowercase()
        )
    }
}
