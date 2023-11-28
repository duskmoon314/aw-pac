#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    csic_ccu: CSIC_CCU,
    _reserved1: [u8; 0x07f0],
    csic_top: CSIC_TOP,
    _reserved2: [u8; 0x06f8],
    csic_parser0: CSIC_PARSER0,
    _reserved3: [u8; 0x7ae4],
    csic_dma: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - CSIC_CCU"]
    #[inline(always)]
    pub const fn csic_ccu(&self) -> &CSIC_CCU {
        &self.csic_ccu
    }
    #[doc = "0x800..0x908 - CSIC_TOP"]
    #[inline(always)]
    pub const fn csic_top(&self) -> &CSIC_TOP {
        &self.csic_top
    }
    #[doc = "0x1000..0x151c - CSIC_PARSER0"]
    #[inline(always)]
    pub const fn csic_parser0(&self) -> &CSIC_PARSER0 {
        &self.csic_parser0
    }
    #[doc = "0x9000..0x93f0 - CSIC_DMA"]
    #[inline(always)]
    pub const fn csic_dma(&self, n: usize) -> &CSIC_DMA {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36864)
                .add(512 * n)
                .cast()
        }
    }
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
