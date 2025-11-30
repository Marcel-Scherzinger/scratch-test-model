#[cfg(not(feature = "thread-share"))]
/// This type alias is – configured by a feature flag – either [std::rc::Rc] or [std::sync::Arc]
pub type ARc<T> = std::rc::Rc<T>;

#[cfg(feature = "thread-share")]
/// This type alias is – configured by a feature flag – either [std::rc::Rc] or [std::sync::Arc]
pub type ARc<T> = std::sync::Arc<T>;
