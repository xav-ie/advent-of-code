use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // create a hash map of the number counts
    let mut counts: HashMap<i32, (i32, i32)> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let left = parts.next().unwrap().parse::<i32>().unwrap();
        let right = parts.next().unwrap().parse::<i32>().unwrap();
        if let Some(val) = counts.get_mut(&left) {
            let (val_left, _val_right) = val;
            *val_left += 1;
        } else {
            counts.insert(left, (1, 0));
        }

        if let Some(val) = counts.get_mut(&right) {
            let (_val_left, val_right) = val;
            *val_right += 1;
        } else {
            counts.insert(right, (0, 1));
        }
    }

    let mut similarity = 0;
    for (key, (left, right)) in counts.iter() {
        similarity += key * left * right;
    }

    Ok(similarity.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
