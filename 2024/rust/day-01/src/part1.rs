#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    // TODO: consider using sorted insert
    // https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.binary_search
    // https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.partition_point
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<i32>().unwrap());
        right.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let zipped = left.iter().zip(right.iter());

    let mut distance = 0;
    for (l, r) in zipped {
        // absolute difference
        let diff = l.abs_diff(*r);
        distance += diff;
    }

    Ok(distance.to_string())
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
