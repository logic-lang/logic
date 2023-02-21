/// A typed term-based AST to manipulate.
///
/// [`Type`] is the required trait that enables interactions with the (H)IR.
/// It is based on the definition of [`Type::Term`]s as generic types over node values.
/// A [`Type`] should be implemented on the recursive type that specifies
/// this generic [`Type::Term`] over a `Sized` type such as [`Box`].
pub trait Type: Sized {
  /// The generic term that defines recursive type `Self`.
  type Term<Node>;

  /// Apply a function to each `Node` of a [`Self::Term`].
  fn map<F, T>(term: Self::Term<F>, f: impl FnMut(F) -> T) -> Self::Term<T>;

  /// Converts `Self` type to a term-based (if required).
  fn into_ty(self) -> Self::Term<Self>;
  /// Converts a term-based type to `Self` (if required).
  fn from_ty(ty: Self::Term<Self>) -> Self;

  fn is_lit() -> bool {
    todo!()
  }
}
