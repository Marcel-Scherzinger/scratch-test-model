pub trait QuirkSink<Msg> {
    fn put(&mut self, msg: Msg);
}

impl<M> QuirkSink<M> for () {
    fn put(&mut self, _: M) {}
}

pub struct ScopedQuirkSink<'a, QS, F> {
    inner_sink: &'a mut QS,
    wrapper: F,
}

impl<'a, QS, F, I, O> QuirkSink<I> for ScopedQuirkSink<'a, QS, F>
where
    F: Fn(I) -> O,
    QS: QuirkSink<O>,
{
    fn put(&mut self, msg: I) {
        self.inner_sink.put((self.wrapper)(msg));
    }
}

pub trait ScopableQuirkSink<I, O> {
    fn scope_map<F>(&mut self, func: F) -> ScopedQuirkSink<'_, Self, F>
    where
        Self: Sized,
        F: Fn(I) -> O,
        Self: QuirkSink<O>;
}

impl<QS, I, O> ScopableQuirkSink<I, O> for QS
where
    QS: QuirkSink<O>,
{
    fn scope_map<F>(&mut self, func: F) -> ScopedQuirkSink<'_, Self, F>
    where
        Self: Sized,
        F: Fn(I) -> O,
        Self: QuirkSink<O>,
    {
        ScopedQuirkSink {
            inner_sink: self,
            wrapper: func,
        }
    }
}
