/**
  * Write a function that, given a string, Finds the longest run of characters
  * and returns an array containing the start and end indices of that run. If
  * there are two runs of equal length, return the first one. For example:
  * longestRun("abbbcc") // [1, 3]
  * longestRun("aabbc")  // [0, 1]
  * longestRun("abcd")   // [0, 0]
  * Try your function with long, random strings to make sure it handles large
  * inputs well.
  **/
pub fn longest_run(run: &str) -> [usize;2] {
    let mut result: [usize; 2] = [0, 0];
    let mut inrun: bool = true;
    let mut start: usize = 0;
    let mut end: usize;
    let mut tempstart: usize = start;
    let mut iter = run.chars().enumerate().peekable();

    while let Some(tuple) = iter.next() {
        if iter.peek() != None {
            inrun = tuple.1 == iter.peek().unwrap().1;
        }
        if !inrun {
            start = tempstart;
            end = tuple.0;
            tempstart = tuple.0 + 1;
            if result[1] - result[0] < end - start {
                result = [start, end];
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::longest_run;
    #[test]
    fn it_works() {
        assert_eq!(longest_run("aabbc"), [0, 1]);
    }
    #[test]
    fn run_in_middle() {
        assert_eq!(longest_run("abbbcc"), [1, 3]);
    }
    #[test]
    fn run_at_end() { 
        assert_eq!(longest_run("aabbcccc"), [4, 7]);
    }
    #[test]
    fn equal_runs() {
        assert_eq!(longest_run("abcd"), [0, 0]);
    }
}
