use std::collections::HashSet;

/// Destination City
///
///     User Accepted: 4760
///     User Tried: 4991
///     Total Accepted: 4800
///     Total Submissions: 5600
///     Difficulty: Easy
///
/// You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a direct
/// path going from cityAi to cityBi. Return the destination city, that is, the city without any
/// path outgoing to another city.
///
/// It is guaranteed that the graph of paths forms a line without any loop, therefore, there will
/// be exactly one destination city.
///
///  
///
/// Example 1:
///
/// Input: paths = [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]
/// Output: "Sao Paulo"
/// Explanation: Starting at "London" city you will reach "Sao Paulo" city which is the destination
/// city. Your trip consist of: "London" -> "New York" -> "Lima" -> "Sao Paulo".
///
/// Example 2:
///
/// Input: paths = [["B","C"],["D","B"],["C","A"]]
/// Output: "A"
/// Explanation: All possible trips are:
/// "D" -> "B" -> "C" -> "A".
/// "B" -> "C" -> "A".
/// "C" -> "A".
/// "A".
/// Clearly the destination city is "A".
///
/// Example 3:
///
/// Input: paths = [["A","Z"]]
/// Output: "Z"

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut sources = HashSet::new();
        let mut dests = HashSet::new();

        for mut path in paths {
            let src = path.remove(0);
            let dest = path.remove(0);

            sources.insert(src.clone());
            if dests.contains(&src) {
                dests.remove(&src);
            }
            if (!dests.contains(&dest)) && (!sources.contains(&dest)) {
                dests.insert(dest);
            }
        }

        println!("dests: {:?}", dests);
        dests.iter().next().unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dest_city() {
        assert_eq!(
            "Sao Paulo".to_string(),
            Solution::dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ])
        );
    }
}
