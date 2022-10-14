#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Communicate with CPU\\[N\\]"]
    pub msgbox: crate::ArrayProxy<MSGBOX, 2, 0x0100>,
}
#[doc = "Communicate with CPU\\[N\\]"]
pub use self::msgbox::MSGBOX;
#[doc = r"Cluster"]
#[doc = "Communicate with CPU\\[N\\]"]
pub mod msgbox;
