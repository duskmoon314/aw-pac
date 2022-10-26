#[doc = "Register `dmac_desc_addr%s` reader"]
pub struct R(crate::R<DMAC_DESC_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_DESC_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_DESC_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_DESC_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_desc_addr%s` writer"]
pub struct W(crate::W<DMAC_DESC_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_DESC_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMAC_DESC_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_DESC_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_desc_high_addr` reader - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
pub type DMA_DESC_HIGH_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dma_desc_high_addr` writer - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
pub type DMA_DESC_HIGH_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC_DESC_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `dma_desc_addr` reader - Lower 30 bits of DMA channel descriptor address"]
pub type DMA_DESC_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dma_desc_addr` writer - Lower 30 bits of DMA channel descriptor address"]
pub type DMA_DESC_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC_DESC_ADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
    #[inline(always)]
    pub fn dma_desc_high_addr(&self) -> DMA_DESC_HIGH_ADDR_R {
        DMA_DESC_HIGH_ADDR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Lower 30 bits of DMA channel descriptor address"]
    #[inline(always)]
    pub fn dma_desc_addr(&self) -> DMA_DESC_ADDR_R {
        DMA_DESC_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
    #[inline(always)]
    #[must_use]
    pub fn dma_desc_high_addr(&mut self) -> DMA_DESC_HIGH_ADDR_W<0> {
        DMA_DESC_HIGH_ADDR_W::new(self)
    }
    #[doc = "Bits 2:31 - Lower 30 bits of DMA channel descriptor address"]
    #[inline(always)]
    #[must_use]
    pub fn dma_desc_addr(&mut self) -> DMA_DESC_ADDR_W<2> {
        DMA_DESC_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_desc_addr](index.html) module"]
pub struct DMAC_DESC_ADDR_SPEC;
impl crate::RegisterSpec for DMAC_DESC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_desc_addr::R](R) reader structure"]
impl crate::Readable for DMAC_DESC_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_desc_addr::W](W) writer structure"]
impl crate::Writable for DMAC_DESC_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_desc_addr%s to value 0"]
impl crate::Resettable for DMAC_DESC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
