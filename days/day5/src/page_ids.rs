use std::{fmt::Display, str::FromStr};

use crate::{
    page_id::PageId,
    rules::{RuleError, RulesMap},
};

pub struct PageIds(Vec<PageId>);

impl Into<Vec<PageId>> for PageIds {
    fn into(self) -> Vec<PageId> {
        self.0
    }
}

impl PageIds {
    pub fn into_inner(self) -> Vec<PageId> {
        self.into()
    }

    pub fn from_inner(ids: Vec<PageId>) -> Self {
        Self(ids)
    }
}

impl Display for PageIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strs: Vec<String> =
            self.0.iter().map(|id| format!("{}", id)).collect();
        write!(f, "{}", strs.join(","))
    }
}

impl FromStr for PageIds {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split(",");
        let mut inner: Vec<PageId> = Vec::new();
        for num in nums {
            inner.push(num.parse()?);
        }
        Ok(Self(inner))
    }
}

impl PageIds {
    pub fn validate_against_ruleset(
        &self,
        ruleset: &RulesMap,
    ) -> Result<(), RuleError> {
        ruleset.verify_sequence(&self.0)
    }

    pub fn middle_id(&self) -> PageId {
        *self
            .0
            .get(&self.0.len() / 2)
            .expect("length of page_ids should not be zero.")
    }
}
