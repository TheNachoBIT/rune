macro_rules! unsigned {
    ($m:ident, $ty:ty) => {
        $m.function("parse", parse).build()?;
        $m.function_meta(to_float)?;

        $m.function_meta(max)?;
        $m.function_meta(min)?;
        $m.function_meta(pow)?;

        $m.function_meta(checked_add)?;
        $m.function_meta(checked_sub)?;
        $m.function_meta(checked_div)?;
        $m.function_meta(checked_mul)?;
        $m.function_meta(checked_rem)?;

        $m.function_meta(wrapping_add)?;
        $m.function_meta(wrapping_sub)?;
        $m.function_meta(wrapping_div)?;
        $m.function_meta(wrapping_mul)?;
        $m.function_meta(wrapping_rem)?;

        $m.function_meta(saturating_add)?;
        $m.function_meta(saturating_sub)?;
        $m.function_meta(saturating_mul)?;
        $m.function_meta(saturating_pow)?;

        $m.function_meta(to_string)?;

        $m.function_meta(clone__meta)?;
        $m.implement_trait::<$ty>(rune::item!(::std::clone::Clone))?;

        $m.function_meta(partial_eq__meta)?;
        $m.implement_trait::<$ty>(rune::item!(::std::cmp::PartialEq))?;

        $m.function_meta(eq__meta)?;
        $m.implement_trait::<$ty>(rune::item!(::std::cmp::Eq))?;

        $m.function_meta(partial_cmp__meta)?;
        $m.implement_trait::<$ty>(rune::item!(::std::cmp::PartialOrd))?;

        $m.function_meta(cmp__meta)?;
        $m.implement_trait::<$ty>(rune::item!(::std::cmp::Ord))?;

        $m.constant("MIN", <$ty>::MIN).build()?.docs(docstring! {
            /// The smallest value that can be represented by this integer type
            /// (&minus;2<sup>63</sup>).
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rune
            #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN, -9223372036854775808);")]
            /// ```
        })?;

        $m.constant("MAX", <$ty>::MAX).build()?.docs(docstring! {
            /// The largest value that can be represented by this integer type
            /// (2<sup>63</sup> &minus; 1).
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```rune
            #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX, 9223372036854775807);")]
            /// ```
        })?;
    };
}

macro_rules! unsigned_fns {
    ($ty:ty) => {
        #[doc = concat!(" Parse an `", stringify!($ty), "`.")]
        ///
        /// # Examples
        ///
        /// ```rune
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::parse(\"10\")?, 10);")]
        /// ```
        fn parse(s: &str) -> Result<$ty, ParseIntError> {
            str::parse::<$ty>(s)
        }

        #[doc = concat!(" Converts an `", stringify!($ty), "` to a `f64`.")]
        ///
        /// # Examples
        ///
        /// ```rune
        /// assert!(10.to::<f64>() is f64);
        /// ```
        #[rune::function(instance, path = to::<f64>)]
        #[inline]
        fn to_float(value: $ty) -> f64 {
            value as f64
        }

        /// Compares and returns the maximum of two values.
        ///
        /// Returns the second argument if the comparison determines them to be equal.
        ///
        /// # Examples
        ///
        /// ```rune
        /// assert_eq!(2, 1.max(2));
        /// assert_eq!(2, 2.max(2));
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn max(this: $ty, other: $ty) -> $ty {
            <$ty>::max(this, other)
        }

        /// Compares and returns the minimum of two values.
        ///
        /// Returns the first argument if the comparison determines them to be equal.
        ///
        /// # Examples
        ///
        /// ```rune
        /// assert_eq!(1, 1.min(2));
        /// assert_eq!(2, 2.min(2));
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn min(this: $ty, other: $ty) -> $ty {
            <$ty>::min(this, other)
        }

        /// Raises self to the power of `exp`, using exponentiation by squaring.
        ///
        /// # Overflow behavior
        ///
        /// This function will wrap on overflow.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// let x = 2;
        ///
        /// assert_eq!(x.pow(5), 32);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn pow(this: $ty, pow: u32) -> $ty {
            <$ty>::wrapping_pow(this, pow)
        }

        /// Checked integer addition. Computes `self + rhs`, returning `None` if
        /// overflow occurred.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        #[doc = concat!(" assert_eq!((", stringify!($ty), "::MAX - 2).checked_add(1), Some(", stringify!($ty), "::MAX - 1));")]
        #[doc = concat!(" assert_eq!((", stringify!($ty), "::MAX - 2).checked_add(3), None);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn checked_add(this: $ty, rhs: $ty) -> Option<$ty> {
            <$ty>::checked_add(this, rhs)
        }

        /// Checked integer subtraction. Computes `self - rhs`, returning `None` if
        /// overflow occurred.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        #[doc = concat!(" assert_eq!((", stringify!($ty), "::MIN + 2).checked_sub(1), Some(", stringify!($ty), "::MIN + 1));")]
        #[doc = concat!(" assert_eq!((", stringify!($ty), "::MIN + 2).checked_sub(3), None);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn checked_sub(this: $ty, rhs: $ty) -> Option<$ty> {
            <$ty>::checked_sub(this, rhs)
        }

        /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs ==
        /// 0` or the division results in overflow.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        #[doc = concat!(" assert_eq!((", stringify!($ty), "::MIN + 1).checked_div(-1), Some(", stringify!($ty), "::MAX));")]
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.checked_div(-1), None);")]
        /// assert_eq!((1).checked_div(0), None);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn checked_div(this: $ty, rhs: $ty) -> Option<$ty> {
            <$ty>::checked_div(this, rhs)
        }

        /// Checked integer multiplication. Computes `self * rhs`, returning `None` if
        /// overflow occurred.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX.checked_mul(1), Some(", stringify!($ty), "::MAX));")]
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX.checked_mul(2), None);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn checked_mul(this: $ty, rhs: $ty) -> Option<$ty> {
            <$ty>::checked_mul(this, rhs)
        }

        /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs
        /// == 0` or the division results in overflow.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(5.checked_rem(2), Some(1));
        /// assert_eq!(5.checked_rem(0), None);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.checked_rem(-1), None);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn checked_rem(this: $ty, rhs: $ty) -> Option<$ty> {
            <$ty>::checked_rem(this, rhs)
        }

        /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the
        /// boundary of the type.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(100.wrapping_add(27), 127);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX.wrapping_add(2), ", stringify!($ty), "::MIN + 1);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn wrapping_add(this: $ty, rhs: $ty) -> $ty {
            <$ty>::wrapping_add(this, rhs)
        }

        /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at
        /// the boundary of the type.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(0.wrapping_sub(127), -127);
        #[doc = concat!(" assert_eq!((-2).wrapping_sub(", stringify!($ty), "::MAX), ", stringify!($ty), "::MAX);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn wrapping_sub(this: $ty, rhs: $ty) -> $ty {
            <$ty>::wrapping_sub(this, rhs)
        }

        /// Wrapping (modular) division. Computes `self / rhs`, wrapping around at the
        /// boundary of the type.
        ///
        /// The only case where such wrapping can occur is when one divides `MIN / -1`
        /// on a signed type (where `MIN` is the negative minimal value for the type);
        /// this is equivalent to `-MIN`, a positive value that is too large to
        /// represent in the type. In such a case, this function returns `MIN` itself.
        ///
        /// # Panics
        ///
        /// This function will panic if `rhs` is 0.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(100.wrapping_div(10), 10);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn wrapping_div(this: $ty, rhs: $ty) -> VmResult<$ty> {
            if rhs == 0 {
                return VmResult::err(VmErrorKind::DivideByZero);
            }

            VmResult::Ok(<$ty>::wrapping_div(this, rhs))
        }

        /// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at
        /// the boundary of the type.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(10.wrapping_mul(12), 120);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn wrapping_mul(this: $ty, rhs: $ty) -> $ty {
            <$ty>::wrapping_mul(this, rhs)
        }

        /// Wrapping (modular) remainder. Computes `self % rhs`, wrapping around at the
        /// boundary of the type.
        ///
        /// Such wrap-around never actually occurs mathematically; implementation
        /// artifacts make `x % y` invalid for `MIN / -1` on a signed type (where `MIN`
        /// is the negative minimal value). In such a case, this function returns `0`.
        ///
        /// # Panics
        ///
        /// This function will panic if `rhs` is 0.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(100.wrapping_rem(10), 0);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn wrapping_rem(this: $ty, rhs: $ty) -> VmResult<$ty> {
            if rhs == 0 {
                return VmResult::err(VmErrorKind::DivideByZero);
            }

            VmResult::Ok(<$ty>::wrapping_rem(this, rhs))
        }

        /// Saturating integer addition. Computes `self + rhs`, saturating at the
        /// numeric bounds instead of overflowing.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(100.saturating_add(1), 101);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX.saturating_add(100), ", stringify!($ty), "::MAX);")]
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.saturating_add(-1), ", stringify!($ty), "::MIN);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn saturating_add(this: $ty, rhs: $ty) -> $ty {
            <$ty>::saturating_add(this, rhs)
        }

        /// Saturating integer subtraction. Computes `self - rhs`, saturating at the
        /// numeric bounds instead of overflowing.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(100.saturating_sub(127), -27);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.saturating_sub(100), ", stringify!($ty), "::MIN);")]
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX.saturating_sub(-1), ", stringify!($ty), "::MAX);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn saturating_sub(this: $ty, rhs: $ty) -> $ty {
            <$ty>::saturating_sub(this, rhs)
        }

        /// Saturating integer multiplication. Computes `self * rhs`, saturating at the
        /// numeric bounds instead of overflowing.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(10.saturating_mul(12), 120);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MAX.saturating_mul(10), ", stringify!($ty), "::MAX);")]
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.saturating_mul(10), ", stringify!($ty), "::MIN);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn saturating_mul(this: $ty, rhs: $ty) -> $ty {
            <$ty>::saturating_mul(this, rhs)
        }

        /// Saturating integer exponentiation. Computes `self.pow(exp)`, saturating at
        /// the numeric bounds instead of overflowing.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!((-4).saturating_pow(3), -64);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.saturating_pow(2), ", stringify!($ty), "::MAX);")]
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.saturating_pow(3), ", stringify!($ty), "::MIN);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn saturating_pow(this: $ty, rhs: u32) -> $ty {
            <$ty>::saturating_pow(this, rhs)
        }

        #[doc = concat!(" Clone a `", stringify!($ty), "`.")]
        ///
        /// Note that since the type is copy, cloning has the same effect as assigning
        /// it.
        ///
        /// # Examples
        ///
        /// ```rune
        /// let a = 5;
        /// let b = a;
        /// let c = a.clone();
        ///
        /// a += 1;
        ///
        /// assert_eq!(a, 6);
        /// assert_eq!(b, 5);
        /// assert_eq!(c, 5);
        /// ```
        #[rune::function(keep, instance, protocol = CLONE)]
        #[inline]
        fn clone(this: $ty) -> $ty {
            this
        }

        /// Test two integers for partial equality.
        ///
        /// # Examples
        ///
        /// ```rune
        /// use std::ops::partial_eq;
        ///
        /// assert_eq!(partial_eq(5, 5), true);
        /// assert_eq!(partial_eq(5, 10), false);
        /// assert_eq!(partial_eq(10, 5), false);
        /// ```
        #[rune::function(keep, instance, protocol = PARTIAL_EQ)]
        #[inline]
        fn partial_eq(this: $ty, rhs: $ty) -> bool {
            this.eq(&rhs)
        }

        /// Test two integers for total equality.
        ///
        /// # Examples
        ///
        /// ```rune
        /// use std::ops::eq;
        ///
        /// assert_eq!(eq(5, 5), true);
        /// assert_eq!(eq(5, 10), false);
        /// assert_eq!(eq(10, 5), false);
        /// ```
        #[rune::function(keep, instance, protocol = EQ)]
        #[inline]
        fn eq(this: $ty, rhs: $ty) -> bool {
            this.eq(&rhs)
        }

        /// Perform a partial ordered comparison between two integers.
        ///
        /// # Examples
        ///
        /// ```rune
        /// use std::cmp::Ordering;
        /// use std::ops::partial_cmp;
        ///
        /// assert_eq!(partial_cmp(5, 10), Some(Ordering::Less));
        /// assert_eq!(partial_cmp(10, 5), Some(Ordering::Greater));
        /// assert_eq!(partial_cmp(5, 5), Some(Ordering::Equal));
        /// ```
        #[rune::function(keep, instance, protocol = PARTIAL_CMP)]
        #[inline]
        fn partial_cmp(this: $ty, rhs: $ty) -> Option<Ordering> {
            this.partial_cmp(&rhs)
        }

        /// Perform a totally ordered comparison between two integers.
        ///
        /// # Examples
        ///
        /// ```rune
        /// use std::cmp::Ordering;
        /// use std::ops::cmp;
        ///
        /// assert_eq!(cmp(5, 10), Ordering::Less);
        /// assert_eq!(cmp(10, 5), Ordering::Greater);
        /// assert_eq!(cmp(5, 5), Ordering::Equal);
        /// ```
        #[rune::function(keep, instance, protocol = CMP)]
        #[inline]
        fn cmp(this: $ty, rhs: $ty) -> Ordering {
            this.cmp(&rhs)
        }

        /// Returns the number as a string.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!((-10).to_string(), "-10");
        /// assert_eq!(10.to_string(), "10");
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn to_string(this: $ty) -> VmResult<alloc::String> {
            VmResult::Ok(vm_try!(this.try_to_string()))
        }
    };
}

macro_rules! signed {
    ($m:ident, $ty:ty) => {
        unsigned!($m, $ty);

        $m.function_meta(abs)?;
        $m.function_meta(saturating_abs)?;
        $m.function_meta(signum)?;
        $m.function_meta(is_positive)?;
        $m.function_meta(is_negative)?;
    };
}

macro_rules! signed_fns {
    ($ty:ty) => {
        unsigned_fns!($ty);

        /// Computes the absolute value of `self`.
        ///
        /// # Overflow behavior
        ///
        #[doc = concat!(" The absolute value of `", stringify!($ty), "::MIN` cannot be represented as an `int`,")]
        /// and attempting to calculate it will cause an overflow. This means
        #[doc = concat!(" that such code will wrap to `", stringify!($ty), "::MIN` without a panic.")]
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(10.abs(), 10);
        /// assert_eq!((-10).abs(), 10);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn abs(this: $ty) -> $ty {
            <$ty>::wrapping_abs(this)
        }

        /// Saturating absolute value. Computes `self.abs()`, returning `MAX` if `self
        /// == MIN` instead of overflowing.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(100.saturating_abs(), 100);
        /// assert_eq!((-100).saturating_abs(), 100);
        #[doc = concat!(" assert_eq!(", stringify!($ty), "::MIN.saturating_abs(), ", stringify!($ty), "::MAX);")]
        #[doc = concat!(" assert_eq!((", stringify!($ty), "::MIN + 1).saturating_abs(), ", stringify!($ty), "::MAX);")]
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn saturating_abs(this: $ty) -> $ty {
            <$ty>::saturating_abs(this)
        }

        /// Returns a number representing sign of `self`.
        ///
        /// - `0` if the number is zero
        /// - `1` if the number is positive
        /// - `-1` if the number is negative
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert_eq!(10.signum(), 1);
        /// assert_eq!(0.signum(), 0);
        /// assert_eq!((-10).signum(), -1);
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn signum(this: $ty) -> $ty {
            <$ty>::signum(this)
        }

        /// Returns `true` if `self` is positive and `false` if the number is zero or
        /// negative.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert!(10.is_positive());
        /// assert!(!(-10).is_positive());
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn is_positive(this: $ty) -> bool {
            <$ty>::is_positive(this)
        }

        /// Returns `true` if `self` is negative and `false` if the number is zero or
        /// positive.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```rune
        /// assert!((-10).is_negative());
        /// assert!(!10.is_negative());
        /// ```
        #[rune::function(instance)]
        #[inline]
        fn is_negative(this: $ty) -> bool {
            <$ty>::is_negative(this)
        }
    }
}
