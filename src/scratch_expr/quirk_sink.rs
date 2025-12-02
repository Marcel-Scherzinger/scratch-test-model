pub trait QuirkSink<Msg> {
    fn put(&mut self, msg: Msg);
}

impl<M> QuirkSink<M> for () {
    fn put(&mut self, _: M) {}
}
