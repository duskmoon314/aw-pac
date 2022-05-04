#[doc = "Register `DMAC_DESC_ADDR_REG%s` reader"]
pub struct R(crate::R<DMAC_DESC_ADDR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_DESC_ADDR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_DESC_ADDR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_DESC_ADDR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_DESC_ADDR_REG%s` writer"]
pub struct W(crate::W<DMAC_DESC_ADDR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_DESC_ADDR_REG_SPEC>;
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
impl From<crate::W<DMAC_DESC_ADDR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_DESC_ADDR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_DESC_ADDR` reader - Lower 30 bits of DMA channel descriptor address"]
pub struct DMA_DESC_ADDR_R(crate::FieldReader<u32>);
impl DMA_DESC_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_DESC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DESC_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DESC_ADDR` writer - Lower 30 bits of DMA channel descriptor address"]
pub struct DMA_DESC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DESC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `DMA_DESC_HIGH_ADDR` reader - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
pub struct DMA_DESC_HIGH_ADDR_R(crate::FieldReader<u8>);
impl DMA_DESC_HIGH_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DESC_HIGH_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DESC_HIGH_ADDR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DESC_HIGH_ADDR` writer - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
pub struct DMA_DESC_HIGH_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DESC_HIGH_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Lower 30 bits of DMA channel descriptor address"]
    #[inline(always)]
    pub fn dma_desc_addr(&self) -> DMA_DESC_ADDR_R {
        DMA_DESC_ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
    #[inline(always)]
    pub fn dma_desc_high_addr(&self) -> DMA_DESC_HIGH_ADDR_R {
        DMA_DESC_HIGH_ADDR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - Lower 30 bits of DMA channel descriptor address"]
    #[inline(always)]
    pub fn dma_desc_addr(&mut self) -> DMA_DESC_ADDR_W {
        DMA_DESC_ADDR_W { w: self }
    }
    #[doc = "Bits 0:1 - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
    #[inline(always)]
    pub fn dma_desc_high_addr(&mut self) -> DMA_DESC_HIGH_ADDR_W {
        DMA_DESC_HIGH_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_desc_addr_reg](index.html) module"]
pub struct DMAC_DESC_ADDR_REG_SPEC;
impl crate::RegisterSpec for DMAC_DESC_ADDR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_desc_addr_reg::R](R) reader structure"]
impl crate::Readable for DMAC_DESC_ADDR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_desc_addr_reg::W](W) writer structure"]
impl crate::Writable for DMAC_DESC_ADDR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_DESC_ADDR_REG%s to value 0"]
impl crate::Resettable for DMAC_DESC_ADDR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
