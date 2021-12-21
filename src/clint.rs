#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSIP Register for hart 0"]
    pub msip: crate::Reg<msip::MSIP_SPEC>,
    _reserved1: [u8; 0x3ffc],
    #[doc = "0x4000 - MTIMECMPL Register for hart 0"]
    pub mtimecmpl: crate::Reg<mtimecmpl::MTIMECMPL_SPEC>,
    #[doc = "0x4004 - MTIMECMPH Register for hart 0"]
    pub mtimecmph: crate::Reg<mtimecmph::MTIMECMPH_SPEC>,
    _reserved3: [u8; 0x7ff0],
    #[doc = "0xbff8..0xc000 - MTIME\n\nREF: opensbi"]
    pub mtime: crate::Reg<mtime::MTIME_SPEC>,
    #[doc = "0xc000 - SSIP Register for hart 0"]
    pub ssip: crate::Reg<ssip::SSIP_SPEC>,
    _reserved5: [u8; 0x0ffc],
    #[doc = "0xd000 - STIMECMPL Register for hart 0"]
    pub stimecmpl: crate::Reg<stimecmpl::STIMECMPL_SPEC>,
    #[doc = "0xd004 - STIMECMPH Register for hart 0"]
    pub stimecmph: crate::Reg<stimecmph::STIMECMPH_SPEC>,
}
#[doc = "msip register accessor: an alias for `Reg<MSIP_SPEC>`"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "MSIP Register for hart 0"]
pub mod msip;
#[doc = "mtimecmpl register accessor: an alias for `Reg<MTIMECMPL_SPEC>`"]
pub type MTIMECMPL = crate::Reg<mtimecmpl::MTIMECMPL_SPEC>;
#[doc = "MTIMECMPL Register for hart 0"]
pub mod mtimecmpl;
#[doc = "mtimecmph register accessor: an alias for `Reg<MTIMECMPH_SPEC>`"]
pub type MTIMECMPH = crate::Reg<mtimecmph::MTIMECMPH_SPEC>;
#[doc = "MTIMECMPH Register for hart 0"]
pub mod mtimecmph;
#[doc = "mtime register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "MTIME\n\nREF: opensbi"]
pub mod mtime;
#[doc = "ssip register accessor: an alias for `Reg<SSIP_SPEC>`"]
pub type SSIP = crate::Reg<ssip::SSIP_SPEC>;
#[doc = "SSIP Register for hart 0"]
pub mod ssip;
#[doc = "stimecmpl register accessor: an alias for `Reg<STIMECMPL_SPEC>`"]
pub type STIMECMPL = crate::Reg<stimecmpl::STIMECMPL_SPEC>;
#[doc = "STIMECMPL Register for hart 0"]
pub mod stimecmpl;
#[doc = "stimecmph register accessor: an alias for `Reg<STIMECMPH_SPEC>`"]
pub type STIMECMPH = crate::Reg<stimecmph::STIMECMPH_SPEC>;
#[doc = "STIMECMPH Register for hart 0"]
pub mod stimecmph;
