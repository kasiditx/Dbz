use super::grid::Nav;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Connection,
    Schema,
    Group,
    Table,
    Column,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub kind: NodeKind,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub expanded: bool,
}

/// Mock Oracle-shaped tree (no database access).
#[derive(Debug, Clone)]
pub struct Explorer {
    pub nodes: Vec<Node>,
    /// Index into the *visible* list.
    pub selected: usize,
}

impl Explorer {
    pub fn mock() -> Self {
        let mut e = Explorer {
            nodes: Vec::new(),
            selected: 0,
        };
        let root = e.add(None, "Oracle DEV (mock)", NodeKind::Connection, true);
        for schema in ["APP", "HR", "SALES_DW"] {
            let s = e.add(Some(root), schema, NodeKind::Schema, false);
            for group in ["Tables", "Views", "Packages", "Sequences"] {
                let g = e.add(Some(s), group, NodeKind::Group, false);
                if group == "Tables" {
                    for t in ["ORDERS", "USERS", "ORDER_ITEMS", "ลูกค้า"] {
                        let tn = e.add(Some(g), t, NodeKind::Table, false);
                        for c in ["ID", "STATUS", "TOTAL", "CREATED_AT"] {
                            e.add(Some(tn), c, NodeKind::Column, false);
                        }
                    }
                }
            }
        }
        e
    }
    fn add(&mut self, parent: Option<usize>, label: &str, kind: NodeKind, expanded: bool) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            label: label.into(),
            kind,
            parent,
            children: Vec::new(),
            expanded,
        });
        if let Some(p) = parent {
            self.nodes[p].children.push(id);
        }
        id
    }
    /// Node indices currently visible (depth-first, collapsed subtrees hidden).
    pub fn visible(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut stack: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.parent.is_none())
            .map(|(i, _)| i)
            .rev()
            .collect();
        while let Some(i) = stack.pop() {
            out.push(i);
            if self.nodes[i].expanded {
                stack.extend(self.nodes[i].children.iter().rev());
            }
        }
        out
    }
    pub fn depth(&self, node: usize) -> usize {
        let mut d = 0;
        let mut cur = node;
        while let Some(p) = self.nodes[cur].parent {
            d += 1;
            cur = p;
        }
        d
    }
    /// Up/Down move; Right expands or enters first child; Left collapses or goes to parent.
    pub fn key(&mut self, nav: Nav) {
        let vis = self.visible();
        if vis.is_empty() {
            return;
        }
        self.selected = self.selected.min(vis.len() - 1);
        let cur = vis[self.selected];
        match nav {
            Nav::Up => self.selected = self.selected.saturating_sub(1),
            Nav::Down => self.selected = (self.selected + 1).min(vis.len() - 1),
            Nav::Right => {
                let n = &self.nodes[cur];
                if n.children.is_empty() {
                } else if !n.expanded {
                    self.nodes[cur].expanded = true;
                } else {
                    self.selected += 1;
                }
            }
            Nav::Left => {
                if self.nodes[cur].expanded {
                    self.nodes[cur].expanded = false;
                } else if let Some(p) = self.nodes[cur].parent {
                    self.selected = vis.iter().position(|&i| i == p).unwrap_or(self.selected);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(e: &Explorer) -> Vec<String> {
        e.visible()
            .into_iter()
            .map(|i| e.nodes[i].label.clone())
            .collect()
    }

    #[test]
    fn starts_with_connection_expanded_and_schemas_collapsed() {
        let e = Explorer::mock();
        let l = labels(&e);
        assert_eq!(l[0], "Oracle DEV (mock)");
        assert!(l.contains(&"APP".to_string()));
        assert!(!l.contains(&"ORDERS".to_string()));
    }

    #[test]
    fn right_expands_and_down_moves() {
        let mut e = Explorer::mock();
        e.key(Nav::Down); // APP
        e.key(Nav::Right); // expand APP
        let l = labels(&e);
        assert!(l.contains(&"Tables".to_string()));
        e.key(Nav::Right); // enter first child
        assert_eq!(e.nodes[e.visible()[e.selected]].label, "Tables");
    }

    #[test]
    fn left_collapses_then_goes_to_parent() {
        let mut e = Explorer::mock();
        e.key(Nav::Down);
        e.key(Nav::Right);
        e.key(Nav::Right); // on Tables
        e.key(Nav::Left); // Tables collapsed already? goes to parent APP
        assert_eq!(e.nodes[e.visible()[e.selected]].label, "APP");
        e.key(Nav::Left); // collapse APP
        assert!(!labels(&e).contains(&"Tables".to_string()));
    }

    #[test]
    fn up_down_clamp() {
        let mut e = Explorer::mock();
        e.key(Nav::Up);
        assert_eq!(e.selected, 0);
        for _ in 0..200 {
            e.key(Nav::Down);
        }
        assert_eq!(e.selected, e.visible().len() - 1);
    }

    #[test]
    fn depth_of_root_is_zero_and_child_is_deeper() {
        let e = Explorer::mock();
        let v = e.visible();
        assert_eq!(e.depth(v[0]), 0);
        assert_eq!(e.depth(v[1]), 1);
    }
}
