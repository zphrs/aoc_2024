use std::{
    cmp::Ordering,
    collections::{hash_set, HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use anyhow::{anyhow, Error};
use lazy_static::lazy_static;

use crate::page_id::PageId;
use regex::Regex;

pub struct Rules {
    id: PageId,
    should_be_before: HashSet<PageId>,
    should_be_after: HashSet<PageId>,
}

impl PartialEq for Rules {
    fn eq(&self, other: &Self) -> bool {
        !self.should_be_before.contains(&other.id)
            && !self.should_be_after.contains(&other.id)
    }
}

impl PartialOrd for Rules {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut other_is_after = false;
        let mut other_is_before = false;
        if self.should_be_after.contains(&other.id) {
            other_is_after = true;
        }
        if self.should_be_before.contains(&other.id) {
            other_is_before = true;
        }
        if other_is_before && other_is_after {
            return None;
        }
        if other_is_before {
            return Some(Ordering::Less);
        }
        if other_is_after {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Equal);
    }
}

#[derive(Debug)]
pub enum RuleError {
    ShouldBeBefore(PageId, PageId),
    ShouldBeAfter(PageId, PageId),
    Cycle(Vec<PageId>),
}

impl std::error::Error for RuleError {}

impl Display for RuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleError::ShouldBeBefore(id, before) => {
                write!(
                    f,
                    "{} should be before {}, is instead after.",
                    id, before
                )?;
            }
            RuleError::ShouldBeAfter(id, after) => {
                write!(
                    f,
                    "{} should be before {}, is instead after",
                    id, after
                )?;
            }
            RuleError::Cycle(ids) => {
                write!(f, "Cycle detected while ordering ids: ")?;
                let strs: Vec<String> =
                    ids.iter().map(|id| format!("{}", id)).collect();
                write!(f, "{}", strs.join("->"))?;
            }
        }
        Ok(())
    }
}

impl Display for Rules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} should be before {:?} and after {:?}",
            self.id, self.should_be_before, self.should_be_after
        )
    }
}

impl Rules {
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            should_be_before: Default::default(),
            should_be_after: Default::default(),
        }
    }

    pub fn new_with_before(id: PageId, before: PageId) -> Self {
        Self {
            id,
            should_be_after: HashSet::from([before]),
            should_be_before: HashSet::new(),
        }
    }
    pub fn new_with_after(id: PageId, after: PageId) -> Self {
        Self {
            id,
            should_be_before: HashSet::from([after]),
            should_be_after: HashSet::new(),
        }
    }

    pub fn assert_none_are_after(
        &self,
        befores: &[PageId],
    ) -> Result<(), RuleError> {
        // make sure none of the befores are in afters
        for before_id in befores {
            if self.should_be_after.contains(before_id) {
                return Err(RuleError::ShouldBeAfter(self.id, *before_id));
            }
        }
        Ok(())
    }

    pub fn has_befores(&self) -> bool {
        !self.should_be_before.is_empty()
    }

    pub fn has_after(&self, id: PageId) -> bool {
        self.should_be_after.contains(&id)
    }

    pub fn filter_afters(&self, ids: &[PageId]) -> Vec<PageId> {
        ids.iter()
            .filter_map(|id| self.should_be_after.contains(id).then(|| *id))
            .collect()
    }

    pub fn intersection_with_afters<'a, 'b>(
        &'a self,
        ids: &'b HashSet<PageId>,
    ) -> hash_set::Intersection<'b, PageId, std::hash::RandomState>
    where
        'a: 'b,
    {
        self.should_be_after.intersection(ids)
    }

    pub fn assert_none_are_before<'a>(
        &self,
        afters: impl IntoIterator<Item = &'a PageId>,
    ) -> Result<(), RuleError> {
        for after_id in afters {
            if self.should_be_before.contains(after_id) {
                return Err(RuleError::ShouldBeBefore(self.id, *after_id));
            }
        }
        Ok(())
    }
}

pub struct Rule {
    before: PageId,
    after: PageId,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Page {} should be before {}.", self.before, self.after)
    }
}

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"(?<before>[0-9]+)\|(?<after>[0-9]+)").unwrap();
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = RULE_REGEX
            .captures(s)
            .ok_or(anyhow!("Couldn't match rule regex."))?;
        let before = c
            .name("before")
            .expect("Should be before match since regex matched.");
        let after = c
            .name("after")
            .expect("Should be after match since regex matched.");

        Ok(Self {
            before: before.as_str().parse().expect("should parse"),
            after: after.as_str().parse().expect("should parse"),
        })
    }
}

pub struct RulesMap(HashMap<PageId, Rules>);

impl Display for RulesMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in self.0.values() {
            writeln!(f, "{rule}")?;
        }
        Ok(())
    }
}

impl RulesMap {
    pub fn verify_sequence(&self, ids: &[PageId]) -> Result<(), RuleError> {
        for (i, id) in ids.iter().enumerate() {
            let Some(rules) = self.0.get(id) else {
                continue;
            };
            if i < ids.len() - 1 {
                rules.assert_none_are_before(&ids[..i])?;
            }
            if i > 0 {
                rules.assert_none_are_after(&ids[i + 1..])?;
            }
        }
        Ok(())
    }
    pub fn get_rule(&self, id: PageId) -> Option<&Rules> {
        self.0.get(&id)
    }
    // assumes that the subset graph that only includes the ids_left is a DAG
    // "https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm"
    fn walk_graph_inner(
        &self,
        ids_left: &mut HashSet<PageId>,
        mut ids_without_incoming: HashSet<PageId>,
    ) -> Result<Vec<PageId>, RuleError> {
        // L ← Empty list that will contain the sorted elements
        let mut l = Vec::new();
        // S ← Set of all nodes with no incoming edge
        // while S is not empty do
        while !ids_without_incoming.is_empty() {
            let id = ids_without_incoming
                .iter()
                .next()
                .expect(
                    "there's a next because \
                    ids_without_incoming isn't empty.",
                )
                .to_owned();
            //     remove a node n from S
            ids_without_incoming.remove(&id).then_some(()).expect(
                "id should be within set because \
                    we just got the id out of the set.",
            );
            // add n to L
            l.push(id);
            let Some(rule) = self.get_rule(id) else {
                continue;
            };
            //         remove edge e from the graph
            ids_left.remove(&id);
            let outgoing_edges = rule.intersection_with_afters(&ids_left);
            //     for each node m with an edge e from n to m do
            for m in outgoing_edges {
                let Some(rule) = self.get_rule(*m) else {
                    l.push(id);
                    continue;
                };
                //         if m has no other incoming edges then
                if rule.assert_none_are_before(ids_left.iter()).is_ok() {
                    //             insert m into S
                    ids_without_incoming.insert(*m);
                };
            }
        }

        // if graph has edges then
        //     return error   (graph has at least one cycle)
        // else
        //     return L   (a topologically sorted order)
        Ok(l)
    }

    pub fn walk_graph(&self, ids: &[PageId]) -> Result<Vec<PageId>, RuleError> {
        let without_befores = ids.iter().filter(|id| {
            let Some(rule) = self.get_rule(**id) else {
                return true;
            };
            !rule.has_befores() || rule.assert_none_are_before(ids).is_ok()
        });
        let mut ids_left: HashSet<PageId, _> =
            ids.iter().map(|id| *id).collect();
        let ids_without_incoming = without_befores.map(|id| *id).collect();
        self.walk_graph_inner(&mut ids_left, ids_without_incoming)
    }
}

impl FromIterator<String> for RulesMap {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let rules =
            iter.map(|rule| rule.parse::<Rule>().expect("Should parse."));
        let out: Self = Self::from_iter(rules);
        out
    }
}

impl FromIterator<Rule> for RulesMap {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        let mut out: HashMap<PageId, Rules> = HashMap::new();
        let iter = iter.into_iter();
        for rule in iter {
            let Rule { before, after } = rule;
            out.entry(before)
                .and_modify(|before_rules| {
                    before_rules.should_be_before.insert(after);
                })
                .or_insert_with(|| Rules::new_with_after(before, after));
            out.entry(after)
                .and_modify(|after_rules| {
                    after_rules.should_be_after.insert(before);
                })
                .or_insert_with(|| Rules::new_with_before(after, before));
        }
        Self(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::page_ids::PageIds;

    use super::{Rule, RulesMap};

    #[test]
    pub fn rule() {
        let r = "16|98";
        let rule: Rule = r.parse().unwrap();
        println!("{rule}");
        let rules: RulesMap = [rule].into_iter().collect();
        println!("{rules}");
        let ids: PageIds = "16,98".parse().unwrap();
        ids.validate_against_ruleset(&rules).unwrap();
    }
}
