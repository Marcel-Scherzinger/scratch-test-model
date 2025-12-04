/// Interface for messages regarding program quriks the user should be warned about.
///
/// Often Scratch tries to do _something_ so that it doesn't have to
/// raise an exception or fatal error. It allows accessing a list
/// out of bounds, computes modulo with floating point arguments, ...
///
/// As the interpreter has to mimic this behaviour it also generates
/// warnings when such behaviour is encountered as it could be considered
/// bad practice. Every "quirky" operation can define an own type with
/// possible warnings by requiring a _qurik sink_ of the type.
///
/// It then would perform it's operation according to Scratch semantics
/// while still being able to emit warnings to the sink.
///
/// `()` is a valid sink for **any** message type so it can be used
/// to ignore any message an operation might send.
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
