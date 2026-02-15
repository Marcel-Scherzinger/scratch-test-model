#[derive(Debug, derive_more::Display)]
#[display("{inner_error}: {json_ctx}")]
pub struct JsonCtx<'a, E> {
    pub(crate) inner_error: E,
    pub(crate) json_ctx: &'a serde_json::Value,
}

impl<'a, E> JsonCtx<'a, E> {
    pub fn inner_error(&self) -> &E {
        &self.inner_error
    }
    pub fn json_ctx(&self) -> &serde_json::Value {
        self.json_ctx
    }
    pub fn into_inner(self) -> (E, &'a serde_json::Value) {
        (self.inner_error, self.json_ctx)
    }
    pub fn map_err<F>(self, m: impl FnOnce(E) -> F) -> JsonCtx<'a, F> {
        JsonCtx {
            inner_error: m(self.inner_error),
            json_ctx: self.json_ctx,
        }
    }
}

#[allow(unused)]
/// Lifts a function that returns a `Result` given an `&serde_json::Value`
/// into a function that includes the json value into the error details
pub fn json_ctx_lift<T, E>(
    func: impl for<'a> FnOnce(&'a serde_json::Value) -> Result<T, E>,
) -> impl for<'a> FnOnce(&'a serde_json::Value) -> Result<T, JsonCtx<'a, E>> {
    |value| {
        let res = func(value);
        res.map_err(|err| JsonCtx {
            inner_error: err,
            json_ctx: value,
        })
    }
}

pub trait WithJsonContextExt<'a> {
    type T;
    type E;
    fn with_ctx(self, value: &'a serde_json::Value) -> Result<Self::T, JsonCtx<'a, Self::E>>;
    // fn without_ctx(self, value: &'a serde_json::Value) -> Result<Self::T, Self::E>;
}
impl<'a, F, T, E> WithJsonContextExt<'a> for F
where
    F: FnOnce(&'a serde_json::Value) -> Result<T, E>,
{
    type T = T;
    type E = E;
    fn with_ctx(self, value: &'a serde_json::Value) -> Result<Self::T, JsonCtx<'a, Self::E>> {
        let res = self(value);
        res.map_err(|err| JsonCtx {
            inner_error: err,
            json_ctx: value,
        })
    }
    /*fn without_ctx(self, value: &'a serde_json::Value) -> Result<Self::T, Self::E> {
        self(value)
    }*/
}

impl<'a, T, E> WithJsonContextExt<'a> for Result<T, E> {
    type T = T;
    type E = E;
    fn with_ctx(self, json: &'a serde_json::Value) -> Result<T, JsonCtx<'a, E>> {
        self.map_err(|err| JsonCtx {
            inner_error: err,
            json_ctx: json,
        })
    }
    /*fn without_ctx(self, _value: &'a serde_json::Value) -> Result<Self::T, Self::E> {
        self
    }*/
}
