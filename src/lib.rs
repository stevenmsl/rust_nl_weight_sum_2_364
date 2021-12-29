#[derive(Debug, PartialEq)]

pub enum NestedInteger {
    List(Vec<NestedInteger>),
    Integer(i32),
}

pub struct Solution {}

/* keys
   - This is a bit more complex than question 339.
     here we need to defer the calculation of the sum
     at a given level until every element has been visited
     at that level to determine how far away the given
     level is from the most bottom level
   - each recursive call to depth_sum_internal will
     give you the number of levels from the most
     bottom level
*/

impl Solution {
    pub fn depth_sum(list: Vec<NestedInteger>) -> i32 {
        let mut sum = 0;
        Self::depth_sum_internal(&list, &mut sum);
        sum
    }

    fn depth_sum_internal(list: &Vec<NestedInteger>, sum: &mut i32) -> isize {
        let mut level_sum = 0;

        /*
          - track how far away we are from the most
            bottom level
        */
        let mut max_level = 1;
        for elment in list.into_iter() {
            match elment {
                NestedInteger::List(some_list) => {
                    let level_found = Self::depth_sum_internal(some_list, sum);
                    if level_found > max_level {
                        max_level = level_found;
                    }
                }
                /*
                  - add any leaf node (integer) you found
                    to the level_sum
                  - we can't calculate the final sum at
                    this level yet as we don't know which
                    level we are in until we finished
                    visiting in each node at this level
                */
                NestedInteger::Integer(some_integer) => {
                    level_sum += some_integer;
                }
            }
        }

        /*
          - ok now that we know which level we are
            in it's time to calculate the weighted
            sum for this level and add it to
            the total sum
        */

        *sum = *sum + level_sum * max_level as i32;

        /*
          - report back the level I am in plus one
        */
        max_level + 1
    }

    pub fn text_fixture_1() -> Vec<NestedInteger> {
        //[[1,1],2,[1,1]]
        vec![
            NestedInteger::List(vec![NestedInteger::Integer(1), NestedInteger::Integer(1)]),
            NestedInteger::Integer(2),
            NestedInteger::List(vec![NestedInteger::Integer(1), NestedInteger::Integer(1)]),
        ]
    }

    pub fn text_fixture_2() -> Vec<NestedInteger> {
        //[1,[4,[6]]]
        vec![
            NestedInteger::Integer(1),
            NestedInteger::List(vec![
                NestedInteger::Integer(4),
                NestedInteger::List(vec![NestedInteger::Integer(6)]),
            ]),
        ]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let result = Solution::depth_sum(Solution::text_fixture_1());
        assert_eq!(result, 8);
    }

    #[test]
    fn sample_2() {
        let result = Solution::depth_sum(Solution::text_fixture_2());
        assert_eq!(result, 17);
    }
}
