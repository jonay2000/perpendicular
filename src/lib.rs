///
/// [![Docs.rs](https://img.shields.io/badge/docs.rs-perpendicular-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0
/// JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/perpendicular)
/// [![Crates.io](https://img.shields.io/crates/v/perpendicular?logo=rust&style=for-the-badge)](https://crates.io/crates/perpendicular)
/// [![Github Workflows](https://img.shields.io/github/workflow/status/jonay2000/perpendicular/label?logo=github&style=for-the-badge)](https://github.com/jonay2000/perpendicular/actions/workflows/ci.yml)
/// Perpendicular is a simple general purpose n-dimensional vector library.
///
/// This is not a general purpose linear algebra library. Instead, it's designed
/// as a tool for simple physics simulations which just need to store some coordinates
/// or velocities together.
///
/// All library documentation can be found on the [`Vector`] struct.
///
/// ```rust
/// use perpendicular::Vector;
///
/// let v1 = Vector::new((1, 2, 3));
/// let v2 = Vector::new((3, 4, 5));
/// let v3 = Vector::new((5, 6, 7));
///
/// println!("{}", v1 + v2 * v3);
///
/// ```
use array_init::from_iter;
use core::fmt;
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Rem, Sub};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use num::traits::Pow;
use std::iter::Sum;

macro_rules! same_length {
    () => {
        "The type system ensures that this value is the right length."
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
pub struct Vector<T, const DIM: usize> {
    values: [T; DIM],
}

/// Type alias for 2d vector
pub type Vector2<T> = Vector<T, 2>;
/// Type alias for 3d vector
pub type Vector3<T> = Vector<T, 3>;
/// Type alias for 4d vector
pub type Vector4<T> = Vector<T, 4>;

impl<T> Vector4<T> {
    /// Create a new 4D vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    /// let v = Vector::new4(1, 2, 3, 4);
    ///
    /// assert_eq!(v.dimensions(), 4);
    /// ```
    pub const fn new4(x: T, y: T, z: T, w: T) -> Self {
        Self::new_from_arr([x, y, z, w])
    }
}

impl<T> Vector3<T> {
    /// Create a new 2D vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    /// let v = Vector::new3(1, 2, 3);
    ///
    /// assert_eq!(v.dimensions(), 3);
    /// ```
    pub const fn new3(x: T, y: T, z: T) -> Self {
        Self::new_from_arr([x, y, z])
    }
}

impl<T> Vector2<T> {
    /// Create a new 2D vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    /// let v = Vector::new2(1, 2);
    ///
    /// assert_eq!(v.dimensions(), 2);
    /// ```
    pub const fn new2(x: T, y: T) -> Self {
        Self::new_from_arr([x, y])
    }
}

impl<T, const DIM: usize> Vector<T, DIM> {
    /// Create a new Vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v = Vector::new([1, 2]);
    /// ```
    pub fn new(value: impl Into<Vector<T, DIM>>) -> Self {
        value.into()
    }

    pub fn repeat(value: T) -> Self
    where
        T: Clone,
    {
        Self::try_new(core::iter::repeat(value).take(DIM)).expect(same_length!())
    }

    /// Try to create a vector from the elements provided (in the form of any
    /// type which implements [`IntoIterator`]). Returns None when the number of
    /// items in the iterator do no much the dimension of the desired vector.
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// assert_eq!(Vector::try_new(vec![1, 2]), Some(Vector::new2(1, 2)));
    /// assert_eq!(Vector::try_new(vec![1, 2, 3]), Option::<Vector<_, 2>>::None);
    /// assert_eq!(Vector::try_new(vec![1]), Option::<Vector<_, 2>>::None);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn try_new(i: impl IntoIterator<Item = T>) -> Option<Self> {
        let i = i.into_iter();
        match i.size_hint() {
            (lower, _) if lower < DIM => None,
            (_, Some(upper)) if upper > DIM => None,
            (lower, Some(upper)) if lower == upper && lower != DIM => None,
            _ => {
                let collected: Vec<_> = i.collect();
                if collected.len() != DIM {
                    return None;
                }

                Some(Self::new_from_arr(from_iter(collected)?))
            }
        }
    }

    /// Like [`try_new`], but the iterator provided may be longer than the desired
    /// vector (extra elements are consumed).
    /// However, it may not be shorter then the desired vector.
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// assert_eq!(Vector::try_new_overflow(vec![1, 2]), Some(Vector::new2(1, 2)));
    /// assert_eq!(Vector::try_new_overflow(vec![1, 2, 3]), Some(Vector::new2(1, 2)));
    /// assert_eq!(Vector::try_new_overflow(vec![1, 2, 3]), Some(Vector::new3(1, 2, 3)));
    /// assert_eq!(Vector::try_new_overflow(vec![1]), Option::<Vector<_, 2>>::None);
    /// ```
    pub fn try_new_overflow(i: impl IntoIterator<Item = T>) -> Option<Self> {
        Some(Self::new_from_arr(from_iter(i.into_iter().take(DIM))?))
    }

    #[doc(hidden)]
    const fn new_from_arr(values: [T; DIM]) -> Self {
        Self { values }
    }

    /// Get the number of dimensions this vector has
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    /// let v = Vector::new([1, 2, 3, 4]);
    ///
    /// assert_eq!(v.dimensions(), 4);
    /// ```
    pub fn dimensions(&self) -> usize {
        DIM
    }

    /// get a reference to the nth item in the vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let mut v = Vector::new2(1, 2);
    /// assert_eq!(v.get(0), Some(&1));
    /// assert_eq!(v.get(1), Some(&2));
    /// assert_eq!(v.get(2), None);
    /// ```
    pub fn get(&self, n: usize) -> Option<&T> {
        self.values.get(n)
    }

    /// get a mutable reference to the nth item in the vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let mut v = Vector::new2(1, 2);
    /// assert_eq!(v.get_mut(0), Some(&mut 1));
    /// assert_eq!(v.get_mut(1), Some(&mut 2));
    /// assert_eq!(v.get_mut(2), None);
    /// ```
    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        self.values.get_mut(n)
    }

    /// Create an iterator over references to items in the vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v = Vector::new2(1, 2);
    /// let mut i = v.iter();
    /// assert_eq!(i.next(), Some(&1));
    /// assert_eq!(i.next(), Some(&2));
    /// assert_eq!(i.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    /// Create an iterator over mutable references to items in the vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let mut v = Vector::new2(1, 2);
    /// let mut i = v.iter_mut();
    /// assert_eq!(i.next(), Some(&mut 1));
    /// assert_eq!(i.next(), Some(&mut 2));
    /// assert_eq!(i.next(), None);
    /// ```
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let mut v = Vector::new2(1, 2);
    /// {
    ///     let mut i = v.iter_mut();
    ///     *i.next().unwrap() = 4;
    /// }
    /// assert_eq!(v.get(0), Some(&4))
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    /// Scale a vector by a scalar, multiplying each element
    /// by n.
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v = Vector::new([1, 2, 3]);
    ///
    /// assert_eq!(v.scale(2), Vector::new([2, 4, 6]));
    /// assert_eq!(v.scale(3), Vector::new([3, 6, 9]));
    ///
    /// ```
    pub fn scale<'a, U>(&'a self, n: U) -> Vector<<&'a T as Mul<U>>::Output, DIM>
    where
        &'a T: Mul<U>,
        U: Clone,
    {
        Vector::new_from_arr(from_iter(self.iter().map(|x| x * n.clone())).expect(same_length!()))
    }

    /// Unscale a vector by a scalar. This divides every element by n.
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v = Vector::new([4, 8, 16]);
    ///
    /// assert_eq!(v.unscale(2), Vector::new([2, 4, 8]));
    /// assert_eq!(v.unscale(4), Vector::new([1, 2, 4]));
    ///
    /// ```
    pub fn unscale<'a, U>(&'a self, other: U) -> Vector<<&'a T as Div<U>>::Output, DIM>
    where
        &'a T: Div<U>,
        U: Clone,
    {
        Vector::new_from_arr(
            from_iter(self.iter().map(|x| x / other.clone())).expect(same_length!()),
        )
    }

    /// Create a vector filled with the zero value of T (according to num)
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v = Vector::new((0, 0, 0));
    /// assert_eq!(Vector::zero(), v);
    /// ```
    pub fn zero() -> Self
    where
        T: num::Num + Clone,
    {
        Self::repeat(T::zero())
    }

    /// Create a vector filled with the one value of T (according to num)
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v = Vector::new((1, 1, 1));
    /// assert_eq!(Vector::one(), v);
    /// ```
    pub fn one() -> Self
    where
        T: num::Num + Clone,
    {
        Self::repeat(T::one())
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Into<f64>,
{
    /// Calculate the magnitude of this vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let mut v = Vector::new2(3, 4);
    ///
    /// assert_eq!(v.magnitude(), 5.0)
    /// ```
    pub fn magnitude(&self) -> f64
    where
        T: Clone,
    {
        self.iter()
            .map(|i: &T| {
                let f: f64 = i.clone().into();
                f.pow(2)
            })
            .sum::<f64>()
            .sqrt()
    }

    /// Create a new vector with the same direction but another magnitude
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    /// let mut v = Vector::new2(3, 4);
    ///
    /// assert_eq!(v.with_magnitude(10), Vector::new((6.0, 8.0)))
    /// ```
    pub fn with_magnitude(&self, magnitude: impl Into<f64>) -> Vector<f64, DIM>
    where
        T: Clone + Into<f64>,
    {
        (self.map(|i| -> f64 { i.clone().into() }) / Vector::<_, DIM>::repeat(self.magnitude()))
            .scale(magnitude.into())
    }

    /// Normalizes the vector. Sets the magnitude to 1.
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    /// let mut v = Vector::new2(3, 4);
    ///
    /// assert_eq!(v.normalize(), Vector::new((3.0/5.0, 4.0/5.0)))
    /// ```
    pub fn normalize(&self) -> Vector<f64, DIM>
    where
        T: Clone + Into<f64>,
    {
        self.with_magnitude(1)
    }

    /// Limit the magnitude of a vector. If the magnitude is less than the limit
    /// nothing changes (except all values are cast to floats). If the magnitude
    /// is larger than the limit, the magnitude is set to this limit.
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// assert_eq!(Vector::new2(3, 4).limit(10), Vector::new((3.0, 4.0)));
    /// assert_eq!(Vector::new2(9, 12).limit(10), Vector::new((6.0, 8.0)));
    /// ```
    pub fn limit(&self, limit: impl Into<f64>) -> Vector<f64, DIM>
    where
        T: Clone + Into<f64>,
    {
        let limit = limit.into();
        if self.magnitude() > limit {
            self.with_magnitude(limit)
        } else {
            self.map(|i| i.clone().into())
        }
    }

    /// Calculates the angle between two vectors (in radians)
    ///
    /// ```
    /// # use perpendicular::Vector;
    ///
    /// let mut v1 = Vector::new2(0, 1);
    /// let mut v2 = Vector::new2(1, 0);
    ///
    /// assert_eq!(v1.angle(&v2).to_degrees(), 90.0)
    /// ```
    pub fn angle<O>(&self, other: &Vector<O, DIM>) -> f64
    where
        T: Mul<O> + Clone,
        <T as Mul<O>>::Output: Sum + Into<f64>,
        O: Clone + Into<f64>,
    {
        let a: f64 = self.dot(other).into() / (self.magnitude() * other.magnitude());
        a.acos()
    }

    /// Calculate the distance from this vector to another vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let mut v1 = Vector::new2(0, 0);
    /// let mut v2 = Vector::new2(3, 4);
    ///
    /// assert_eq!(v1.distance(&v2), 5.0)
    /// ```
    pub fn distance<O>(&self, other: &Vector<O, DIM>) -> f64
    where
        for<'a> Self: Sub<&'a Vector<O, DIM>>,
        O: Into<f64> + Clone,
        T: Clone,
    {
        (self.map(|i| i.clone().into()) - other.map(|i| i.clone().into())).magnitude()
    }

    /// Calculate the dot product of this vector
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v1 = Vector::new((1, 2));
    /// let v2 = Vector::new((&2, &1));
    ///
    /// assert_eq!(v1.dot(&v2), 4)
    ///
    /// ```
    pub fn dot<O>(&self, other: &Vector<O, DIM>) -> <T as Mul<O>>::Output
    where
        T: Mul<O>,
        for<'a, 'b> &'a Self: Mul<&'b Vector<O, DIM>, Output = Vector<<T as Mul<O>>::Output, DIM>>,
        <T as Mul<O>>::Output: Sum<<T as Mul<O>>::Output>,
    {
        (self * other).into_iter().sum()
    }

    /// Find if the angle between two vectors is 90 degrees
    ///
    /// ```rust
    /// # use perpendicular::Vector;
    ///
    /// let v1 = Vector::new((0, 1));
    /// let v2 = Vector::new((1, 0));
    /// let v3 = Vector::new((1, 1));
    ///
    /// assert!(v1.perpendicular(&v2));
    /// assert!(!v1.perpendicular(&v3));
    ///
    /// ```
    pub fn perpendicular<O>(&self, other: &Vector<O, DIM>) -> bool
    where
        T: Mul<O>,
        for<'a, 'b> &'a Self: Mul<&'b Vector<O, DIM>, Output = Vector<<T as Mul<O>>::Output, DIM>>,
        <T as Mul<O>>::Output: Sum<<T as Mul<O>>::Output>,
        <T as Mul<O>>::Output: num::Num,
    {
        self.dot(other) == num::zero()
    }
}

/// Trait to allow for mapping Vector *and* &Vector
pub trait MapVector<T, const DIM: usize> {
    /// Map an operation over every element of the vector
    ///
    /// ```rust
    /// # use crate::perpendicular::Vector;
    /// use perpendicular::MapVector;
    /// let v = Vector::new((1, 2, 3, 4));
    /// assert_eq!(v.clone().map(|i| i * 3), Vector::new((3, 6, 9, 12)));
    /// assert_eq!(v.clone().map(|i| -i), Vector::new((-1, -2, -3, -4)));
    /// ```
    fn map<U, F: FnMut(T) -> U>(self, func: F) -> Vector<U, DIM>;
}

impl<'a, T, const DIM: usize> MapVector<&'a T, DIM> for &'a Vector<T, DIM> {
    fn map<U, F: FnMut(&'a T) -> U>(self, func: F) -> Vector<U, DIM> {
        Vector::try_new(self.into_iter().map(func)).expect(same_length!())
    }
}

impl<T, const DIM: usize> MapVector<T, DIM> for Vector<T, DIM> {
    fn map<U, F: FnMut(T) -> U>(self, func: F) -> Vector<U, DIM> {
        Vector::try_new(self.into_iter().map(func)).expect(same_length!())
    }
}

impl<T: fmt::Display, const DIM: usize> fmt::Display for Vector<T, DIM> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec{}(", self.dimensions())?;

        let mut iter = self.iter();
        if let Some(i) = iter.next() {
            write!(f, "{}", i)?;
        }
        for i in iter {
            write!(f, ", {}", i)?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl<T, const DIM: usize> Index<usize> for Vector<T, DIM> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T, const DIM: usize> IndexMut<usize> for Vector<T, DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T, const DIM: usize> IntoIterator for Vector<T, DIM> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, DIM>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new(self.values)
    }
}

impl<'a, T, const DIM: usize> IntoIterator for &'a Vector<T, DIM> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(arr: [T; N]) -> Self {
        Self::new_from_arr(arr)
    }
}

macro_rules! length {
    ($_: tt $($rest: tt)*) => {
        1usize + length!($($rest)*)
    };
    () => {
        0usize
    };
}

mod private {
    pub trait Sealed {}
}

/// Trait to allow for turning Vector *and* &Vector into tuples
///
/// ```rust
///
///
/// ```
///
pub trait AsTuple: private::Sealed {
    type Res;

    /// Turn a vector into a tuple
    ///
    /// ```rust
    /// # use crate::perpendicular::Vector;
    /// use perpendicular::AsTuple;
    /// let v = Vector::new((1, 2, 3, 4));
    ///
    /// assert_eq!(v.as_tuple(), (1, 2, 3, 4));
    /// assert_eq!((&v).as_tuple(), (&1, &2, &3, &4));
    /// ```
    fn as_tuple(self) -> Self::Res;
}

macro_rules! replace_ident {
    ($i:ident => $($j:tt)*) => ($($j)*)
}

macro_rules! impl_tuple {
    ($first: tt $($rest: tt)*) => {
        impl<T> From<(T, $(replace_ident!($rest => T)),*)> for Vector<T, {length!($($rest)*)+1}> {
            #[allow(non_snake_case)]
            fn from(($first, $($rest),*): (T, $(replace_ident!($rest => T)),*)) -> Self {
                Self::new_from_arr([$first, $($rest),*])
            }
        }

        impl<T> private::Sealed for Vector<T, {length!($($rest)*)+1}> {}
        impl<'a, T> private::Sealed for &'a Vector<T, {length!($($rest)*)+1}> {}

        impl<T> AsTuple for Vector<T, {length!($($rest)*)+1}> {
            type Res = (T, $(replace_ident!($rest => T)),*);

            #[allow(non_snake_case)]
            fn as_tuple(self) -> (T, $(replace_ident!($rest => T)),*) {
                let [$first, $($rest),*] = self.values;
                ($first, $($rest),*)
            }
        }

        impl<'a, T> AsTuple for &'a Vector<T, {length!($($rest)*)+1}> {
            type Res = (&'a T, $(replace_ident!($rest => &'a T)),*);

            #[allow(non_snake_case)]
            fn as_tuple(self) -> (&'a T, $(replace_ident!($rest => &'a T)),*) {
                let [$first, $($rest),*] = &self.values;
                ($first, $($rest),*)
            }
        }
        impl_tuple!($($rest)*);
    };
    () => {}
}

impl_tuple!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF );

macro_rules! names {
    ($($letters: ident),*;$($rest: tt),*) => {
        names!(; $($letters)*; $($rest)*);
    };

    ($($had: ident $had_length: tt)*; $letter: ident $($letters: ident)*; $($rest: tt)*) => {
        impl<T> Vector<T, {length!($($had)*)+1}> {
            $(
                #[allow(unused)]
                pub fn $had(&self) -> &T {
                    self.get($had_length).expect(same_length!())
                }
                concat_idents::concat_idents!(fn_name = $had, _mut {
                    #[allow(unused)]
                    pub fn fn_name(&mut self) -> &mut T {
                        self.get_mut($had_length).expect(same_length!())
                    }
                });
            )*

            #[allow(unused)]
            pub fn $letter(&self) -> &T {
                self.get(length!($($had)*)).expect(same_length!())
            }
            concat_idents::concat_idents!(fn_name = $letter, _mut {
                #[allow(unused)]
                pub fn fn_name(&mut self) -> &mut T {
                    self.get_mut(length!($($had)*)).expect(same_length!())
                }
            });
        }

        names!($($had $had_length)* $letter {length!($($had)*)}; $($letters)*; $($rest)*);
    };

    ($($had: ident $had_length: tt)*;; $r: tt $($rest: tt)*) => {
        impl<T> Vector<T, {length!($($had)*)+1 + length!($($rest)*)}> {
            $(
                #[allow(unused)]
                pub fn $had(&self) -> &T {
                    self.get($had_length).expect(same_length!())
                }
                concat_idents::concat_idents!(fn_name = $had, _mut {
                    #[allow(unused)]
                    pub fn fn_name(&mut self) -> &mut T {
                        self.get_mut($had_length).expect(same_length!())
                    }
                });
            )*
        }
        names!($($had $had_length)*;; $($rest)*);
    };
    ($($had: ident $had_length: tt)*;;) => {}
}

names!(x, y, z, w; _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _);
names!(a, b, c, d; _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _);
names!(m, n; _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _);

macro_rules! impl_bin_op {
    ($op: tt, $trait: ident, $method: ident) => {
        impl<T: $trait<T>, const DIM: usize> $trait<Vector<T, DIM>> for Vector<T, DIM> {
            type Output = Vector<<T as $trait<T>>::Output, DIM>;
            fn $method(self, rhs: Self) -> Self::Output {
                let self_iter = std::array::IntoIter::new(self.values);
                let other_iter = std::array::IntoIter::new(rhs.values);
                Vector::new_from_arr(
                    from_iter(
                        self_iter.zip(other_iter)
                            .map(|(x, y)| {x $op y})
                    ).expect(same_length!())
                )
            }
        }

        // TODO: remove clone bound
        impl<T: $trait<T> + Clone, const DIM: usize> $trait<&Vector<T, DIM>> for Vector<T, DIM> {
            type Output = Vector<<T as $trait<T>>::Output, DIM>;

            fn $method(self, rhs: &Self) -> Self::Output {
                let self_iter = std::array::IntoIter::new(self.values);
                let other_iter = rhs.values.iter();
                Vector::new_from_arr(
                    from_iter(
                        self_iter.zip(other_iter)
                            .map(|(x, y)| {x $op y.clone()})
                    ).expect(same_length!())
                )
            }
        }

        // TODO: remove clone bound
        impl<U: Clone, T: $trait<U> + Clone, const DIM: usize> $trait<Vector<U, DIM>> for &Vector<T, DIM> {
            type Output = Vector<<T as $trait<U>>::Output, DIM>;

            fn $method(self, rhs: Vector<U, DIM>) -> Self::Output {
                let self_iter = self.values.iter();
                let other_iter = std::array::IntoIter::new(rhs.values);
                Vector::new_from_arr(
                    from_iter(
                        self_iter.zip(other_iter)
                            .map(|(x, y)| {x.clone() $op y})
                    ).expect(same_length!())
                )
            }
        }

        impl<'a, 'b, U: Clone, T: $trait<U> + Clone, const DIM: usize> $trait<&'a Vector<U, DIM>> for &'b Vector<T, DIM> {
            type Output = Vector<<T as $trait<U>>::Output, DIM>;

            fn $method(self, rhs: &'a Vector<U, DIM>) -> Self::Output {
                let self_iter = self.values.iter();
                let other_iter = rhs.values.iter();
                Vector::new_from_arr(
                    from_iter(
                        self_iter.zip(other_iter)
                            .map(|(x, y)| {x.clone() $op y.clone()})
                    ).expect(same_length!())
                )
            }
        }
    };
}

impl_bin_op!(+, Add, add);
impl_bin_op!(-, Sub, sub);
impl_bin_op!(*, Mul, mul);
impl_bin_op!(/, Div, div);
impl_bin_op!(%, Rem, rem);

impl<T: Neg, const DIM: usize> Neg for Vector<T, DIM> {
    type Output = Vector<<T as Neg>::Output, DIM>;

    fn neg(self) -> Self::Output {
        let self_iter = std::array::IntoIter::new(self.values);
        Vector::try_new_overflow(self_iter.map(|i| -i)).expect(same_length!())
    }
}

impl<'a, T, const DIM: usize> Neg for &'a Vector<T, DIM>
where
    &'a T: Neg,
{
    type Output = Vector<<&'a T as Neg>::Output, DIM>;

    fn neg(self) -> Self::Output {
        Vector::try_new_overflow(self.values.iter().map(|i| -i)).expect(same_length!())
    }
}

#[cfg(test)]
mod tests {
    use crate::{AsTuple, Vector};

    #[test]
    pub fn test_letters() {
        assert_eq!(Vector::new([1]).x(), &1);
        assert_eq!(Vector::new([1, 2]).x(), &1);
        assert_eq!(Vector::new([1, 2, 3]).x(), &1);
        assert_eq!(Vector::new([1, 2, 3, 4]).x(), &1);

        assert_eq!(Vector::new([1, 2]).y(), &2);
        assert_eq!(Vector::new([1, 2, 3]).y(), &2);
        assert_eq!(Vector::new([1, 2, 3, 4]).y(), &2);

        assert_eq!(Vector::new([1, 2, 3]).z(), &3);
        assert_eq!(Vector::new([1, 2, 3, 4]).z(), &3);

        assert_eq!(Vector::new([1, 2, 3, 4]).w(), &4);

        assert_eq!(Vector::new([1, 2, 3, 4, 5]).x(), &1);
        assert_eq!(Vector::new([1, 2, 3, 4, 5]).y(), &2);
        assert_eq!(Vector::new([1, 2, 3, 4, 5]).z(), &3);
        assert_eq!(Vector::new([1, 2, 3, 4, 5]).w(), &4);

        assert_eq!(Vector::new([1, 2, 3, 4, 5, 6]).x(), &1);
        assert_eq!(Vector::new([1, 2, 3, 4, 5, 6]).y(), &2);
        assert_eq!(Vector::new([1, 2, 3, 4, 5, 6]).z(), &3);
        assert_eq!(Vector::new([1, 2, 3, 4, 5, 6]).w(), &4);
    }

    #[test]
    pub fn test_ops() {
        let a = Vector::new2(1, 2);
        let b = Vector::new2(3, 4);
        assert_eq!(a + b, Vector::new2(4, 6));
        assert_eq!(a + &b, Vector::new2(4, 6));
        assert_eq!(&a + b, Vector::new2(4, 6));
        assert_eq!(&a + &b, Vector::new2(4, 6));

        assert_eq!(a - b, Vector::new2(-2, -2));
        assert_eq!(a - &b, Vector::new2(-2, -2));
        assert_eq!(&a - b, Vector::new2(-2, -2));
        assert_eq!(&a - &b, Vector::new2(-2, -2));

        assert_eq!(a * b, Vector::new2(3, 8));
        assert_eq!(a * &b, Vector::new2(3, 8));
        assert_eq!(&a * b, Vector::new2(3, 8));
        assert_eq!(&a * &b, Vector::new2(3, 8));

        assert_eq!(a / b, Vector::new2(0, 0));
        assert_eq!(a / &b, Vector::new2(0, 0));
        assert_eq!(&a / b, Vector::new2(0, 0));
        assert_eq!(&a / &b, Vector::new2(0, 0));
    }

    #[test]
    pub fn test_from_tuple() {
        let v: Vector<_, 1> = (1,).into();
        assert_eq!(v.x(), &1);
        let v = Vector::new((1,));
        assert_eq!(v.x(), &1);
        let v = Vector::new((1, 2, 3, 4));
        assert_eq!(v.z(), &3);
    }

    #[test]
    pub fn test_to_tuple() {
        let v: Vector<_, 1> = (1,).into();
        assert_eq!(v.as_tuple(), (1,));
        let v = Vector::new((1, 2, 3, 4));
        assert_eq!(v.as_tuple(), (1, 2, 3, 4));
    }

    #[test]
    pub fn test_index() {
        let v = Vector::new((1, 2, 3, 4));
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
        assert_eq!(v[3], 4);
    }

    #[test]
    pub fn test_display() {
        let v = Vector::new((1, 2, 3, 4));
        assert_eq!(format!("{}", v), "vec4(1, 2, 3, 4)");

        let v = Vector::new((1, 2));
        assert_eq!(format!("{}", v), "vec2(1, 2)");

        let v = Vector::new((1,));
        assert_eq!(format!("{}", v), "vec1(1)");
    }

    #[test]
    #[should_panic]
    pub fn test_index_out_of_bounds() {
        let v = Vector::new((1, 2, 3, 4));
        v[4];
    }
}
