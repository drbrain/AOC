use anyhow::Result;

use aoc2020::read;

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;

fn main() -> Result<()> {
    let input = read("./07.input")?;

    println!("part A: {}", day_7_a(&input));
    //println!("part B: {}", day_7_b(&input));

    Ok(())
}

fn day_7_a(rules: &str) -> usize {
    let rules = Rules::from(rules);

    rules.count_contains("shiny gold")
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn count_contains(&self, color: &str) -> usize {
        let tree = self.build_tree();

        let mut found = HashSet::new();

        count_direct(&tree, color, &mut found);
        count_indirect(&tree, color, &mut found);

        found.remove(color);

        found.len()
    }

    fn build_tree(&self) -> HashMap<String, Vec<String>> {
        let mut tree: HashMap<String, Vec<String>> = HashMap::new();

        for rule in &self.rules {
            for inside in &rule.inside {
                match tree.get_mut(&inside.bag) {
                    Some(bags) => bags.push(rule.bag.clone()),
                    None => {
                        tree.insert(inside.bag.clone(), vec![rule.bag.clone()]);
                    }
                }
            }
        }

        tree
    }
}

fn count_direct(tree: &HashMap<String, Vec<String>>, color: &str, found: &mut HashSet<String>) {
    match tree.get(color) {
        Some(containers) => {
            for color in containers {
                found.insert(color.to_string());
            }
        },
        None => (),
    }
}

fn count_indirect(tree: &HashMap<String, Vec<String>>, color: &str, found: &mut HashSet<String>) {
    let found = match tree.get(color) {
        Some(containers) => {
            found.insert(color.to_string());

            for color in containers {
                count_indirect(tree, &color, found);
            }
        },
        None => {
            found.insert(color.to_string());
        },
    };

    found
}

impl From<&str> for Rules {
    fn from(rules: &str) -> Self {
        let rules: Vec<Rule> = rules.lines().map(|rule| Rule::from(rule)).collect();

        Rules { rules }
    }
}

#[derive(Debug)]
struct Rule {
    bag: String,
    inside: Vec<Inside>,
}

impl From<&str> for Rule {
    fn from(rule: &str) -> Self {
        let split: Vec<&str> = rule.splitn(2, " bags contain ").collect();

        let inside: Vec<Inside> = split[1]
            .split(", ")
            .map(|bag| bag.splitn(2, " bag").nth(0).unwrap())
            .map(|bag| Inside::from(bag))
            .collect();

        let bag = split[0].to_string();

        Rule { bag, inside }
    }
}

#[derive(Debug)]
struct Inside {
    count: usize,
    bag: String,
}

impl From<&str> for Inside {
    fn from(bag: &str) -> Self {
        let bag: Vec<&str> = bag.splitn(2, " ").collect();

        let count = match bag[0] {
            "no" => 0,
            _ => bag[0].parse::<usize>().unwrap(),
        };

        let bag = bag[1].to_string();

        Inside { count, bag }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_7_a() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(4, day_7_a(input));
    }

    #[test]
    fn test_day_7_count_contains() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let rules = Rules::from(input);

        assert_eq!(4, rules.count_contains("shiny gold"));
        assert_eq!(7, rules.count_contains("dotted black"));
        assert_eq!(2, rules.count_contains("muted yellow"));
        assert_eq!(7, rules.count_contains("faded blue"));
        assert_eq!(5, rules.count_contains("vibrant plum"));
        assert_eq!(5, rules.count_contains("dark olive"));
        assert_eq!(0, rules.count_contains("dark orange"));
        assert_eq!(0, rules.count_contains("light red"));
    }
}
