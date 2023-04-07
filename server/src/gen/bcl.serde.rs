// @generated
impl serde::Serialize for DateTime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        if self.scale != 0 {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bcl.DateTime", len)?;
        if self.value != 0 {
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if self.scale != 0 {
            let v = date_time::TimeSpanScale::from_i32(self.scale)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.scale)))?;
            struct_ser.serialize_field("scale", &v)?;
        }
        if self.kind != 0 {
            let v = date_time::DateTimeKind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DateTime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "scale",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Scale,
            Kind,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "scale" => Ok(GeneratedField::Scale),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DateTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bcl.DateTime")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DateTime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut scale__ = None;
                let mut kind__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scale => {
                            if scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            scale__ = Some(map.next_value::<date_time::TimeSpanScale>()? as i32);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map.next_value::<date_time::DateTimeKind>()? as i32);
                        }
                    }
                }
                Ok(DateTime {
                    value: value__.unwrap_or_default(),
                    scale: scale__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bcl.DateTime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for date_time::DateTimeKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Utc => "UTC",
            Self::Local => "LOCAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for date_time::DateTimeKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "UTC",
            "LOCAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = date_time::DateTimeKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(date_time::DateTimeKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(date_time::DateTimeKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(date_time::DateTimeKind::Unspecified),
                    "UTC" => Ok(date_time::DateTimeKind::Utc),
                    "LOCAL" => Ok(date_time::DateTimeKind::Local),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for date_time::TimeSpanScale {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Days => "DAYS",
            Self::Hours => "HOURS",
            Self::Minutes => "MINUTES",
            Self::Seconds => "SECONDS",
            Self::Milliseconds => "MILLISECONDS",
            Self::Ticks => "TICKS",
            Self::Minmax => "MINMAX",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for date_time::TimeSpanScale {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DAYS",
            "HOURS",
            "MINUTES",
            "SECONDS",
            "MILLISECONDS",
            "TICKS",
            "MINMAX",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = date_time::TimeSpanScale;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(date_time::TimeSpanScale::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(date_time::TimeSpanScale::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DAYS" => Ok(date_time::TimeSpanScale::Days),
                    "HOURS" => Ok(date_time::TimeSpanScale::Hours),
                    "MINUTES" => Ok(date_time::TimeSpanScale::Minutes),
                    "SECONDS" => Ok(date_time::TimeSpanScale::Seconds),
                    "MILLISECONDS" => Ok(date_time::TimeSpanScale::Milliseconds),
                    "TICKS" => Ok(date_time::TimeSpanScale::Ticks),
                    "MINMAX" => Ok(date_time::TimeSpanScale::Minmax),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Decimal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lo != 0 {
            len += 1;
        }
        if self.hi != 0 {
            len += 1;
        }
        if self.sign_scale != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bcl.Decimal", len)?;
        if self.lo != 0 {
            struct_ser.serialize_field("lo", ToString::to_string(&self.lo).as_str())?;
        }
        if self.hi != 0 {
            struct_ser.serialize_field("hi", &self.hi)?;
        }
        if self.sign_scale != 0 {
            struct_ser.serialize_field("signScale", &self.sign_scale)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decimal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lo",
            "hi",
            "signScale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lo,
            Hi,
            SignScale,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "lo" => Ok(GeneratedField::Lo),
                            "hi" => Ok(GeneratedField::Hi),
                            "signScale" => Ok(GeneratedField::SignScale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decimal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bcl.Decimal")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Decimal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lo__ = None;
                let mut hi__ = None;
                let mut sign_scale__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Lo => {
                            if lo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lo"));
                            }
                            lo__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hi => {
                            if hi__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hi"));
                            }
                            hi__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SignScale => {
                            if sign_scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signScale"));
                            }
                            sign_scale__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Decimal {
                    lo: lo__.unwrap_or_default(),
                    hi: hi__.unwrap_or_default(),
                    sign_scale: sign_scale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bcl.Decimal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Guid {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lo != 0 {
            len += 1;
        }
        if self.hi != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bcl.Guid", len)?;
        if self.lo != 0 {
            struct_ser.serialize_field("lo", ToString::to_string(&self.lo).as_str())?;
        }
        if self.hi != 0 {
            struct_ser.serialize_field("hi", ToString::to_string(&self.hi).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Guid {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lo",
            "hi",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lo,
            Hi,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "lo" => Ok(GeneratedField::Lo),
                            "hi" => Ok(GeneratedField::Hi),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Guid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bcl.Guid")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Guid, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lo__ = None;
                let mut hi__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Lo => {
                            if lo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lo"));
                            }
                            lo__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hi => {
                            if hi__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hi"));
                            }
                            hi__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Guid {
                    lo: lo__.unwrap_or_default(),
                    hi: hi__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bcl.Guid", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetObjectProxy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.existing_object_key != 0 {
            len += 1;
        }
        if self.new_object_key != 0 {
            len += 1;
        }
        if self.existing_type_key != 0 {
            len += 1;
        }
        if self.new_type_key != 0 {
            len += 1;
        }
        if !self.type_name.is_empty() {
            len += 1;
        }
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bcl.NetObjectProxy", len)?;
        if self.existing_object_key != 0 {
            struct_ser.serialize_field("existingObjectKey", &self.existing_object_key)?;
        }
        if self.new_object_key != 0 {
            struct_ser.serialize_field("newObjectKey", &self.new_object_key)?;
        }
        if self.existing_type_key != 0 {
            struct_ser.serialize_field("existingTypeKey", &self.existing_type_key)?;
        }
        if self.new_type_key != 0 {
            struct_ser.serialize_field("newTypeKey", &self.new_type_key)?;
        }
        if !self.type_name.is_empty() {
            struct_ser.serialize_field("typeName", &self.type_name)?;
        }
        if !self.payload.is_empty() {
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetObjectProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "existingObjectKey",
            "newObjectKey",
            "existingTypeKey",
            "newTypeKey",
            "typeName",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExistingObjectKey,
            NewObjectKey,
            ExistingTypeKey,
            NewTypeKey,
            TypeName,
            Payload,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "existingObjectKey" => Ok(GeneratedField::ExistingObjectKey),
                            "newObjectKey" => Ok(GeneratedField::NewObjectKey),
                            "existingTypeKey" => Ok(GeneratedField::ExistingTypeKey),
                            "newTypeKey" => Ok(GeneratedField::NewTypeKey),
                            "typeName" => Ok(GeneratedField::TypeName),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetObjectProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bcl.NetObjectProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NetObjectProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut existing_object_key__ = None;
                let mut new_object_key__ = None;
                let mut existing_type_key__ = None;
                let mut new_type_key__ = None;
                let mut type_name__ = None;
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExistingObjectKey => {
                            if existing_object_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("existingObjectKey"));
                            }
                            existing_object_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NewObjectKey => {
                            if new_object_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newObjectKey"));
                            }
                            new_object_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExistingTypeKey => {
                            if existing_type_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("existingTypeKey"));
                            }
                            existing_type_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NewTypeKey => {
                            if new_type_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newTypeKey"));
                            }
                            new_type_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(NetObjectProxy {
                    existing_object_key: existing_object_key__.unwrap_or_default(),
                    new_object_key: new_object_key__.unwrap_or_default(),
                    existing_type_key: existing_type_key__.unwrap_or_default(),
                    new_type_key: new_type_key__.unwrap_or_default(),
                    type_name: type_name__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bcl.NetObjectProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeSpan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        if self.scale != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bcl.TimeSpan", len)?;
        if self.value != 0 {
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if self.scale != 0 {
            let v = time_span::TimeSpanScale::from_i32(self.scale)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.scale)))?;
            struct_ser.serialize_field("scale", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeSpan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "scale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Scale,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "scale" => Ok(GeneratedField::Scale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeSpan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bcl.TimeSpan")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TimeSpan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut scale__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scale => {
                            if scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            scale__ = Some(map.next_value::<time_span::TimeSpanScale>()? as i32);
                        }
                    }
                }
                Ok(TimeSpan {
                    value: value__.unwrap_or_default(),
                    scale: scale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bcl.TimeSpan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for time_span::TimeSpanScale {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Days => "DAYS",
            Self::Hours => "HOURS",
            Self::Minutes => "MINUTES",
            Self::Seconds => "SECONDS",
            Self::Milliseconds => "MILLISECONDS",
            Self::Ticks => "TICKS",
            Self::Minmax => "MINMAX",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for time_span::TimeSpanScale {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DAYS",
            "HOURS",
            "MINUTES",
            "SECONDS",
            "MILLISECONDS",
            "TICKS",
            "MINMAX",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = time_span::TimeSpanScale;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(time_span::TimeSpanScale::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(time_span::TimeSpanScale::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DAYS" => Ok(time_span::TimeSpanScale::Days),
                    "HOURS" => Ok(time_span::TimeSpanScale::Hours),
                    "MINUTES" => Ok(time_span::TimeSpanScale::Minutes),
                    "SECONDS" => Ok(time_span::TimeSpanScale::Seconds),
                    "MILLISECONDS" => Ok(time_span::TimeSpanScale::Milliseconds),
                    "TICKS" => Ok(time_span::TimeSpanScale::Ticks),
                    "MINMAX" => Ok(time_span::TimeSpanScale::Minmax),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
