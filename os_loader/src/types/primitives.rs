#[repr(transparent)]
#[repr(align(8))]
struct EfiGuid(u128);

#[repr(transparent)]
#[repr(align(4))]
struct EfiIpAddr([u8; 16]);
