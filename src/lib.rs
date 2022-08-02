#![feature(associated_type_bounds)]

use std::marker::PhantomData;

pub trait Func<I> {
    type Output;
    fn call(&self, args: I) -> Self::Output;
}

impl<F, R> Func<()> for F
where
    F: Fn() -> R,
{
    type Output = R;

    #[inline]
    fn call(&self, _: ()) -> Self::Output {
        (*self)()
    }
}

/// Implements a macro to expand function implementations that can capture any number of
/// arguments and return a given set output type. Typically this can be expanded to extract
/// values when working with multiple chained functions.
macro_rules! generics {
    ($type:ident) => {
        impl<F, R, $type> Func<($type,)> for F
        where
            F: Fn($type) -> R
        {
            type Output = (R,);

            #[inline]
            fn call(&self, args: ($type,)) -> Self::Output {
                ((*self)(args.0),)
            }
        }
    };

    ($type1:ident, $( $type:ident ),*) => {
        generics!($( $type ),*);

        impl<F, R, $type1, $( $type ),*> Func<($type1, $($type),*)> for F
        where
            F: Fn($type1, $( $type ),*) -> R
        {
            type Output = R;

            #[inline]
            fn call(&self, args: ($type1, $($type),*)) -> Self::Output {
                #[allow(non_snake_case)]
                let ($type1, $( $type ),*) = args;
                (*self)($type1, $( $type ),*)
            }
        }
    };
}

generics! { T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16 }

/// Unwraps a tuple output for the next sequence
pub struct Map<A, B, Args>
where
    A: Func<Args>,
    B: Func<A::Output>,
{
    a: A,
    b: B,
    _p: PhantomData<Args>,
}

pub trait Handler<A, B, Args>
where
    A: Func<Args>,
    B: Func<A::Output>,
{
    type Input;
    type Output;
    fn pipe(self) -> Map<A, B, Args>;
}

impl<A, B, Args> Handler<A, B, Args> for (A, B)
where
    A: Func<Args>,
    B: Func<A::Output>,
{
    type Input = Args;
    type Output = B::Output;

    fn pipe(self) -> Map<A, B, Args> {
        Map {
            a: self.0,
            b: self.1,
            _p: PhantomData::default(),
        }
    }
}

impl<A, B, C, Args> Handler<Map<A, B, Args>, C, Args> for (A, B, C)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
{
    type Input = Args;
    type Output = C::Output;

    fn pipe(self) -> Map<Map<A, B, Args>, C, Args> {
        let m = (self.0, self.1).pipe();
        (m, self.2).pipe()
    }
}

impl<A, B, C, D, Args> Handler<Map<Map<A, B, Args>, C, Args>, D, Args> for (A, B, C, D)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
    D: Func<C::Output>,
{
    type Input = Args;
    type Output = D::Output;

    fn pipe(self) -> Map<Map<Map<A, B, Args>, C, Args>, D, Args> {
        let m = (self.0, self.1).pipe();
        let m = (m, self.2).pipe();
        (m, self.3).pipe()
    }
}

impl<A, B, C, D, E, Args> Handler<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>
    for (A, B, C, D, E)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
    D: Func<C::Output>,
    E: Func<D::Output>,
{
    type Input = Args;
    type Output = E::Output;

    fn pipe(self) -> Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args> {
        let m = (self.0, self.1).pipe();
        let m = (m, self.2).pipe();
        let m = (m, self.3).pipe();
        (m, self.4).pipe()
    }
}

impl<A, B, C, D, E, F, Args>
    Handler<Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>, F, Args>
    for (A, B, C, D, E, F)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
    D: Func<C::Output>,
    E: Func<D::Output>,
    F: Func<E::Output>,
{
    type Input = Args;
    type Output = F::Output;

    fn pipe(self) -> Map<Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>, F, Args> {
        let m = (self.0, self.1).pipe();
        let m = (m, self.2).pipe();
        let m = (m, self.3).pipe();
        let m = (m, self.4).pipe();
        (m, self.5).pipe()
    }
}

impl<A, B, C, D, E, F, G, Args>
    Handler<Map<Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>, F, Args>, G, Args>
    for (A, B, C, D, E, F, G)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
    D: Func<C::Output>,
    E: Func<D::Output>,
    F: Func<E::Output>,
    G: Func<F::Output>,
{
    type Input = Args;
    type Output = G::Output;

    fn pipe(
        self,
    ) -> Map<Map<Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>, F, Args>, G, Args> {
        let m = (self.0, self.1).pipe();
        let m = (m, self.2).pipe();
        let m = (m, self.3).pipe();
        let m = (m, self.4).pipe();
        let m = (m, self.5).pipe();
        (m, self.6).pipe()
    }
}

impl<A, B, C, D, E, F, G, H, Args>
    Handler<
        Map<Map<Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>, F, Args>, G, Args>,
        H,
        Args,
    > for (A, B, C, D, E, F, G, H)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
    D: Func<C::Output>,
    E: Func<D::Output>,
    F: Func<E::Output>,
    G: Func<F::Output>,
    H: Func<G::Output>,
{
    type Input = Args;
    type Output = H::Output;

    fn pipe(
        self,
    ) -> Map<
        Map<Map<Map<Map<Map<Map<A, B, Args>, C, Args>, D, Args>, E, Args>, F, Args>, G, Args>,
        H,
        Args,
    > {
        let m = (self.0, self.1).pipe();
        let m = (m, self.2).pipe();
        let m = (m, self.3).pipe();
        let m = (m, self.4).pipe();
        let m = (m, self.5).pipe();
        let m = (m, self.6).pipe();
        (m, self.7).pipe()
    }
}

impl<A, B, Args> Func<Args> for Map<A, B, Args>
where
    A: Func<Args>,
    B: Func<A::Output>,
{
    type Output = B::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.a.call(args);
        self.b.call(args)
    }
}

fn map<F, F2, Args>(f: F, f2: F2) -> impl Func<Args>
where
    F: Func<Args>,
    F2: Func<F::Output>,
{
    (f, f2).pipe()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {}

    fn input(a: i32) {
        println!("{}", a);
    }

    fn test2(_: ()) {}

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn log_adder(a: i32, b: i32) -> (i32, i32) {
        println!("{a} {b}");
        (a, b)
    }

    fn multiply(c: i32) -> i32 {
        c * 4
    }

    fn stringer<T: std::fmt::Display>(v: T) -> String {
        format!("{}", v)
    }

    fn log_multiply(c: i32) -> i32 {
        println!("{c}");
        c
    }

    fn foo() -> i32 {
        3
    }

    fn multi() -> (i32, i32) {
        (4, 8)
    }

    fn single(a: i32) -> (i32, i32) {
        (a * 2, a * 4)
    }

    fn outer(a: i32, b: i32) -> (i32, i32) {
        (a + 1, b + 1)
    }

    fn tuple_add((a, b): (i32, i32)) -> i32 {
        a + b
    }

    fn plus(a: i32) -> i32 {
        a + 1
    }

    fn assert_impl_handler<Args>(f: impl Func<Args>) {}

    fn assert_impl<A, B, Args>(f: impl Handler<A, B, Args>)
    where
        A: Func<Args>,
        B: Func<A::Output>,
    {
    }

    fn assert_spec_i32<A, B, Args>(f: impl Handler<A, B, Args, Input = (i32,), Output = (i32,)>)
    where
        A: Func<Args>,
        B: Func<A::Output>,
    {
    }

    fn assert_constraint<Args, F, A, B>(f: F) -> impl Func<Args>
    where
        F: Handler<A, B, Args>,
        A: Func<Args>,
        B: Func<A::Output>,
    {
        f.pipe()
    }

    #[test]
    fn test_chain() {
        map(plus, map(multiply, stringer));
        map(plus, multiply);
    }

    #[test]
    fn test_map() {
        // test |> multi |> add
        map(test, map(multi, add));
    }

    #[test]
    fn test_long_tuple() {
        assert_impl((test, multi, add));
        assert_impl((multiply, multiply, multiply, multiply));
        assert_impl((log_multiply, multiply, multiply, multiply));
        assert_impl((plus, multiply, plus, input));
        assert_impl((plus, multiply, single, tuple_add));
        assert_impl((multi, outer, log_adder, add));
    }

    #[test]
    fn test_effects() {
        assert_impl((log_adder, add));
        assert_impl((log_adder, add));
        assert_impl((log_multiply, multiply));
    }

    #[test]
    fn test_constraints() {
        assert_spec_i32((plus, plus));
        assert_constraint((plus, plus));
    }

    #[test]
    fn test_tuple_unwrapping() {
        assert_impl((plus, input));
        //assert_impl((single, add));
        //assert_impl(((single, add).pipe(), plus));
        assert_impl((test, (multi, add).pipe()));
        assert_impl((test, foo));
        assert_impl((multi, add));
        //assert_impl((test, multi, add));
        map(test, map(multi, add));
        assert_impl(((test, multi).pipe(), add));
    }

    #[test]
    fn test_handlers() {
        assert_impl_handler(test);
        assert_impl_handler(add);
        assert_impl_handler(multiply);
        assert_impl_handler(plus);
        assert_impl_handler(foo);
        assert_impl_handler(multi);
        assert_impl_handler(outer);
        //assert_impl((outer, outer, outer));
        assert_impl((outer, outer));
        let _ = map(outer, map(outer, add)).call((4, 5));
        //assert_impl((plus, plus, plus));
        //assert_impl((plus, plus, single));
        //assert_impl((plus, single, tuple_add));
        map(plus, single);
        assert_impl((plus, plus));
        map(plus, map(plus, plus));
        map(test, test);
        assert_impl((test, foo));
        assert_impl((test, multi));
        // FIXME assert_impl((foo, plus)); foo => i32
        // FIXME assert_impl((foo, multiply));
        assert_impl(((multiply, multiply).pipe(), multiply));
        assert_impl((single, tuple_add));
        assert_impl((multiply, single));

        (multi, add).pipe().call(());
    }

    #[test]
    fn test_empty_seq() {
        map(test, test);
        map(test, foo);
        map(input, test2);
        assert_impl((test, test));
        assert_impl((test, foo));
        assert_impl((input, test2));
        // FIXME assert_impl((input, test));
    }
}

/*
/// Default handler implementation for a tuple of handlers.
///
/// ## Examples
/// ```rust
/// use sidemount::Handler;
///
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
/// fn test2() {}
///
/// assert_impl_handler((test, test2));
/// ```
macro_rules! ary {
    ($($name:ident)+) => (
        impl<$($name),*> Handler for ($($name,)*)
            where $($name: Handler),*
        {
            #[allow(non_snake_case)]
            fn call(&self) {
                let ($(ref $name,)*) = *self;
                $(
                    $name.call();
                )*
            }
        }
    );
}

ary! { A B }
ary! { A B C }
ary! { A B C D }
ary! { A B C D E }
ary! { A B C D E F }
ary! { A B C D E F G }
ary! { A B C D E F G H }
ary! { A B C D E F G H I }
ary! { A B C D E F G H I J }
ary! { A B C D E F G H I J K }
ary! { A B C D E F G H I J K L }
ary! { A B C D E F G H I J K L M }
ary! { A B C D E F G H I J K L M N }
ary! { A B C D E F G H I J K L M N O }
ary! { A B C D E F G H I J K L M N O P }
ary! { A B C D E F G H I J K L M N O P Q }
ary! { A B C D E F G H I J K L M N O P Q R }
ary! { A B C D E F G H I J K L M N O P Q R S }
ary! { A B C D E F G H I J K L M N O P Q R S T }
ary! { A B C D E F G H I J K L M N O P Q R S T U }
ary! { A B C D E F G H I J K L M N O P Q R S T U V }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }

/// Default handler implementation for an Arc handler.
///
/// ## Examples
/// ```rust
/// use std::sync::Arc;
/// use sidemount::Handler;
///
/// fn assert_impl_handler(f: impl Handler) {}
///
/// fn test() {}
///
/// assert_impl_handler(Arc::new(test));
/// ```
impl<T> Handler for Arc<T>
where
    T: Handler,
{
    fn call(&self) {
        self.as_ref().call();
    }
}
*/

/*
impl<A, B, Args> Handler<(A, B, PhantomData<Args>, ())> for (A, B)
where
    A: Func<Args>,
    B: Func<(A::Output,)>,
{
    type Input = Args;
    type Output = B::Output;

    fn pipe(self) -> Mapped<(A, B, PhantomData<Args>, ())> {
        Mapped((self.0, self.1, PhantomData::default(), ()))
    }
}
*/
/*
impl<A, B, Args> Func<Args> for Mapped<(A, B, PhantomData<Args>, ())>
where
    A: Func<Args>,
    B: Func<(A::Output,)>,
{
    type Output = B::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let (a, b, ..) = &self.0;
        let args = a.call(args);
        b.call((args,))
    }
}
*/
