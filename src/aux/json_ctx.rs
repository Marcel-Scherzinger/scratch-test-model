/// Combines an error with a json value that serves as context.
///
/// Typically, this json context is the input the function, that produced the error, received.
#[derive(Debug, PartialEq, Clone, Hash, derive_more::Display)]
#[display("{inner_error}: {json_ctx}")]
pub struct JsonCtx<'a, E> {
    pub(crate) inner_error: E,
    pub(crate) json_ctx: &'a serde_json::Value,
}

impl<'a, E> JsonCtx<'a, E> {
    /// The wrapped error
    pub fn inner_error(&self) -> &E {
        &self.inner_error
    }
    /// The json value that is related to the error.
    /// Normally, this will be the input to the function that produced the error.
    pub fn json_ctx(&self) -> &serde_json::Value {
        self.json_ctx
    }
    /// This consumes the [`JsonCtx`] and returns its parts as tuple
    /// so that they can be used further.
    pub fn into_inner(self) -> (E, &'a serde_json::Value) {
        (self.inner_error, self.json_ctx)
    }
    /// Wraps the error in a closure. It doesn't change the context.
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

/// This is an awesome trick that allows augmenting different computations with
/// a contextual json value.
///
/// This trait is implemented for [`Result`] where it wraps the [`Result::Err`] value with context.
///
/// The more interesting implementation is the one for functions (and closures)
/// that take a reference to [`serde_json::Value`] and produce a result.
/// It allows _calling_ the method [`WithJsonContextExt::with_ctx`] on an unevaluated
/// function/method.
/// This enabled code like `SomeStruct::from_json.with_ctx(value)` where `from_json` is
/// a function transforming a json value reference into a `Result`.
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
