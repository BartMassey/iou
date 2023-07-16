//! Type for lazily initializing a value on use. [Iou] is
//! similar to `std::borrow::Cow`: instead of taking
//! ownership when first written, [Iou] initializes into a
//! owned value when first referenced.
//!
//! [Iou] is useful in cases where initialization is
//! expensive or time consuming, and the price is better
//! paid later.
//!
//! An [Iou] will have a "corrupted cell" if its initialization
//! function panics during initialization. Operations on an [Iou]
//! with a corrupted cell will themselves panic.

use std::cell::{RefCell, Ref, RefMut};

/// Initialize on use: a value that will be lazily
/// initialized at first reference.
pub struct Iou<S, F, T>(RefCell<IouState<S, F, T>>);

enum IouState<S, F, T> {
    /// Not yet initialized.
    PreInit(Option<(S, F)>),
    /// Initialized.
    Init(T),
}

impl<S, F, T> Iou<S, F, T> {
    /// Create a new [Iou] that will be initialized on first
    /// use by applying the function `f` to the
    /// initialization data `init`.
    pub fn new(init: S, f: F) -> Self {
        Iou(RefCell::new(IouState::PreInit(Some((init, f)))))
    }
}

impl<S, F, T> Iou<S, F, T>
    where F: FnOnce(S) -> T
{
    /// Initialize the [Iou] if needed and return the
    /// initialized value, consuming the [Iou].
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn unwrap(self) -> T {
        match self.0.into_inner() {
            IouState::PreInit(Some((s, f))) => f(s),
            IouState::Init(t) => t,
            _ => panic!("Iou: corrupted cell"),
        }
    }

    /// Check whether the value has been initialized yet.
    pub fn is_init(&self) -> bool {
        matches!(&*self.0.borrow(), IouState::Init(_))
    }

    /// Initialize the [Iou] if not yet initialized.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn init(&self) {
        if self.is_init() {
            return;
        }
        let mut iou = self.0.borrow_mut();
        if let IouState::PreInit(p) = &mut *iou {
            let (s, f) = p.take().expect("Iou: corrupted cell");
            *iou = IouState::Init(f(s));
        }
    }

    /// Initialize the [Iou] if not yet initialized, then
    /// return a reference to the initialized value.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.init();
        Ref::map(
            self.0.borrow(),
            |s| {
                match s {
                    IouState::Init(t) => t,
                    _ => panic!("Iou: corrupted cell"),
                }
            },
        )
    }

    /// Initialize the [Iou] if not yet initialized, then
    /// return a mutable reference to the initialized value.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.init();
        RefMut::map(
            self.0.borrow_mut(),
            |s| {
                match s {
                    IouState::Init(t) => t,
                    _ => panic!("Iou: corrupted cell"),
                }
            },
        )
    }
}
