///
/// Handler implementations for (A, B, C)
///
impl<A, B, C, Args> Func<Args> for (A, B, C)
where
    A: Func<Args>,
    B: Func<(A::Output,)>,
    C: Func<(B::Output,)>,
{
    type Output = C::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let res = self.0.call(args);
        let res = self.1.call((res,));
        self.2.call((res,))
    }
}

impl<A, B, C, Args> Handler<(A, B, C, PhantomData<Args>)> for (A, B, C)
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
{
    type Input = Args;
    type Output = C::Output;

    fn pipe(self) -> Mapped<(A, B, C, PhantomData<Args>)> {
        Mapped((self.0, self.1, self.2, PhantomData::default()))
    }
}
impl<A, B, C, Args> Handler<(A, B, C, PhantomData<Args>, ())> for (A, B, C)
where
    A: Func<Args>,
    B: Func<(A::Output,)>,
    C: Func<(B::Output,)>,
{
    type Input = Args;
    type Output = C::Output;

    fn pipe(self) -> Mapped<(A, B, C, PhantomData<Args>, ())> {
        Mapped((self.0, self.1, self.2, PhantomData::default(), ()))
    }
}
impl<A, B, C, Args> Func<Args> for Mapped<(A, B, C, PhantomData<Args>)>
where
    A: Func<Args>,
    B: Func<A::Output>,
    C: Func<B::Output>,
{
    type Output = C::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let (a, b, c, ..) = &self.0;
        let args = a.call(args);
        let args = b.call(args);
        c.call(args)
    }
}
impl<A, B, C, Args> Func<Args> for Mapped<(A, B, C, PhantomData<Args>, ())>
where
    A: Func<Args>,
    B: Func<(A::Output,)>,
    C: Func<(B::Output,)>,
{
    type Output = C::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let (a, b, c, ..) = &self.0;
        let args = a.call(args);
        let args = b.call((args,));
        c.call((args,))
    }
}
