#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - CSIC_CCU"]
    pub csic_ccu: CSIC_CCU,
    _reserved1: [u8; 0x07f0],
    #[doc = "0x800..0x908 - CSIC_TOP"]
    pub csic_top: CSIC_TOP,
    _reserved2: [u8; 0x06f8],
    #[doc = "0x1000..0x151c - CSIC_PARSER0"]
    pub csic_parser0: CSIC_PARSER0,
    _reserved3: [u8; 0x7ae4],
    #[doc = "0x9000 - CSIC_DMA"]
    pub csic_dma: crate::ArrayProxy<CSIC_DMA, 2, 0x0200>,
}
#[doc = "CSIC_CCU"]
pub use self::csic_ccu::CSIC_CCU;
#[doc = r"Cluster"]
#[doc = "CSIC_CCU"]
pub mod csic_ccu;
#[doc = "CSIC_TOP"]
pub use self::csic_top::CSIC_TOP;
#[doc = r"Cluster"]
#[doc = "CSIC_TOP"]
pub mod csic_top;
#[doc = "CSIC_PARSER0"]
pub use self::csic_parser0::CSIC_PARSER0;
#[doc = r"Cluster"]
#[doc = "CSIC_PARSER0"]
pub mod csic_parser0;
#[doc = "CSIC_DMA"]
pub use self::csic_dma::CSIC_DMA;
#[doc = r"Cluster"]
#[doc = "CSIC_DMA"]
pub mod csic_dma;
