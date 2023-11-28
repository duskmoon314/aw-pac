#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    msgbox: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x120 - Communicate with CPU\\[N\\]"]
    #[inline(always)]
    pub const fn msgbox(&self, n: usize) -> &MSGBOX {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        }
    }
}
#[doc = "Communicate with CPU\\[N\\]"]
pub use self::msgbox::MSGBOX;
#[doc = r"Cluster"]
#[doc = "Communicate with CPU\\[N\\]"]
pub mod msgbox;
