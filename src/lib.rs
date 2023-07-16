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

use core::cell::UnsafeCell;

/// Initialize on use: a value that will be lazily
/// initialized at first reference.
pub struct Iou<S, F, T>(UnsafeCell<IouState<S, F, T>>);

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
        Iou(UnsafeCell::new(IouState::PreInit(Some((init, f)))))
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

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_ref(&self) -> &IouState<S, F, T> {
        UnsafeCell::raw_get(&self.0).as_ref().unwrap()
    }

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut_ref(&self) -> &mut IouState<S, F, T> {
        UnsafeCell::raw_get(&self.0).as_mut().unwrap()
    }

    /// Check whether the value has been initialized yet.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn is_init(&self) -> bool {
        // Safety: We are only reading the value, which
        // cannot be mutated while we are checking.
        unsafe {
            let contents = self.get_ref();
            if matches!(contents, IouState::PreInit(None)) {
                panic!("Iou: corrupted cell");
            }
            matches!(contents, IouState::Init(_))
        }
    }

    /// Initialize the [Iou] if not yet initialized.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn init(&self) {
        // Safety: This code can only panic:
        //
        // * During the `take()`, which is harmless as the
        // value has not been altered.
        //
        // * During execution of `f()`, which leaves the
        // value in a "corrupted cell" state that will be
        // caught by future operations.
        unsafe { 
            let contents = self.get_mut_ref();
            match contents {
                IouState::PreInit(p) => {
                    let (s, f) = p.take().expect("init: corrupted cell");
                    *contents = IouState::Init(f(s));
                }
                IouState::Init(_) => (),
            }
        }
    }

    /// Initialize the [Iou] if not yet initialized, then
    /// return a reference to the initialized value.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    pub fn get(&self) -> &T {
        // Safety: At this point, the value will not be altered.
        // The lifetime of the returned reference is valid, because
        // this [Iou] owns its `Init` value and the [Iou] itself
        // cannot be replaced or moved out of.
        unsafe { 
            let contents = self.get_ref();
            match contents {
                IouState::Init(ref t) => t,
                _ => panic!("init: corrupted cell"),
            }
        }
    }

    /// Initialize the [Iou] if not yet initialized, then
    /// return a mutable reference to the initialized value.
    ///
    /// # Panics
    /// Panics on corrupted cell.
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        // Safety: At this point, the value will not be altered.
        // The lifetime of the returned reference is valid, because
        // this [Iou] owns its `Init` value and the [Iou] itself
        // cannot be replaced or moved out of.
        unsafe { 
            let contents = self.get_mut_ref();
            match contents {
                IouState::Init(ref mut t) => t,
                _ => panic!("init: corrupted cell"),
            }
        }
    }
}
