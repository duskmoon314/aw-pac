#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x90 - Communicate with CPU\\[N\\]"]
    pub msgbox0: MSGBOX,
    _reserved1: [u8; 0x70],
    #[doc = "0x100..0x190 - Communicate with CPU\\[N\\]"]
    pub msgbox1: MSGBOX,
}
#[doc = "Communicate with CPU\\[N\\]"]
pub use msgbox::MSGBOX;
#[doc = r"Cluster"]
#[doc = "Communicate with CPU\\[N\\]"]
pub mod msgbox;
