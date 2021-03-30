//// u128 type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct U128 {
    /// Little-endian unsigned 128.
    #[prost(bytes, tag = "1")]
    pub buf: std::vec::Vec<u8>,
}
/// Type of contract argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VmTypeTag {
    /// Bool 0x0 - false, 0x1 - true.
    Bool = 0,
    /// Uint64. Little-endian unsigned 64 bits integer.
    U64 = 1,
    /// Vector of bytes.
    Vector = 2,
    /// Address, in bech32 form. 20 bytes.
    Address = 3,
    /// U8
    U8 = 4,
    /// U128 Little-endian unsigned 128 bits integer.
    U128 = 5,
}
