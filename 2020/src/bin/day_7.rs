use anyhow::Result;

use aoc2020::read;

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;

fn main() -> Result<()> {
    let input = read("./07.input")?;

    println!("part A: {}", day_7_a(&input));
    println!("part B: {}", day_7_b(&input));

    Ok(())
}

fn day_7_a(rules: &str) -> usize {
    let rules = Rules::from(rules);

    rules.count_contains("shiny gold")
}

fn day_7_b(rules: &str) -> usize {
    let rules = Rules::from(rules);

    rules.count_nesting("shiny gold")
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn count_contains(&self, color: &str) -> usize {
        let tree = self.build_contains_tree();

        let mut found = HashSet::new();

        count_direct(&tree, color, &mut found);
        count_indirect(&tree, color, &mut found);

        found.remove(color);

        found.len()
    }

    fn count_nesting(&self, color: &str) -> usize {
        let tree = self.build_nesting_tree();

        bags_inside(&tree, color) - 1
    }

    fn build_contains_tree(&self) -> HashMap<String, Vec<String>> {
        let mut tree: HashMap<String, Vec<String>> = HashMap::new();

        for rule in &self.rules {
            for inside in &rule.inside {
                match tree.get_mut(&inside.color) {
                    Some(bags) => bags.push(rule.bag.clone()),
                    None => {
                        tree.insert(inside.color.clone(), vec![rule.bag.clone()]);
                    }
                }
            }
        }

        tree
    }

    fn build_nesting_tree(&self) -> HashMap<String, Vec<Inside>> {
        let mut tree: HashMap<String, Vec<Inside>> = HashMap::new();

        for rule in &self.rules {
            tree.insert(rule.bag.clone(), rule.inside.clone());
        }

        tree
    }
}

fn bags_inside(tree: &HashMap<String, Vec<Inside>>, color: &str) -> usize {
    let bags = match tree.get(color) {
        Some(bags) => {
            bags.iter()
                .map(|b| b.count * bags_inside(tree, &b.color))
                .sum::<usize>()
                + 1
        }
        None => 0,
    };

    bags
}

fn count_direct(tree: &HashMap<String, Vec<String>>, color: &str, found: &mut HashSet<String>) {
    if let Some(containers) = tree.get(color) {
        for color in containers {
            found.insert(color.to_string());
        }
    }
}

fn count_indirect(tree: &HashMap<String, Vec<String>>, color: &str, found: &mut HashSet<String>) {
    let found = match tree.get(color) {
        Some(containers) => {
            found.insert(color.to_string());

            for color in containers {
                count_indirect(tree, &color, found);
            }
        }
        None => {
            found.insert(color.to_string());
        }
    };

    found
}

impl From<&str> for Rules {
    fn from(rules: &str) -> Self {
        let rules: Vec<Rule> = rules.lines().map(Rule::from).collect();

        Rules { rules }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    bag: String,
    inside: Vec<Inside>,
}

impl From<&str> for Rule {
    fn from(rule: &str) -> Self {
        let split: Vec<&str> = rule.splitn(2, " bags contain ").collect();

        let inside: Vec<Inside> = split[1]
            .split(", ")
            .map(|bag| bag.splitn(2, " bag").next().unwrap())
            .map(Inside::from)
            .collect();

        let bag = split[0].to_string();

        Rule { bag, inside }
    }
}

#[derive(Clone, Debug)]
struct Inside {
    count: usize,
    color: String,
}

impl From<&str> for Inside {
    fn from(bag: &str) -> Self {
        let bag: Vec<&str> = bag.splitn(2, ' ').collect();

        let count = match bag[0] {
            "no" => 0,
            _ => bag[0].parse::<usize>().unwrap(),
        };

        let color = bag[1].to_string();

        Inside { count, color }
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

    #[test]
    fn test_day_7_b() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(32, day_7_b(input));

        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(126, day_7_b(input));
    }
}
