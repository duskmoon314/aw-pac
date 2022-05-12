#[doc = "Register `DMAC_BCNT_LEFT_REG%s` reader"]
pub struct R(crate::R<DMAC_BCNT_LEFT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_BCNT_LEFT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_BCNT_LEFT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_BCNT_LEFT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_BCNT_LEFT` reader - DMA Channel Byte Counter Left"]
pub type DMA_BCNT_LEFT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - DMA Channel Byte Counter Left"]
    #[inline(always)]
    pub fn dma_bcnt_left(&self) -> DMA_BCNT_LEFT_R {
        DMA_BCNT_LEFT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "DMAC Channel Byte Counter Left Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_bcnt_left_reg](index.html) module"]
pub struct DMAC_BCNT_LEFT_REG_SPEC;
impl crate::RegisterSpec for DMAC_BCNT_LEFT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_bcnt_left_reg::R](R) reader structure"]
impl crate::Readable for DMAC_BCNT_LEFT_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAC_BCNT_LEFT_REG%s to value 0"]
impl crate::Resettable for DMAC_BCNT_LEFT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
