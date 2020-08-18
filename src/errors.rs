use failure::Error;
/// Supererror type for SafeArray element conversion errors
#[derive(Debug, Fail)]
pub enum ElementError {
    /// Holds FromSafeArrElemErrors in a box
    #[fail(display = "{}", _0)]
    From(Box<FromSafeArrElemError>),
    /// Holds IntoSafeArrElemErrors in a box
    #[fail(display = "{}", _0)]
    Into(Box<IntoSafeArrElemError>),
}

impl From<FromSafeArrElemError> for ElementError {
    /// Holds a [`FromSafeArrElemError`] with a box. This means conversion is free.
    fn from(fsaee: FromSafeArrElemError) -> ElementError {
        ElementError::From(Box::new(fsaee))
    }
}

impl From<IntoSafeArrElemError> for ElementError {
    /// Holds a [`IntoSafeArrElemError`] with a box. This means conversion is free.
    fn from(isaee: IntoSafeArrElemError) -> ElementError {
        ElementError::Into(Box::new(isaee))
    }
}

impl From<IntoVariantError> for ElementError {
    /// Uses From impls on [`IntoSafeArrElemError] and [`ElementError`] to convert the error.
    fn from(ive: IntoVariantError) -> Self {
        ElementError::from(IntoSafeArrElemError::from(ive))
    }
}

impl From<FromVariantError> for ElementError {
    /// Uses From impls on [`FromSafeArrElemError] and [`ElementError`] to convert the error.
    fn from(fve: FromVariantError) -> Self {
        ElementError::from(FromSafeArrElemError::from(fve))
    }
}

/// Errors for converting from C/C++ data structure to Rust types
#[derive(Debug, Fail)]
pub enum FromSafeArrElemError {
    /// The unsafe call to SafeArrayGetElement failed - HRESULT stored within tells why
    #[fail(display = "SafeArrayGetElement failed with HRESULT=0x{:x}", hr)]
    GetElementFailed {
        /// Holds an HRESULT value
        hr: i32,
    },
    /// Holds a [`BStringError`] in a box.
    #[fail(display = "{}", _0)]
    BStringFailed(Box<BStringError>),
    /// [`from_variant`] failed somehow. Error is stored in a box.
    #[fail(display = "from variant failure: {}", _0)]
    FromVarError(Box<FromVariantError>),
}

impl From<FromVariantError> for FromSafeArrElemError {
    /// Boxes a [`FromVariantError`] into a [`FromSafeArrElemError`] which means the conversion is free.
    fn from(fve: FromVariantError) -> FromSafeArrElemError {
        FromSafeArrElemError::FromVarError(Box::new(fve))
    }
}

/// Errors for converting into C/C++ data structures from Rust types
#[derive(Debug, Fail)]
pub enum IntoSafeArrElemError {
    /// `SysAllocStringLen` failed with len
    #[fail(display = "{}", _0)]
    BStringFailed(Box<BStringError>),
    /// `SafeArrayPutElement` failed with `HRESULT`
    #[fail(display = "SafeArrayPutElement failed with HRESULT = 0x{:x}", hr)]
    PutElementFailed {
        /// HRESULT returned by SafeArrayPutElement call
        hr: i32,
    },
    /// Encapsulates a `IntoVariantError`
    #[fail(display = "IntoVariantError: {}", _0)]
    IntoVariantError(Box<IntoVariantError>),
}

impl From<IntoVariantError> for IntoSafeArrElemError {
    /// Boxes an [`IntoVariantError`] into an [`IntoSafeArrElemError`]
    fn from(ive: IntoVariantError) -> IntoSafeArrElemError {
        IntoSafeArrElemError::IntoVariantError(Box::new(ive))
    }
}

impl From<BStringError> for IntoSafeArrElemError {
    /// Boxes a [`BStringError`] into an [`IntoSafeArrElemError`]. This means the conversion is free.
    fn from(bse: BStringError) -> IntoSafeArrElemError {
        IntoSafeArrElemError::BStringFailed(Box::new(bse))
    }
}

/// Supererror for SafeArray errors
#[derive(Debug, Fail)]
pub enum SafeArrayError {
    /// From wrapper for `FromSafeArrayError`
    #[fail(display = "{}", _0)]
    From(Box<FromSafeArrayError>),
    /// Into wrapper for `IntoSafeArrayError`
    #[fail(display = "{}", _0)]
    Into(Box<IntoSafeArrayError>),
}

impl From<FromSafeArrayError> for SafeArrayError {
    /// Holds a [`FromSafeArrayError`] with a box. This means conversion is free.
    fn from(fsae: FromSafeArrayError) -> SafeArrayError {
        SafeArrayError::From(Box::new(fsae))
    }
}

impl From<IntoSafeArrayError> for SafeArrayError {
    /// Holds a [`IntoSafeArrayError`] with a box. This means conversion is free.
    fn from(isae: IntoSafeArrayError) -> SafeArrayError {
        SafeArrayError::Into(Box::new(isae))
    }
}

impl From<FromVariantError> for SafeArrayError {
    /// Uses From impls on [`FromSafeArrayError] and [`SafeArrayError`] to convert the error.
    fn from(fve: FromVariantError) -> Self {
        SafeArrayError::from(FromSafeArrayError::from(fve))
    }
}

/// Represents the different ways converting from `SAFEARRAY` can fail
#[derive(Debug, Fail)]
pub enum FromSafeArrayError {
    /// Either the safe array dimensions = 0 or > 1
    /// multi-dimensional arrays are *not* handled.
    #[fail(display = "Safe array dimensions are invalid: {}", sa_dims)]
    SafeArrayDimsInvalid {
        /// safe array dimensions that was wrong
        sa_dims: u32,
    },
    /// Expected vartype did not match found vartype - runtime consistency check
    #[fail(
        display = "expected vartype was not found - expected: {} - found: {}",
        expected, found
    )]
    VarTypeDoesNotMatch {
        /// The expected vartype
        expected: u32,
        /// the found vartype
        found: u32,
    },
    /// Call to SafeArrayGetLBound failed
    #[fail(display = "SafeArrayGetLBound failed with HRESULT = 0x{:x}", hr)]
    SafeArrayLBoundFailed {
        /// HRESULT returned
        hr: i32,
    },
    /// Call to SafeArrayGetRBound failed
    #[fail(display = "SafeArrayGetRBound failed with HRESULT = 0x{:x}", hr)]
    SafeArrayRBoundFailed {
        /// HRESULT returned
        hr: i32,
    },
    /// Call to SafeArrayGetVartype failed
    #[fail(display = "SafeArrayGetVartype failed with HRESULT = 0x{:x}", hr)]
    SafeArrayGetVartypeFailed {
        /// HRESULT returned
        hr: i32,
    },
    /// Encapsulates the `ElementError` that occurred during conversion
    #[fail(
        display = "element conversion failed at index {} with {}",
        index, element
    )]
    ElementConversionFailed {
        /// the index the conversion failed at
        index: usize,
        /// The element error encapsulating the failure
        element: Box<ElementError>,
    },
    /// [`from_variant`] call failed
    #[fail(display = "from variant failure: {}", _0)]
    FromVariantError(Box<FromVariantError>),
}

impl From<FromVariantError> for FromSafeArrayError {
    /// Boxes a [`FromVariantError`] into a [`FromSafeArrElemError`] which means the conversion is free.
    fn from(fve: FromVariantError) -> Self {
        FromSafeArrayError::FromVariantError(Box::new(fve))
    }
}

impl FromSafeArrayError {
    /// Boxes an [`ElementError`] into a [`FromSafeArrayError`].
    ///
    /// Need the index so a From impl doesn't apply.
    pub fn from_element_err<E: Into<ElementError>>(ee: E, index: usize) -> FromSafeArrayError {
        FromSafeArrayError::ElementConversionFailed {
            index: index,
            element: Box::new(ee.into()),
        }
    }
}

/// Represents the different ways converting into `SAFEARRAY` can fail
#[derive(Debug, Fail)]
pub enum IntoSafeArrayError {
    /// Encapsulates the [`ElementError`] that occurred during conversion
    #[fail(
        display = "element conversion failed at index {} with {}",
        index, element
    )]
    ElementConversionFailed {
        /// the index the conversion failed at
        index: usize,
        /// The element error encapsulating the failure
        element: Box<ElementError>,
    },
    /// into_variant call failed
    #[fail(display = "into variant failure: {}", _0)]
    IntoVariantError(Box<IntoVariantError>),
}

impl From<IntoVariantError> for IntoSafeArrayError {
    /// Boxes a [`FromVariantError`] into a [`FromSafeArrElemError`] which means the conversion is free.
    fn from(ive: IntoVariantError) -> Self {
        IntoSafeArrayError::IntoVariantError(Box::new(ive))
    }
}

impl IntoSafeArrayError {
    /// Boxes an [`ElementError`] into a [`IntoSafeArrayError`].
    ///
    /// Need the index so a From impl doesn't apply.
    pub fn from_element_err<E: Into<ElementError>>(ee: E, index: usize) -> IntoSafeArrayError {
        IntoSafeArrayError::ElementConversionFailed {
            index: index,
            element: Box::new(ee.into()),
        }
    }
}

/// Ways BString can fail. Currently just one way.
#[derive(Clone, Copy, Debug, Fail)]
pub enum BStringError {
    /// SysAllocStringLen failed
    #[fail(display = "BSTR allocation failed for len: {}", len)]
    AllocateFailed {
        /// The len which was used for allocation
        len: usize,
    },
}

/// Encapsulates the ways converting from a `VARIANT` can fail.
#[derive(Copy, Clone, Debug, Fail)]
pub enum FromVariantError {
    /// `VARIANT` pointer during conversion was null
    #[fail(display = "VARIANT pointer is null")]
    VariantPtrNull,
    /// Unknown VT for
    #[fail(display = "Variants does not support this vartype: {:p}", _0)]
    UnknownVarType(u16),
}

/// Encapsulates errors that can occur during conversion into VARIANT
#[derive(Debug, Fail)]
pub enum IntoVariantError {
    /// Encapsulates a `BStringError`
    #[fail(display = "{}", _0)]
    AllocBStrFailed(Box<BStringError>),
    /// Encapsulates a `SafeArrayError`
    #[fail(display = "SafeArray conversion failed: {}", _0)]
    SafeArrConvFailed(Box<SafeArrayError>),
    ///
    #[fail(display = "Can't convert &dyn CVariantWrappers into Ptr<VARIANTS>")]
    CVarWrapper,
}

impl From<BStringError> for IntoVariantError {
    /// Boxes a [`BStringError`] into a [`FromSafeArrElemError`]. This means the conversion is free.
    fn from(bse: BStringError) -> IntoVariantError {
        IntoVariantError::AllocBStrFailed(Box::new(bse))
    }
}

/// Errors which can arise primarily from using `Conversion::convert` calls
#[derive(Debug, Fail)]
pub enum ConversionError {
    /// Ptr being used was null
    #[fail(display = "pointer was null")]
    PtrWasNull,
    /// General purpose holder of `failure::Error` values
    #[fail(display = "{}", _0)]
    General(Box<Error>),
}
