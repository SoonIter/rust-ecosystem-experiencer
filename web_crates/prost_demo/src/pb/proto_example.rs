// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(enumeration = "PhoneType", tag = "2")]
    pub phone_type: i32,
    #[prost(string, tag = "3")]
    pub phone_number: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhoneType {
    Mobile = 0,
    Home = 1,
    Work = 2,
}
impl PhoneType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PhoneType::Mobile => "MOBILE",
            PhoneType::Home => "HOME",
            PhoneType::Work => "WORK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MOBILE" => Some(Self::Mobile),
            "HOME" => Some(Self::Home),
            "WORK" => Some(Self::Work),
            _ => None,
        }
    }
}
