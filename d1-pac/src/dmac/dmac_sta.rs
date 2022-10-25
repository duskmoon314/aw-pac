#[doc = "Register `dmac_sta` reader"]
pub struct R(crate::R<DMAC_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dma_status[0-15]` reader - DMA Channel\\[15:0\\] Status"]
pub type DMA_STATUS_R = crate::BitReader<DMA_STATUS_A>;
#[doc = "DMA Channel\\[15:0\\] Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_STATUS_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<DMA_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_STATUS_A {
        match self.bits {
            false => DMA_STATUS_A::IDLE,
            true => DMA_STATUS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DMA_STATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DMA_STATUS_A::BUSY
    }
}
#[doc = "Field `mbus_fifo_status` reader - MBUS FIFO Status"]
pub type MBUS_FIFO_STATUS_R = crate::BitReader<MBUS_FIFO_STATUS_A>;
#[doc = "MBUS FIFO Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBUS_FIFO_STATUS_A {
    #[doc = "0: `0`"]
    EMPTY = 0,
    #[doc = "1: `1`"]
    NOT_EMPTY = 1,
}
impl From<MBUS_FIFO_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MBUS_FIFO_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl MBUS_FIFO_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBUS_FIFO_STATUS_A {
        match self.bits {
            false => MBUS_FIFO_STATUS_A::EMPTY,
            true => MBUS_FIFO_STATUS_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == MBUS_FIFO_STATUS_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == MBUS_FIFO_STATUS_A::NOT_EMPTY
    }
}
impl R {
    #[doc = "DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub unsafe fn dma_status(&self, n: u8) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status0(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status1(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status2(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status3(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status4(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status5(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status6(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status7(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status8(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status9(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status10(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status11(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status12(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status13(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status14(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA Channel\\[15:0\\] Status"]
    #[inline(always)]
    pub fn dma_status15(&self) -> DMA_STATUS_R {
        DMA_STATUS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - MBUS FIFO Status"]
    #[inline(always)]
    pub fn mbus_fifo_status(&self) -> MBUS_FIFO_STATUS_R {
        MBUS_FIFO_STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_sta](index.html) module"]
pub struct DMAC_STA_SPEC;
impl crate::RegisterSpec for DMAC_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_sta::R](R) reader structure"]
impl crate::Readable for DMAC_STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_sta to value 0"]
impl crate::Resettable for DMAC_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
