use std::collections::HashMap;

use super::{Decision, Incompatibility, SolveError, Term};
use crate::version::VersionSet;
use mgc_types::{PackageName, Version};

#[derive(Debug, Default)]
pub struct PubGrubSolver {
    incompatibilities: Vec<Incompatibility>,
    assignment: Vec<(PackageName, Version, VersionSet)>,
    decision_level: usize,
    decisions: HashMap<PackageName, Version>,
}

impl PubGrubSolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_incompatibility(&mut self, inc: Incompatibility) {
        self.incompatibilities.push(inc);
    }

    #[allow(clippy::result_large_err)]
    pub fn check_consistency(&self) -> Result<(), Incompatibility> {
        for inc in &self.incompatibilities {
            if inc.terms.iter().all(|term| self.term_satisfied(term)) {
                return Err(inc.clone());
            }
        }
        Ok(())
    }

    fn term_satisfied(&self, term: &Term) -> bool {
        match term {
            Term::Positive(name, vs) => self
                .decisions
                .get(name)
                .map(|v| vs.contains(v))
                .unwrap_or(false),
            Term::Negative(name, vs) => self
                .decisions
                .get(name)
                .map(|v| !vs.contains(v))
                .unwrap_or(true),
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn propagate(&mut self) -> Result<Vec<Decision>, Incompatibility> {
        let mut propagated = Vec::new();
        let mut changed = true;

        while changed {
            changed = false;
            let mut new_decisions = Vec::new();

            for inc in &self.incompatibilities {
                // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
                if let Some(decision) = self.find_unit_propagation(inc)
                    && !propagated.contains(&decision)
                    && !new_decisions.contains(&decision)
                {
                    new_decisions.push(decision.clone());
                    changed = true;
                }
            }

            for decision in new_decisions {
                propagated.push(decision.clone());
                self.apply_decision(&decision);
            }
        }

        for inc in &self.incompatibilities {
            if inc.terms.iter().all(|t| self.term_satisfied(t)) {
                return Err(inc.clone());
            }
        }

        Ok(propagated)
    }

    fn find_unit_propagation(&self, inc: &Incompatibility) -> Option<Decision> {
        let unsatisfied: Vec<_> = inc
            .terms
            .iter()
            .filter(|t| !self.term_satisfied(t))
            .collect();

        if unsatisfied.len() == 1 {
            let term = unsatisfied[0];
            match term {
                Term::Positive(name, vs) => Some(Decision::Assigned(
                    name.clone(),
                    vs.satisfying_version()
                        .unwrap_or_else(|| Version::new(0, 0, 0)),
                    Box::new(vs.clone()),
                )),
                Term::Negative(name, _vs) => Some(Decision::Backtracked(name.clone())),
            }
        } else {
            None
        }
    }

    fn apply_decision(&mut self, decision: &Decision) {
        match decision {
            Decision::Assigned(name, version, vs) => {
                self.decisions.insert(name.clone(), version.clone());
                self.assignment
                    .push((name.clone(), version.clone(), vs.as_ref().clone()));
                self.decision_level += 1;
            }
            Decision::Backtracked(name) => {
                self.decisions.remove(name);
                self.assignment.retain(|(n, _, _)| n != name);
                self.decision_level = self.decision_level.saturating_sub(1);
            }
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn solve(
        &mut self,
        wanted: &[(PackageName, VersionSet)],
    ) -> Result<HashMap<PackageName, Version>, SolveError> {
        for (name, vs) in wanted {
            self.add_incompatibility(Incompatibility::new(vec![Term::Positive(
                name.clone(),
                vs.clone(),
            )]));
        }

        self.propagate()?;

        loop {
            if self.is_complete(wanted) {
                return Ok(self.decisions.clone());
            }

            let decision = self.choose_package(wanted)?;
            self.apply_decision(&decision);

            match self.propagate() {
                Ok(_) => continue,
                Err(conflict) => {
                    if !self.backtrack(&conflict)? {
                        return Err(SolveError::Unsatisfiable(Box::new(conflict)));
                    }
                }
            }
        }
    }

    fn is_complete(&self, wanted: &[(PackageName, VersionSet)]) -> bool {
        wanted
            .iter()
            .all(|(name, _)| self.decisions.contains_key(name))
    }

    #[allow(clippy::result_large_err)]
    fn choose_package(&self, wanted: &[(PackageName, VersionSet)]) -> Result<Decision, SolveError> {
        for (name, vs) in wanted {
            // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
            if !self.decisions.contains_key(name)
                && let Some(version) = vs.satisfying_version()
            {
                return Ok(Decision::Assigned(
                    name.clone(),
                    version,
                    Box::new(vs.clone()),
                ));
            }
        }

        for inc in &self.incompatibilities {
            for term in &inc.terms {
                // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
                if let Term::Positive(name, vs) = term
                    && !self.decisions.contains_key(name)
                    && let Some(version) = vs.satisfying_version()
                {
                    return Ok(Decision::Assigned(
                        name.clone(),
                        version,
                        Box::new(vs.clone()),
                    ));
                }
            }
        }

        Err(SolveError::NoPackageFound)
    }

    #[allow(clippy::result_large_err)]
    fn backtrack(&mut self, _conflict: &Incompatibility) -> Result<bool, SolveError> {
        if let Some(last) = self.assignment.pop() {
            self.decisions.remove(&last.0);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
