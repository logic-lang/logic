use std::hash::Hash;

pub trait Type
where
  Self: Clone + Eq + Hash,
{
  type Node;

  fn cost(&self) -> u64;
  fn map(self, f: impl FnMut(Self::Node) -> Self::Node) -> Self;
}
