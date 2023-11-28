#[doc = "Register `dmac_cfg%s` reader"]
pub type R = crate::R<DMAC_CFG_SPEC>;
#[doc = "Field `dma_src_drq_type` reader - DMA Source DRQ Type"]
pub type DMA_SRC_DRQ_TYPE_R = crate::FieldReader;
#[doc = "Field `dma_src_block_size` reader - DMA Source Block Size"]
pub type DMA_SRC_BLOCK_SIZE_R = crate::FieldReader<DMA_SRC_BLOCK_SIZE_A>;
#[doc = "DMA Source Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_SRC_BLOCK_SIZE_A {
    #[doc = "0: `0`"]
    S1 = 0,
    #[doc = "1: `1`"]
    S4 = 1,
    #[doc = "2: `10`"]
    S8 = 2,
    #[doc = "3: `11`"]
    S16 = 3,
}
impl From<DMA_SRC_BLOCK_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_SRC_BLOCK_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_SRC_BLOCK_SIZE_A {
    type Ux = u8;
}
impl DMA_SRC_BLOCK_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_SRC_BLOCK_SIZE_A {
        match self.bits {
            0 => DMA_SRC_BLOCK_SIZE_A::S1,
            1 => DMA_SRC_BLOCK_SIZE_A::S4,
            2 => DMA_SRC_BLOCK_SIZE_A::S8,
            3 => DMA_SRC_BLOCK_SIZE_A::S16,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_s1(&self) -> bool {
        *self == DMA_SRC_BLOCK_SIZE_A::S1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_s4(&self) -> bool {
        *self == DMA_SRC_BLOCK_SIZE_A::S4
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_s8(&self) -> bool {
        *self == DMA_SRC_BLOCK_SIZE_A::S8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_s16(&self) -> bool {
        *self == DMA_SRC_BLOCK_SIZE_A::S16
    }
}
#[doc = "Field `dma_src_addr_mode` reader - DMA Source Address Mode"]
pub type DMA_SRC_ADDR_MODE_R = crate::BitReader<DMA_SRC_ADDR_MODE_A>;
#[doc = "DMA Source Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_SRC_ADDR_MODE_A {
    #[doc = "0: `0`"]
    LINEAR = 0,
    #[doc = "1: `1`"]
    IO = 1,
}
impl From<DMA_SRC_ADDR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_SRC_ADDR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_SRC_ADDR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_SRC_ADDR_MODE_A {
        match self.bits {
            false => DMA_SRC_ADDR_MODE_A::LINEAR,
            true => DMA_SRC_ADDR_MODE_A::IO,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == DMA_SRC_ADDR_MODE_A::LINEAR
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == DMA_SRC_ADDR_MODE_A::IO
    }
}
#[doc = "Field `dma_src_data_width` reader - DMA Source Data Width"]
pub type DMA_SRC_DATA_WIDTH_R = crate::FieldReader<DMA_SRC_DATA_WIDTH_A>;
#[doc = "DMA Source Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_SRC_DATA_WIDTH_A {
    #[doc = "0: 8 bit"]
    B8 = 0,
    #[doc = "1: 16 bit"]
    B16 = 1,
    #[doc = "2: 32 bit"]
    B32 = 2,
    #[doc = "3: 64 bit"]
    B64 = 3,
}
impl From<DMA_SRC_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_SRC_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_SRC_DATA_WIDTH_A {
    type Ux = u8;
}
impl DMA_SRC_DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_SRC_DATA_WIDTH_A {
        match self.bits {
            0 => DMA_SRC_DATA_WIDTH_A::B8,
            1 => DMA_SRC_DATA_WIDTH_A::B16,
            2 => DMA_SRC_DATA_WIDTH_A::B32,
            3 => DMA_SRC_DATA_WIDTH_A::B64,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == DMA_SRC_DATA_WIDTH_A::B8
    }
    #[doc = "16 bit"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        *self == DMA_SRC_DATA_WIDTH_A::B16
    }
    #[doc = "32 bit"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        *self == DMA_SRC_DATA_WIDTH_A::B32
    }
    #[doc = "64 bit"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        *self == DMA_SRC_DATA_WIDTH_A::B64
    }
}
#[doc = "Field `dma_dest_drq_type` reader - DMA Destination DRQ Type"]
pub type DMA_DEST_DRQ_TYPE_R = crate::FieldReader;
#[doc = "Field `dma_dest_block_size` reader - DMA Destination Block Size"]
pub type DMA_DEST_BLOCK_SIZE_R = crate::FieldReader<DMA_DEST_BLOCK_SIZE_A>;
#[doc = "DMA Destination Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_DEST_BLOCK_SIZE_A {
    #[doc = "0: `0`"]
    S1 = 0,
    #[doc = "1: `1`"]
    S4 = 1,
    #[doc = "2: `10`"]
    S8 = 2,
    #[doc = "3: `11`"]
    S16 = 3,
}
impl From<DMA_DEST_BLOCK_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_DEST_BLOCK_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_DEST_BLOCK_SIZE_A {
    type Ux = u8;
}
impl DMA_DEST_BLOCK_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_DEST_BLOCK_SIZE_A {
        match self.bits {
            0 => DMA_DEST_BLOCK_SIZE_A::S1,
            1 => DMA_DEST_BLOCK_SIZE_A::S4,
            2 => DMA_DEST_BLOCK_SIZE_A::S8,
            3 => DMA_DEST_BLOCK_SIZE_A::S16,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_s1(&self) -> bool {
        *self == DMA_DEST_BLOCK_SIZE_A::S1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_s4(&self) -> bool {
        *self == DMA_DEST_BLOCK_SIZE_A::S4
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_s8(&self) -> bool {
        *self == DMA_DEST_BLOCK_SIZE_A::S8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_s16(&self) -> bool {
        *self == DMA_DEST_BLOCK_SIZE_A::S16
    }
}
#[doc = "Field `dma_addr_mode` reader - DMA Destination Address Mode"]
pub type DMA_ADDR_MODE_R = crate::BitReader<DMA_ADDR_MODE_A>;
#[doc = "DMA Destination Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_ADDR_MODE_A {
    #[doc = "0: `0`"]
    LINEAR = 0,
    #[doc = "1: `1`"]
    IO = 1,
}
impl From<DMA_ADDR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ADDR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_ADDR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_ADDR_MODE_A {
        match self.bits {
            false => DMA_ADDR_MODE_A::LINEAR,
            true => DMA_ADDR_MODE_A::IO,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == DMA_ADDR_MODE_A::LINEAR
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == DMA_ADDR_MODE_A::IO
    }
}
#[doc = "Field `dma_dest_data_width` reader - DMA Destination Data Width"]
pub type DMA_DEST_DATA_WIDTH_R = crate::FieldReader<DMA_DEST_DATA_WIDTH_A>;
#[doc = "DMA Destination Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_DEST_DATA_WIDTH_A {
    #[doc = "0: 8 bit"]
    B8 = 0,
    #[doc = "1: 16 bit"]
    B16 = 1,
    #[doc = "2: 32 bit"]
    B32 = 2,
    #[doc = "3: 64 bit"]
    B64 = 3,
}
impl From<DMA_DEST_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_DEST_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA_DEST_DATA_WIDTH_A {
    type Ux = u8;
}
impl DMA_DEST_DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_DEST_DATA_WIDTH_A {
        match self.bits {
            0 => DMA_DEST_DATA_WIDTH_A::B8,
            1 => DMA_DEST_DATA_WIDTH_A::B16,
            2 => DMA_DEST_DATA_WIDTH_A::B32,
            3 => DMA_DEST_DATA_WIDTH_A::B64,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == DMA_DEST_DATA_WIDTH_A::B8
    }
    #[doc = "16 bit"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        *self == DMA_DEST_DATA_WIDTH_A::B16
    }
    #[doc = "32 bit"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        *self == DMA_DEST_DATA_WIDTH_A::B32
    }
    #[doc = "64 bit"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        *self == DMA_DEST_DATA_WIDTH_A::B64
    }
}
#[doc = "Field `bmode_sel` reader - "]
pub type BMODE_SEL_R = crate::BitReader<BMODE_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMODE_SEL_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    BMODE = 1,
}
impl From<BMODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BMODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BMODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BMODE_SEL_A {
        match self.bits {
            false => BMODE_SEL_A::NORMAL,
            true => BMODE_SEL_A::BMODE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BMODE_SEL_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_bmode(&self) -> bool {
        *self == BMODE_SEL_A::BMODE
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Source DRQ Type"]
    #[inline(always)]
    pub fn dma_src_drq_type(&self) -> DMA_SRC_DRQ_TYPE_R {
        DMA_SRC_DRQ_TYPE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - DMA Source Block Size"]
    #[inline(always)]
    pub fn dma_src_block_size(&self) -> DMA_SRC_BLOCK_SIZE_R {
        DMA_SRC_BLOCK_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - DMA Source Address Mode"]
    #[inline(always)]
    pub fn dma_src_addr_mode(&self) -> DMA_SRC_ADDR_MODE_R {
        DMA_SRC_ADDR_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - DMA Source Data Width"]
    #[inline(always)]
    pub fn dma_src_data_width(&self) -> DMA_SRC_DATA_WIDTH_R {
        DMA_SRC_DATA_WIDTH_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 16:21 - DMA Destination DRQ Type"]
    #[inline(always)]
    pub fn dma_dest_drq_type(&self) -> DMA_DEST_DRQ_TYPE_R {
        DMA_DEST_DRQ_TYPE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - DMA Destination Block Size"]
    #[inline(always)]
    pub fn dma_dest_block_size(&self) -> DMA_DEST_BLOCK_SIZE_R {
        DMA_DEST_BLOCK_SIZE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - DMA Destination Address Mode"]
    #[inline(always)]
    pub fn dma_addr_mode(&self) -> DMA_ADDR_MODE_R {
        DMA_ADDR_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - DMA Destination Data Width"]
    #[inline(always)]
    pub fn dma_dest_data_width(&self) -> DMA_DEST_DATA_WIDTH_R {
        DMA_DEST_DATA_WIDTH_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn bmode_sel(&self) -> BMODE_SEL_R {
        BMODE_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "DMAC Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_CFG_SPEC;
impl crate::RegisterSpec for DMAC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cfg::R`](R) reader structure"]
impl crate::Readable for DMAC_CFG_SPEC {}
#[doc = "`reset()` method sets dmac_cfg%s to value 0"]
impl crate::Resettable for DMAC_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
