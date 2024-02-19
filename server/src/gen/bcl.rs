// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSpan {
    /// the size of the timespan (in units of the selected scale)
    #[prost(sint64, tag="1")]
    pub value: i64,
    /// the scale of the timespan \[default = DAYS\]
    #[prost(enumeration="time_span::TimeSpanScale", tag="2")]
    pub scale: i32,
}
/// Nested message and enum types in `TimeSpan`.
pub mod time_span {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeSpanScale {
        Days = 0,
        Hours = 1,
        Minutes = 2,
        Seconds = 3,
        Milliseconds = 4,
        Ticks = 5,
        /// dubious
        Minmax = 15,
    }
    impl TimeSpanScale {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeSpanScale::Days => "DAYS",
                TimeSpanScale::Hours => "HOURS",
                TimeSpanScale::Minutes => "MINUTES",
                TimeSpanScale::Seconds => "SECONDS",
                TimeSpanScale::Milliseconds => "MILLISECONDS",
                TimeSpanScale::Ticks => "TICKS",
                TimeSpanScale::Minmax => "MINMAX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAYS" => Some(Self::Days),
                "HOURS" => Some(Self::Hours),
                "MINUTES" => Some(Self::Minutes),
                "SECONDS" => Some(Self::Seconds),
                "MILLISECONDS" => Some(Self::Milliseconds),
                "TICKS" => Some(Self::Ticks),
                "MINMAX" => Some(Self::Minmax),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTime {
    /// the offset (in units of the selected scale) from 1970/01/01
    #[prost(sint64, tag="1")]
    pub value: i64,
    /// the scale of the timespan \[default = DAYS\]
    #[prost(enumeration="date_time::TimeSpanScale", tag="2")]
    pub scale: i32,
    /// the kind of date/time being represented \[default = UNSPECIFIED\]
    #[prost(enumeration="date_time::DateTimeKind", tag="3")]
    pub kind: i32,
}
/// Nested message and enum types in `DateTime`.
pub mod date_time {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeSpanScale {
        Days = 0,
        Hours = 1,
        Minutes = 2,
        Seconds = 3,
        Milliseconds = 4,
        Ticks = 5,
        /// dubious
        Minmax = 15,
    }
    impl TimeSpanScale {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeSpanScale::Days => "DAYS",
                TimeSpanScale::Hours => "HOURS",
                TimeSpanScale::Minutes => "MINUTES",
                TimeSpanScale::Seconds => "SECONDS",
                TimeSpanScale::Milliseconds => "MILLISECONDS",
                TimeSpanScale::Ticks => "TICKS",
                TimeSpanScale::Minmax => "MINMAX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAYS" => Some(Self::Days),
                "HOURS" => Some(Self::Hours),
                "MINUTES" => Some(Self::Minutes),
                "SECONDS" => Some(Self::Seconds),
                "MILLISECONDS" => Some(Self::Milliseconds),
                "TICKS" => Some(Self::Ticks),
                "MINMAX" => Some(Self::Minmax),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DateTimeKind {
        /// The time represented is not specified as either local time or Coordinated Universal Time (UTC).
        Unspecified = 0,
        /// The time represented is UTC.
        Utc = 1,
        /// The time represented is local time.
        Local = 2,
    }
    impl DateTimeKind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DateTimeKind::Unspecified => "UNSPECIFIED",
                DateTimeKind::Utc => "UTC",
                DateTimeKind::Local => "LOCAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "UTC" => Some(Self::Utc),
                "LOCAL" => Some(Self::Local),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetObjectProxy {
    /// for a tracked object, the key of the **first** time this object was seen
    #[prost(int32, tag="1")]
    pub existing_object_key: i32,
    /// for a tracked object, a **new** key, the first time this object is seen
    #[prost(int32, tag="2")]
    pub new_object_key: i32,
    /// for dynamic typing, the key of the **first** time this type was seen
    #[prost(int32, tag="3")]
    pub existing_type_key: i32,
    /// for dynamic typing, a **new** key, the first time this type is seen
    #[prost(int32, tag="4")]
    pub new_type_key: i32,
    /// for dynamic typing, the name of the type (only present along with newTypeKey)
    #[prost(string, tag="8")]
    pub type_name: ::prost::alloc::string::String,
    /// the new string/value (only present along with newObjectKey)
    #[prost(bytes="vec", tag="10")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Guid {
    /// the first 8 bytes of the guid (note:crazy-endian)
    #[prost(fixed64, tag="1")]
    pub lo: u64,
    /// the second 8 bytes of the guid (note:crazy-endian)
    #[prost(fixed64, tag="2")]
    pub hi: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decimal {
    /// the first 64 bits of the underlying value
    #[prost(uint64, tag="1")]
    pub lo: u64,
    /// the last 32 bis of the underlying value
    #[prost(uint32, tag="2")]
    pub hi: u32,
    /// the number of decimal digits (bits 1-16), and the sign (bit 0)
    #[prost(uint32, tag="3")]
    pub sign_scale: u32,
}
// @@protoc_insertion_point(module)
