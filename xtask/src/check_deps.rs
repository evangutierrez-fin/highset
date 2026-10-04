//! `xtask check-deps`: fail on any internal dependency edge that `docs/ARCHITECTURE.md` §2
//! does not allow.
//!
//! The rules live in [`RULES`]. To change them, change ARCHITECTURE.md first (an ADR may be
//! needed), then this table, in the same commit.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserialize;

/// The crate every test helper lives in. Other crates may use it only as a dev-dependency.
const TESTKIT: &str = "highset-testkit";

/// Which internal crates a workspace member may depend on.
#[derive(Debug, Clone, Copy)]
enum Allowed {
    /// Exactly these crates (normal and build dependencies).
    Only(&'static [&'static str]),
    /// Any workspace crate. Only for dev-only helpers such as `highset-testkit`.
    Any,
}

/// Allowed internal edges, from ARCHITECTURE.md §2 "Dependency rules".
///
/// `highset-core` is the shared contract, so every crate except `highset-i18n` and `xtask`
/// may depend on it directly.
const RULES: &[(&str, Allowed)] = &[
    ("highset-core", Allowed::Only(&[])),
    ("highset-protocol", Allowed::Only(&["highset-core"])),
    ("highset-store", Allowed::Only(&["highset-core"])),
    ("highset-search", Allowed::Only(&["highset-core"])),
    ("highset-git", Allowed::Only(&["highset-core"])),
    ("highset-secrets", Allowed::Only(&["highset-core"])),
    ("highset-i18n", Allowed::Only(&[])),
    (
        "highset-llm",
        Allowed::Only(&["highset-core", "highset-secrets"]),
    ),
    (
        "highset-agents",
        Allowed::Only(&["highset-core", "highset-git"]),
    ),
    (
        "highset-context",
        Allowed::Only(&["highset-core", "highset-store", "highset-secrets"]),
    ),
    (
        "highset-method",
        Allowed::Only(&["highset-core", "highset-store", "highset-git"]),
    ),
    (
        "highset-plugins",
        Allowed::Only(&["highset-core", "highset-store"]),
    ),
    (
        "highset-mcp",
        Allowed::Only(&["highset-core", "highset-protocol"]),
    ),
    (
        "highset-daemon",
        Allowed::Only(&[
            "highset-core",
            "highset-protocol",
            "highset-store",
            "highset-search",
            "highset-git",
            "highset-agents",
            "highset-context",
            "highset-llm",
            "highset-method",
            "highset-plugins",
            "highset-secrets",
        ]),
    ),
    (
        "highset-tui",
        Allowed::Only(&["highset-core", "highset-protocol", "highset-i18n"]),
    ),
    (
        "highset-cli",
        Allowed::Only(&[
            "highset-core",
            "highset-protocol",
            "highset-tui",
            "highset-daemon",
            "highset-mcp",
            "highset-i18n",
        ]),
    ),
    (TESTKIT, Allowed::Any),
    ("fake-agent", Allowed::Only(&["highset-core"])),
    ("xtask", Allowed::Only(&[])),
];

/// The subset of `cargo metadata --format-version 1 --no-deps` that the check reads.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    name: String,
    /// `None` for normal dependencies, `Some("dev")` or `Some("build")` otherwise.
    kind: Option<String>,
}

/// One forbidden dependency edge.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Violation {
    /// The crate that declares the dependency.
    pub from: String,
    /// The internal crate it depends on (or the crate itself when it has no rule).
    pub to: String,
    /// `normal`, `dev` or `build`.
    pub kind: String,
    /// Why the edge is forbidden.
    pub reason: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} ({}): {}",
            self.from, self.to, self.kind, self.reason
        )
    }
}

/// Checks every internal edge in `metadata` against [`RULES`].
pub fn check(metadata: &Metadata) -> Vec<Violation> {
    let rules: BTreeMap<&str, Allowed> = RULES.iter().copied().collect();
    let members: BTreeSet<&str> = metadata.packages.iter().map(|p| p.name.as_str()).collect();
    let mut violations = Vec::new();

    for package in &metadata.packages {
        let Some(allowed) = rules.get(package.name.as_str()) else {
            violations.push(Violation {
                from: package.name.clone(),
                to: package.name.clone(),
                kind: "-".to_owned(),
                reason: "workspace crate has no dependency rule; add it to ARCHITECTURE.md §2 \
                         and xtask/src/check_deps.rs"
                    .to_owned(),
            });
            continue;
        };

        for dep in &package.dependencies {
            if !members.contains(dep.name.as_str()) {
                continue; // third-party crate
            }
            let kind = dep.kind.as_deref().unwrap_or("normal");
            if let Some(reason) = edge_error(&package.name, *allowed, &dep.name, kind) {
                violations.push(Violation {
                    from: package.name.clone(),
                    to: dep.name.clone(),
                    kind: kind.to_owned(),
                    reason,
                });
            }
        }
    }

    violations.sort();
    violations
}

/// Returns why the edge `from -> to` of the given kind is forbidden, or `None` if it is allowed.
fn edge_error(from: &str, allowed: Allowed, to: &str, kind: &str) -> Option<String> {
    if to == TESTKIT {
        return (kind != "dev" && from != TESTKIT)
            .then(|| format!("{TESTKIT} may only be used as a dev-dependency"));
    }
    match allowed {
        Allowed::Any => None,
        Allowed::Only(list) if list.contains(&to) => None,
        Allowed::Only(list) => Some(format!(
            "not allowed by ARCHITECTURE.md §2 (allowed: {})",
            if list.is_empty() {
                "none".to_owned()
            } else {
                list.join(", ")
            }
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(dependency name, kind)`; kind is `None` for a normal dependency.
    type Dep<'a> = (&'a str, Option<&'a str>);

    fn metadata(edges: &[(&str, &[Dep<'_>])]) -> Metadata {
        let packages = edges
            .iter()
            .map(|(name, deps)| {
                let deps = deps
                    .iter()
                    .map(|(dep, kind)| serde_json::json!({ "name": dep, "kind": kind, "req": "*" }))
                    .collect::<Vec<_>>();
                serde_json::json!({ "name": name, "version": "0.1.0", "dependencies": deps })
            })
            .collect::<Vec<_>>();
        serde_json::from_value(serde_json::json!({ "packages": packages, "workspace_members": [] }))
            .unwrap()
    }

    #[test]
    fn allowed_graph_passes() {
        let m = metadata(&[
            ("highset-core", &[("serde", None)]),
            (
                "highset-protocol",
                &[("highset-core", None), ("tokio", None)],
            ),
            (
                "highset-tui",
                &[("highset-protocol", None), ("highset-i18n", None)],
            ),
            ("highset-i18n", &[]),
        ]);
        assert_eq!(check(&m), vec![]);
    }

    #[test]
    fn forbidden_edge_fails() {
        // A client must never reach a service crate directly.
        let m = metadata(&[
            ("highset-tui", &[("highset-store", None)]),
            ("highset-store", &[]),
        ]);
        let v = check(&m);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].from, "highset-tui");
        assert_eq!(v[0].to, "highset-store");
        assert_eq!(v[0].kind, "normal");
    }

    #[test]
    fn core_cannot_depend_on_internal_crates() {
        let m = metadata(&[
            ("highset-core", &[("highset-protocol", None)]),
            ("highset-protocol", &[("highset-core", None)]),
        ]);
        let v = check(&m);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].from, "highset-core");
    }

    #[test]
    fn sideways_service_edges_fail_even_as_build_dependencies() {
        let m = metadata(&[
            ("highset-agents", &[("highset-context", Some("build"))]),
            ("highset-context", &[]),
        ]);
        let v = check(&m);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].kind, "build");
    }

    #[test]
    fn testkit_only_as_dev_dependency() {
        let m = metadata(&[
            ("highset-store", &[("highset-testkit", Some("dev"))]),
            ("highset-daemon", &[("highset-testkit", None)]),
            (
                "highset-testkit",
                &[("highset-daemon", None), ("highset-store", None)],
            ),
        ]);
        let v = check(&m);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].from, "highset-daemon");
        assert_eq!(v[0].to, "highset-testkit");
    }

    #[test]
    fn dev_dependencies_follow_the_same_rules() {
        let m = metadata(&[
            ("highset-tui", &[("highset-daemon", Some("dev"))]),
            ("highset-daemon", &[]),
        ]);
        assert_eq!(check(&m).len(), 1);
    }

    #[test]
    fn unknown_member_fails() {
        let m = metadata(&[("highset-new-thing", &[])]);
        let v = check(&m);
        assert_eq!(v.len(), 1);
        assert!(v[0].reason.contains("no dependency rule"));
    }

    #[test]
    fn every_rule_names_a_known_crate() {
        let names: BTreeSet<&str> = RULES.iter().map(|(n, _)| *n).collect();
        for (_, allowed) in RULES {
            if let Allowed::Only(list) = allowed {
                for dep in *list {
                    assert!(names.contains(dep), "rule references unknown crate {dep}");
                }
            }
        }
    }
}
