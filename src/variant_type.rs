use byteorder::ByteOrder;
use std::{error, fmt, str};

use crate::utils::padding_for_n_bytes;
use crate::SharedData;
use crate::{Array, DictEntry, ObjectPath, Signature, Structure};
use crate::{SimpleDecode, Variant, VariantTypeConstants};

#[derive(Debug)]
pub enum VariantError {
    ExcessData,
    IncorrectType,
    IncorrectValue,
    InvalidUtf8,
    InsufficientData,
    UnsupportedType(Signature),
}

impl error::Error for VariantError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantError::ExcessData => write!(f, "excess data"),
            VariantError::IncorrectType => write!(f, "incorrect type"),
            VariantError::IncorrectValue => write!(f, "incorrect value"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType(s) => {
                write!(f, "unsupported type (signature: \"{}\")", s.as_str())
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EncodingFormat {
    DBus,
    // TODO: GVariant
}

impl Default for EncodingFormat {
    fn default() -> Self {
        EncodingFormat::DBus
    }
}

pub trait Encode: std::fmt::Debug {
    fn signature_char() -> char
    where
        Self: Sized;
    fn signature_str() -> &'static str
    where
        Self: Sized;
    fn alignment() -> usize
    where
        Self: Sized;

    // Only use for the first data in a message
    fn encode(&self, format: EncodingFormat) -> Vec<u8> {
        let mut bytes = vec![];
        self.encode_into(&mut bytes, format);

        bytes
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat);

    fn signature(&self) -> Signature
    where
        Self: Sized,
    {
        Signature::new(Self::signature_str())
    }

    fn add_padding(bytes: &mut Vec<u8>, format: EncodingFormat)
    where
        Self: Sized,
    {
        let padding = Self::padding(bytes.len(), format);
        if padding > 0 {
            bytes.resize(bytes.len() + padding, 0);
        }
    }

    fn padding(n_bytes_before: usize, _format: EncodingFormat) -> usize
    where
        Self: Sized,
    {
        padding_for_n_bytes(n_bytes_before, Self::alignment())
    }

    // Into<Variant> trait bound would have been better and it's possible but since `Into<T> for T`
    // is provided implicitly, the default no-op implementation for `Variant` won't do the right
    // thing: unflatten it.
    // `TryFrom<Variant>`.
    fn to_variant(self) -> Variant
    where
        Self: Sized;
}

pub trait Decode: Encode + std::fmt::Debug {
    // Default implementation works for constant-sized types where size is the same as their
    // alignment
    fn slice_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(data.position(), format);
        let len = Self::alignment() + padding;
        ensure_sufficient_bytes(data.bytes(), len)?;

        Ok(data.subset(0, len))
    }

    fn ensure_correct_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError>
    where
        Self: Sized,
    {
        let signature = signature.into();

        if signature != Self::signature_str() {
            return Err(VariantError::IncorrectType);
        }

        Ok(signature)
    }

    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError>
    where
        Self: Sized;

    fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError>
    where
        Self: Sized,
    {
        let slice: Signature = signature.into()[0..1].into();

        Self::ensure_correct_signature(slice)
    }

    // Mostly a helper for decode() implementation. Removes any leading padding bytes.
    fn slice_for_decoding(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(data.position(), format);
        let len = Self::alignment() + padding;
        ensure_sufficient_bytes(data.bytes(), len)?;

        Ok(data.tail(padding))
    }

    /// Checks if variant value is of the generic type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbus::{Encode, Decode};
    ///
    /// let v = String::from("hello").to_variant();
    /// assert!(!u32::is(&v));
    /// assert!(String::is(&v));
    /// ```
    ///
    /// ```
    /// use zbus::{Encode, Decode};
    ///
    /// let v = 147u32.to_variant();
    /// assert!(u32::is(&v));
    /// assert!(!String::is(&v));
    /// ```
    fn is(variant: &Variant) -> bool
    where
        Self: Sized;

    // `TryFrom<Variant>` trait bound would have been better but we can't use that unfortunately
    // since Variant implements Decode.
    fn take_from_variant(variant: Variant) -> Result<Self, VariantError>
    where
        Self: Sized;

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError>
    where
        Self: Sized;
}

impl Encode for u8 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U8(self)
    }
}

impl Decode for u8 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(slice.bytes()[0])
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U8(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U8(u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U8(ref u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for bool {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&(*self as u32).to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::Bool(self)
    }
}

impl Decode for bool {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        match byteorder::NativeEndian::read_u32(slice.bytes()) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(VariantError::IncorrectValue),
        }
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Bool(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Bool(u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Bool(u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for i16 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I16(self)
    }
}

impl Decode for i16 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_i16(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::I16(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::I16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::I16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for u16 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U16(self)
    }
}

impl Decode for u16 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_u16(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U16(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for i32 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I32(self)
    }
}

impl Decode for i32 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_i32(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::I32(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::I32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::I32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for u32 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U32(self)
    }
}

impl Decode for u32 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_u32(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U32(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for i64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I64(self)
    }
}

impl Decode for i64 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_i64(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::I64(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::I64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::I64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for u64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U64(self)
    }
}

impl Decode for u64 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_u64(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U64(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Encode for f64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        let mut buf = [0; 8];
        byteorder::NativeEndian::write_f64(&mut buf, *self);
        bytes.extend_from_slice(&buf);
    }

    fn to_variant(self) -> Variant {
        Variant::F64(self)
    }
}

impl Decode for f64 {
    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_f64(slice.bytes()))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::F64(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::F64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::F64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

pub(crate) fn ensure_sufficient_bytes(bytes: &[u8], size: usize) -> Result<(), VariantError> {
    if bytes.len() < size {
        return Err(VariantError::InsufficientData);
    }

    Ok(())
}

pub(crate) fn slice_data(
    data: &SharedData,
    signature: impl Into<Signature>,
    format: EncodingFormat,
) -> Result<SharedData, VariantError> {
    let signature = signature.into();

    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::slice_data_simple(data, format),
        bool::SIGNATURE_CHAR => bool::slice_data_simple(data, format),
        i16::SIGNATURE_CHAR => i16::slice_data_simple(data, format),
        u16::SIGNATURE_CHAR => u16::slice_data_simple(data, format),
        i32::SIGNATURE_CHAR => i32::slice_data_simple(data, format),
        u32::SIGNATURE_CHAR => u32::slice_data_simple(data, format),
        i64::SIGNATURE_CHAR => i64::slice_data_simple(data, format),
        u64::SIGNATURE_CHAR => u64::slice_data_simple(data, format),
        f64::SIGNATURE_CHAR => f64::slice_data_simple(data, format),
        String::SIGNATURE_CHAR => String::slice_data_simple(data, format),
        Array::SIGNATURE_CHAR => Array::slice_data(data, signature, format),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::slice_data_simple(data, format),
        Signature::SIGNATURE_CHAR => Signature::slice_data_simple(data, format),
        Structure::SIGNATURE_CHAR => Structure::slice_data(data, signature, format),
        Variant::SIGNATURE_CHAR => Variant::slice_data(data, signature, format),
        DictEntry::SIGNATURE_CHAR => DictEntry::slice_data(data, signature, format),
        _ => return Err(VariantError::UnsupportedType(signature)),
    }
}

pub(crate) fn padding_for_signature(
    n_bytes_before: usize,
    signature: impl Into<Signature>,
    format: EncodingFormat,
) -> usize {
    let signature = signature.into();

    match signature.chars().next().unwrap_or('\0') {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::padding(n_bytes_before, format),
        bool::SIGNATURE_CHAR => bool::padding(n_bytes_before, format),
        i16::SIGNATURE_CHAR => i16::padding(n_bytes_before, format),
        u16::SIGNATURE_CHAR => u16::padding(n_bytes_before, format),
        i32::SIGNATURE_CHAR => i32::padding(n_bytes_before, format),
        u32::SIGNATURE_CHAR => u32::padding(n_bytes_before, format),
        i64::SIGNATURE_CHAR => i64::padding(n_bytes_before, format),
        u64::SIGNATURE_CHAR => u64::padding(n_bytes_before, format),
        f64::SIGNATURE_CHAR => f64::padding(n_bytes_before, format),
        String::SIGNATURE_CHAR => String::padding(n_bytes_before, format),
        Array::SIGNATURE_CHAR => Array::padding(n_bytes_before, format),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::padding(n_bytes_before, format),
        Signature::SIGNATURE_CHAR => Signature::padding(n_bytes_before, format),
        Structure::SIGNATURE_CHAR => Structure::padding(n_bytes_before, format),
        Variant::SIGNATURE_CHAR => Variant::padding(n_bytes_before, format),
        DictEntry::SIGNATURE_CHAR => DictEntry::padding(n_bytes_before, format),
        _ => {
            println!("WARNING: Unsupported signature: {}", signature.as_str());

            0
        }
    }
}

pub(crate) fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
    let signature = signature.into();

    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::slice_signature(signature),
        bool::SIGNATURE_CHAR => bool::slice_signature(signature),
        i16::SIGNATURE_CHAR => i16::slice_signature(signature),
        u16::SIGNATURE_CHAR => u16::slice_signature(signature),
        i32::SIGNATURE_CHAR => i32::slice_signature(signature),
        u32::SIGNATURE_CHAR => u32::slice_signature(signature),
        i64::SIGNATURE_CHAR => i64::slice_signature(signature),
        u64::SIGNATURE_CHAR => u64::slice_signature(signature),
        f64::SIGNATURE_CHAR => f64::slice_signature(signature),
        String::SIGNATURE_CHAR => String::slice_signature(signature),
        Array::SIGNATURE_CHAR => Array::slice_signature(signature),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::slice_signature(signature),
        Signature::SIGNATURE_CHAR => Signature::slice_signature(signature),
        Structure::SIGNATURE_CHAR => Structure::slice_signature(signature),
        Variant::SIGNATURE_CHAR => Variant::slice_signature(signature),
        DictEntry::SIGNATURE_CHAR => DictEntry::slice_signature(signature),
        _ => return Err(VariantError::UnsupportedType(signature)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encode, EncodingFormat, SharedData, SimpleDecode};

    // Ensure Encode can be used as Boxed type
    #[test]
    fn trait_object() {
        let boxed = Box::new(42u8);

        let format = EncodingFormat::default();
        let encoded = SharedData::new(encode_u8(boxed, format));
        assert!(u8::decode_simple(&encoded, format).unwrap() == 42u8);
    }

    fn encode_u8(boxed: Box<dyn Encode>, format: EncodingFormat) -> Vec<u8> {
        boxed.encode(format)
    }
}
