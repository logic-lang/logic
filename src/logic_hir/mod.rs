use crate::ty::Type;

use rustc_hash::FxHashMap;
use std::fmt;
use std::hash::Hash;
use std::mem;

/// A unique high-level-representation Id to an expression `Type`.
#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct HirId(pub usize);

/// A class of equivalent `Type` expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirClass {
  pub nodes: Vec<HirId>,
}

/// A context maintaining equivalence between `Type` expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirCtx<T>
where
  T: Type,
{
  /// A map indexed on an IR `Type` arena.
  pub ty_map: Vec<T>,
  /// A map from IR `Type` to unique `HirId`.
  pub id_map: FxHashMap<T, HirId>,
  /// A quotient space of equivalence classes.
  pub qspace: FxHashMap<HirId, HirClass>,
  /// A union-find equivalence structure.
  pub parent: Vec<HirId>,
}

impl<T> HirCtx<T>
where
  T: Type<Node = HirId>,
{
  /// Constructs a new, empty `HirCtx<T>`.
  pub fn new() -> Self {
    Self {
      ty_map: Vec::new(),
      id_map: FxHashMap::default(),
      qspace: FxHashMap::default(),
      parent: Vec::new(),
    }
  }

  /// Check if two expressions are equivalent.
  pub fn is_equivalent(
    // class(lhs) = class(rhs)
    &mut self,
    lhs: T,
    rhs: T,
  ) -> bool {
    let lhs_id = self.insert(lhs);
    let rhs_id = self.insert(rhs);
    self.find(lhs_id) == self.find(rhs_id)
  }

  /// Find the root (minimal) equivalent expression.
  pub fn find(
    // cur_id < n
    &mut self,
    cur_id: HirId,
  ) -> HirId {
    assert!(cur_id.0 < self.parent.len());
    if cur_id != self.parent[cur_id.0] {
      let path_id = self.parent[cur_id.0];
      self.parent[cur_id.0] = self.parent[path_id.0];
      self.find(path_id)
    } else {
      cur_id
    }
  }

  pub fn search(
    // cur_id < n
    &self,
    mut cur_id: HirId,
  ) -> HirId {
    assert!(cur_id.0 < self.parent.len());
    while cur_id != self.parent[cur_id.0] {
      // cur_id = parent
      // min_{cur_id}
      cur_id = self.parent[cur_id.0];
    }

    cur_id
  }

  /// Insert an expression and return its unique Id.
  pub fn insert(
    // expr ∈ graph: id
    // expr ∉ graph: n + 1
    &mut self,
    expr: T,
  ) -> HirId {
    let expr = self.canonalize(expr);
    if let Some(cur_id) = self.id_map.get(&expr).cloned() {
      self.find(cur_id)
    } else {
      self.new_id(expr)
    }
  }

  fn new_id(
    // expr ∉ graph
    &mut self,
    expr: T,
  ) -> HirId {
    let next_id = HirId(self.ty_map.len());
    // ty_map = { 0 -> x_0, ... , n -> x_n, n + 1 -> x_ }
    self.ty_map.push(expr.clone());
    // id_map = { x_0 -> 0, ... , x_n -> n, x_ -> n + 1 }
    self.id_map.insert(expr, next_id);
    // qspace = { 0 ⊃ { 0, ..., n } }
    self.qspace.insert(next_id, HirClass { nodes: vec![next_id] });
    // parent(n + 1) = n + 1
    self.parent.push(next_id);
    next_id
  }

  /// Merge two expressions by their union.
  pub fn union(
    // lhs_id < n
    // rhs_id < n
    &mut self,
    lhs_id: HirId,
    rhs_id: HirId,
    cong: bool,
  ) -> Option<HirId> {
    assert!(lhs_id.0 < self.parent.len());
    assert!(rhs_id.0 < self.parent.len());

    let mut lhs_id = self.find(lhs_id);
    let mut rhs_id = self.find(rhs_id);

    if lhs_id != rhs_id {
      let lhs_len = self.qspace[&lhs_id].len();
      let rhs_len = self.qspace[&rhs_id].len();
      // min_{lhs, rhs} len
      if !cong && lhs_len < rhs_len {
        mem::swap(&mut lhs_id, &mut rhs_id);
      }

      // lhs (optimal)
      // = parent
      // = rhs
      self.parent[rhs_id.0] = lhs_id;
      if let Some((mut rhs_class, lhs_class)) = self.qspace.remove(&rhs_id).zip(self.qspace.get_mut(&lhs_id)) {
        lhs_class.nodes.append(
          // class(rhs) ⊆ class(lhs)
          &mut rhs_class.nodes,
        )
      }
      Some(lhs_id)
    } else {
      None
    }
  }

  /// Propagate congruence relations.
  pub fn rebuild(
    // ∀ expr: id
    &mut self,
  ) {
    loop {
      let mut cong = true;
      for (expr, id) in self.id_map.clone().into_iter() {
        let cexpr = self.canonalize(expr);
        if let Some(cid) = self.id_map.get(&cexpr) {
          if self
            .union(
              // id = expr ∈ graph
              *cid, id, true,
            )
            .is_some()
          {
            cong = false;
          }
        } else {
          self.id_map.insert(cexpr.clone(), id);
          self.id_map.remove(&std::mem::replace(
            // id = cong(expr)
            &mut self.ty_map[id.0],
            cexpr,
          ));
        }
      }

      assert_eq!(
        // bidir invariant
        self.id_map.len(),
        self.ty_map.len(),
      );
      if cong {
        break;
      }
    }
  }

  fn canonalize(
    // cur_id < n
    &mut self,
    expr: T,
  ) -> T {
    expr.map(|node| self.find(node))
  }
}

impl<T> Default for HirCtx<T>
where
  T: Type<Node = HirId>,
{
  /// Creates an empty `HirCtx<T>`.
  fn default() -> Self {
    Self::new()
  }
}

impl fmt::Debug for HirId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "`{}`", self.0)
  }
}

impl HirClass {
  fn len(&self) -> usize {
    self.nodes.len()
  }
}

impl<H: Type<Node = HirId> + fmt::Debug> fmt::Display for HirCtx<H> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "hir")?;
    for (parent, class) in &self.qspace {
      writeln!(f, "├─ ⊆ {{{parent:?}}}: {:?}", self.ty_map[parent.0])?;
      for equiv in &class.nodes {
        if equiv != parent {
          writeln!(f, "│  ├─ {equiv:?}: {:?}", self.ty_map[equiv.0])?;
        }
      }
    }

    write!(
      // stats
      f,
      "\n\
      {} expressions \n\
      {} classes     \n",
      self.id_map.len(),
      self.qspace.len()
    )?;
    Ok(())
  }
}
