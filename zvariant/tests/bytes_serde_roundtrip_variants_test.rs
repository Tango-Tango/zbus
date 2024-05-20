use endi::Endian;
use serde::{Deserialize, Serialize};
use zvariant::{
    serialized::{Context, Data},
    Array, ObjectPath, OwnedValue, Type, Value,
};

fn generate_contexts() -> Vec<Context> {
    vec![
        Context::new_dbus(Endian::Little, 0),
        Context::new_dbus(Endian::Big, 0),
        // NOTE: Behavior isn't supported for gvariant
        // encoding right now.
    ]
}

#[test]
fn serde_u8() {
    for context in generate_contexts() {
        // Check u8 -> variant -> Value
        {
            let value: u8 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::U8(42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> u8
        {
            let value = Value::U8(42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (u8, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }

        // Check u8 -> variant -> u8
        {
            let value: u8 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (u8, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_u16() {
    for context in generate_contexts() {
        // Check u16 -> variant -> Value
        {
            let value: u16 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::U16(42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> u16
        {
            let value = Value::U16(42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (u16, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }

        // Check u16 -> variant -> u16
        {
            let value: u16 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (u16, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_u32() {
    for context in generate_contexts() {
        // Check u32 -> variant -> Value
        {
            let value: u32 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::U32(42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> u32
        {
            let value = Value::U32(42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (u32, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }

        // Check u32 -> variant -> u32
        {
            let value: u32 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (u32, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_u64() {
    for context in generate_contexts() {
        // Check u64 -> variant -> Value
        {
            let value: u64 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::U64(42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> u64
        {
            let value = Value::U64(42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (u64, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }

        // Check u64 -> variant -> u64
        {
            let value: u64 = 42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (u64, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_i16() {
    for context in generate_contexts() {
        // Check i16 -> variant -> Value
        {
            let value: i16 = -42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::I16(-42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> i16
        {
            let value = Value::I16(-42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (i16, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, -42);
            assert_eq!(decoded, serialized.len());
        }

        // Check i16 -> variant -> i16
        {
            let value: i16 = -42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (i16, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, -42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_i32() {
    for context in generate_contexts() {
        // Check i32 -> variant -> Value
        {
            let value: i32 = -42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::I32(-42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> i32
        {
            let value = Value::I32(-42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (i32, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, -42);
            assert_eq!(decoded, serialized.len());
        }

        // Check i32 -> variant -> i32
        {
            let value: i32 = -42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (i32, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, -42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_i64() {
    for context in generate_contexts() {
        // Check i64 -> variant -> Value
        {
            let value: i64 = -42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::I64(-42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> i64
        {
            let value = Value::I64(-42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (i64, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, -42);
            assert_eq!(decoded, serialized.len());
        }

        // Check i64 -> variant -> i64
        {
            let value: i64 = -42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (i64, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, -42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_f64() {
    for context in generate_contexts() {
        // Check f64 -> variant -> Value
        {
            let value: f64 = 42.42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::F64(42.42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> f64
        {
            let value = Value::F64(42.42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (f64, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42.42);
            assert_eq!(decoded, serialized.len());
        }

        // Check f64 -> variant -> f64
        {
            let value: f64 = 42.42;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (f64, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, 42.42);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_string() {
    for context in generate_contexts() {
        // Check String -> variant -> Value
        {
            let value: String = "Hello, world!".to_string();
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(
                deserialized,
                Value::Str("Hello, world!".into()).try_to_owned().unwrap()
            );
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> String
        {
            let value = Value::Str("Hello, world!".into());
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (String, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, "Hello, world!".to_string());
            assert_eq!(decoded, serialized.len());
        }

        // Check String -> variant -> String
        {
            let value: String = "Hello, world!".to_string();
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (String, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, "Hello, world!".to_string());
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
#[ignore]
fn serde_object_path() {
    for context in generate_contexts() {
        // Check ObjectPath -> variant -> Value
        {
            let value: zvariant::ObjectPath =
                ObjectPath::from_static_str_unchecked("/org/freedesktop/DBus");
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(
                deserialized,
                Value::ObjectPath(value).try_to_owned().unwrap()
            );
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> ObjectPath
        {
            let path: zvariant::ObjectPath =
                ObjectPath::from_static_str_unchecked("/org/freedesktop/DBus");
            let value = Value::ObjectPath(path.clone());
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (zvariant::ObjectPath, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, path);
            assert_eq!(decoded, serialized.len());
        }

        // Check ObjectPath -> variant -> ObjectPath
        {
            let value: zvariant::ObjectPath =
                ObjectPath::from_static_str_unchecked("/org/freedesktop/DBus");
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (zvariant::ObjectPath, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, value);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
#[ignore]
fn serde_signature() {
    for context in generate_contexts() {
        // Check Signature -> variant -> Value
        {
            let value: zvariant::Signature =
                zvariant::Signature::from_static_str_unchecked("a{sv}");
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(
                deserialized,
                Value::Signature(value).try_to_owned().unwrap()
            );
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> Signature
        {
            let signature: zvariant::Signature =
                zvariant::Signature::from_static_str_unchecked("a{sv}");
            let value = Value::Signature(signature.clone());
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (zvariant::Signature, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, signature);
            assert_eq!(decoded, serialized.len());
        }

        // Check Signature -> variant -> Signature
        {
            let value: zvariant::Signature =
                zvariant::Signature::from_static_str_unchecked("a{sv}");
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (zvariant::Signature, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, value);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_unit_variant() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    enum UnitVariant {
        A,
        B,
    }

    for context in generate_contexts() {
        // Check () -> variant -> Value
        {
            let value: UnitVariant = UnitVariant::B;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::U32(1).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> ()
        {
            let value = Value::U32(1);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (UnitVariant, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, UnitVariant::B);
            assert_eq!(decoded, serialized.len());
        }

        // Check () -> variant -> ()
        {
            let value: UnitVariant = UnitVariant::B;
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (UnitVariant, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, UnitVariant::B);
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
fn serde_newtype_struct() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    struct NewtypeStruct(i32);

    for context in generate_contexts() {
        // Check NewtypeStruct -> variant -> Value
        {
            let value: NewtypeStruct = NewtypeStruct(42);
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(deserialized, Value::I32(42).try_to_owned().unwrap());
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> NewtypeStruct
        {
            let value = Value::I32(42);
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (NewtypeStruct, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, NewtypeStruct(42));
            assert_eq!(decoded, serialized.len());
        }

        // Check NewtypeStruct -> variant -> NewtypeStruct
        {
            let value: NewtypeStruct = NewtypeStruct(42);
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (NewtypeStruct, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, NewtypeStruct(42));
            assert_eq!(decoded, serialized.len());
        }
    }
}

#[test]
#[ignore]
fn serde_seq() {
    for context in generate_contexts() {
        // Check Vec<u8> -> variant -> Value
        {
            let value: Vec<u8> = vec![1, 2, 3, 4, 5];
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (OwnedValue, usize) = serialized.deserialize().unwrap();
            assert_eq!(
                deserialized,
                Value::Array(Array::from(vec![1, 2, 3, 4, 5]))
                    .try_to_owned()
                    .unwrap()
            );
            assert_eq!(decoded, serialized.len());
        }

        // Check Value -> variant -> Vec<u8>
        {
            let value = Value::Array(Array::from(vec![1, 2, 3, 4, 5]));
            let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
            let (deserialized, decoded): (Vec<u8>, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, vec![1, 2, 3, 4, 5]);
            assert_eq!(decoded, serialized.len());
        }

        // Check Vec<u8> -> variant -> Vec<u8>
        {
            let value: Vec<u8> = vec![1, 2, 3, 4, 5];
            let serialized: Data<'_, '_> =
                zvariant::to_bytes_for_signature(context, "v", &value).unwrap();
            let (deserialized, decoded): (Vec<u8>, usize) =
                serialized.deserialize_for_signature("v").unwrap();
            assert_eq!(deserialized, vec![1, 2, 3, 4, 5]);
            assert_eq!(decoded, serialized.len());
        }
    }
}
