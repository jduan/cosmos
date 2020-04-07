/// Group Anagrams
///
/// Given an array of strings, group anagrams together.
///
/// Example:
///
/// Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
/// Output:
/// [
///   ["ate","eat","tea"],
///   ["nat","tan"],
///   ["bat"]
/// ]
///
/// Note:
///
///     All inputs will be in lowercase.
///     The order of your output does not matter.

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::BTreeMap;

        let mut anagrams: BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
        for str in strs {
            let original_str = str.clone();
            let mut chars = str.chars().collect::<Vec<char>>();
            chars.sort();
            let mut sorted_str = String::new();
            for c in chars {
                sorted_str.push(c);
            }
            anagrams
                .entry(sorted_str)
                .and_modify(|v| v.push(str))
                .or_insert({ vec![original_str] });
        }

        let mut ret = vec![];
        for x in anagrams.values_mut() {
            x.sort();
            ret.push(x.to_owned());
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(
            vec![vec!["bat"], vec!["ate", "eat", "tea"], vec!["nat", "tan"],],
            Solution::group_anagrams(vec![
                "eat".into(),
                "tea".into(),
                "tan".into(),
                "ate".into(),
                "nat".into(),
                "bat".into()
            ]),
        );

        assert_eq!(
            vec![vec!["", ""]],
            Solution::group_anagrams(vec!["".into(), "".into()])
        );
    }
}
