//! Equivalence graph and expression Ids.

use crate::ty::Type;

use rustc_hash::FxHashMap;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::ops::Index;

/// A unique high-level-representation Id to an expression [`Type`].
#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct HirId(usize);

/// A class of equivalent [`Type`] expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirClass {
  nodes: Vec<HirId>,
}

/// A graph maintaining equivalence between [`Type`] expressions.
#[derive(Default, Debug, Clone)]
pub struct Graph<T, K> {
  /// A map indexed on an IR `Type` arena.
  ty_map: Vec<K>,
  /// A map from IR `Type` to unique `HirId`.
  id_map: FxHashMap<K, HirId>,
  /// A temporary placeholder.
  holder: PhantomData<T>,
  /// A quotient space of equivalence classes.
  qspace: FxHashMap<HirId, HirClass>,
  /// A union-find equivalence structure.
  parent: Vec<HirId>,
}

impl<T, K> Graph<T, K>
where
  T: Type<Term<HirId> = K>,
  K: Clone + Eq + Hash,
{
  /// Constructs a new, empty [`Graph`].
  pub fn new() -> Self {
    Self {
      ty_map: Vec::new(),
      id_map: FxHashMap::default(),
      holder: PhantomData,
      qspace: FxHashMap::default(),
      parent: Vec::new(),
    }
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

  pub(crate) fn _search(
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
    // ∀ ty: id
    &mut self,
  ) {
    loop {
      let mut cong = true;
      for (expr, id) in self.id_map.clone().into_iter() {
        let cexpr = self.canonalize(expr);
        if let Some(cid) = self.id_map.get(&cexpr) {
          if self
            .union(
              // id = ty ∈ graph
              *cid, id, true,
            )
            .is_some()
          {
            cong = false;
          }
        } else {
          self.id_map.insert(cexpr.clone(), id);
          self.id_map.remove(&std::mem::replace(
            // id = cong(ty)
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

  /// Instantiate an expression from IR.
  pub fn inst(
    // cur_id < n
    &self,
    cur_id: HirId,
  ) -> T {
    T::from_ty(T::map(self[cur_id].clone(), |sub| self.inst(sub)))
  }

  /// Lower an expression into IR.
  pub fn lower(
    // ∀ expr: id
    &mut self,
    expr: T,
  ) -> HirId {
    let hir_ty = T::map(expr.into_ty(), |sub| self.lower(sub));
    self.insert(
      hir_ty, // ty ∈ graph
    )
  }

  /// Insert an expression and return its unique Id.
  pub(crate) fn insert(
    // ty ∈ graph: id
    // ty ∉ graph: n + 1
    &mut self,
    ty: K,
  ) -> HirId {
    let ty = self.canonalize(ty);
    if let Some(cur_id) = self.id_map.get(&ty).cloned() {
      self.find(cur_id)
    } else {
      self.new_id(ty)
    }
  }

  fn new_id(
    // ty ∉ graph
    &mut self,
    ty: K,
  ) -> HirId {
    let next_id = HirId(self.ty_map.len());
    // ty_map = { 0 -> x_0, ... , n -> x_n, n + 1 -> x_ }
    self.ty_map.push(ty.clone());
    // id_map = { x_0 -> 0, ... , x_n -> n, x_ -> n + 1 }
    self.id_map.insert(ty, next_id);
    // qspace = { 0 ⊃ { 0, ..., n } }
    self.qspace.insert(next_id, HirClass { nodes: vec![next_id] });
    // parent(n + 1) = n + 1
    self.parent.push(next_id);
    next_id
  }

  fn canonalize(
    // parent(ty) ∈ graph
    &mut self,
    ty: K,
  ) -> K {
    T::map(ty, |node| self.find(node))
  }
}

impl<T, K> Index<HirId> for Graph<T, K> {
  /// Index on a [`HirId`] type.
  type Output = K;

  fn index(&self, id: HirId) -> &Self::Output {
    &self.ty_map[id.0]
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

impl<T, K> fmt::Display for Graph<T, K>
where
  K: fmt::Debug,
{
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

    // stats
    write!(
      f,
      "\
      {} expressions \n\
      {} classes     \n",
      self.id_map.len(),
      self.qspace.len()
    )?;
    Ok(())
  }
}
