use std::cmp::Ordering;

struct Rule {
    before: u32,
    after: u32,
}
impl Rule {
    fn from_string(raw_rule: &str) -> Rule {
        let (before, after) = raw_rule.split_once("|").expect("Invalid Syntax, rules");

        Rule {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        }
    }

    fn check(&self, manual: &Manual) -> bool {
        !manual.contains(vec![self.before, self.after])
            || manual.position(&self.before).unwrap() < manual.position(&self.after).unwrap()
    }

    fn is_relevant(&self, numbers: &Vec<u32>) -> bool {
        numbers.contains(&self.before) && numbers.contains(&self.after)
    }

    fn apply(&self, a: u32, b:u32) -> Ordering {
        let less = (self.before, self.after);
        let greater = (self.after,self.before);
        if (a,b) == less {
            Ordering::Less
        } else if (a,b) == greater {
            Ordering::Greater
        } else {
            unreachable!()
        }
    }
}

struct Manual {
    pages: Vec<u32>,
}
impl Manual {
    fn from_string(raw_manual: &str) -> Manual {
        Manual {
            pages: raw_manual.split(",").map(|e| e.parse().unwrap()).collect(),
        }
    }
    fn conforms_to(&self, rules: &Vec<Rule>) -> bool {
        rules.iter().all(|rule| rule.check(&self))
    }
    fn contains(&self, numbers: Vec<u32>) -> bool {
        numbers.iter().all(|number| self.pages.contains(number))
    }
    fn position(&self, input: &u32) -> Option<usize> {
        self.pages.iter().position(|num| num == input)
    }
    fn middle(&self) -> Option<u32> {
        let len = self.pages.len();
        if len == 0 {
            return None;
        }
        let position =len/2;
        Some(self.pages[position])
    }
    // cardinal of functional mockery, in place revision
    // jk lmao copy time
    fn sort(&self, rules: &Vec<Rule>) -> Manual{
        let mut output = self.pages.clone();
        output.sort_by(|left,right| sort_by_relevant_rule(*left, *right, rules));
        Manual {pages: output}
    }
}

fn sort_by_relevant_rule(left:u32,right:u32,rules:&Vec<Rule>) -> Ordering {
    let rule: Vec<&Rule> = rules.iter().filter(|rule|rule.is_relevant(&vec![left,right])).collect();
    if rule.is_empty(){
        return Ordering::Equal;
    }

    rule[0].apply(left,right)
}

fn parse(input: String) -> (Vec<Rule>, Vec<Manual>) {
    let (raw_rules, raw_manuals) = input
        .split_once("\n\n")
        .expect("Invalid Syntax(Split Rules and Manuals)");

    let rules = raw_rules
        .lines()
        .map(|raw_rule| Rule::from_string(raw_rule))
        .collect();
    let manuals = raw_manuals
        .lines()
        .map(|raw_manual| Manual::from_string(raw_manual))
        .collect();
    (rules, manuals)
}

pub mod p1 {
    pub fn run(_input: String) -> u32 {
        let (rules, manuals) = super::parse(_input);
        manuals
            .iter()
            .filter(|&manual| manual.conforms_to(&rules))
            .filter_map(|manual| manual.middle())
            .sum()
    }
}

pub mod p2 {
    pub fn run(_input: String) -> u32 {
        let (rules, manuals) = super::parse(_input);
        manuals
            .iter()
            .filter(|&manual| !manual.conforms_to(&rules))
            .map(|e| {
                e.sort(&rules)
            })
            .filter_map(|manual| manual.middle())
            .sum()
    }
}
