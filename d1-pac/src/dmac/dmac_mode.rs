#[doc = "Register `dmac_mode%s` reader"]
pub struct R(crate::R<DMAC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmac_mode%s` writer"]
pub struct W(crate::W<DMAC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_MODE_SPEC>;
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
impl From<crate::W<DMAC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_src_mode` reader - Source Communication Mode Select"]
pub type DMA_SRC_MODE_R = crate::BitReader<DMA_SRC_MODE_A>;
#[doc = "Source Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_SRC_MODE_A {
    #[doc = "0: `0`"]
    WAITING = 0,
    #[doc = "1: `1`"]
    HANDSHAKE = 1,
}
impl From<DMA_SRC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_SRC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_SRC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_SRC_MODE_A {
        match self.bits {
            false => DMA_SRC_MODE_A::WAITING,
            true => DMA_SRC_MODE_A::HANDSHAKE,
        }
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == DMA_SRC_MODE_A::WAITING
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == DMA_SRC_MODE_A::HANDSHAKE
    }
}
#[doc = "Field `dma_src_mode` writer - Source Communication Mode Select"]
pub type DMA_SRC_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_MODE_SPEC, DMA_SRC_MODE_A, O>;
impl<'a, const O: u8> DMA_SRC_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn waiting(self) -> &'a mut W {
        self.variant(DMA_SRC_MODE_A::WAITING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut W {
        self.variant(DMA_SRC_MODE_A::HANDSHAKE)
    }
}
#[doc = "Field `dma_dst_mode` reader - Destination Communication Mode Select"]
pub type DMA_DST_MODE_R = crate::BitReader<DMA_DST_MODE_A>;
#[doc = "Destination Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_DST_MODE_A {
    #[doc = "0: `0`"]
    WAITING = 0,
    #[doc = "1: `1`"]
    HANDSHAKE = 1,
}
impl From<DMA_DST_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DST_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_DST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_DST_MODE_A {
        match self.bits {
            false => DMA_DST_MODE_A::WAITING,
            true => DMA_DST_MODE_A::HANDSHAKE,
        }
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == DMA_DST_MODE_A::WAITING
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == DMA_DST_MODE_A::HANDSHAKE
    }
}
#[doc = "Field `dma_dst_mode` writer - Destination Communication Mode Select"]
pub type DMA_DST_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC_MODE_SPEC, DMA_DST_MODE_A, O>;
impl<'a, const O: u8> DMA_DST_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn waiting(self) -> &'a mut W {
        self.variant(DMA_DST_MODE_A::WAITING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut W {
        self.variant(DMA_DST_MODE_A::HANDSHAKE)
    }
}
impl R {
    #[doc = "Bit 2 - Source Communication Mode Select"]
    #[inline(always)]
    pub fn dma_src_mode(&self) -> DMA_SRC_MODE_R {
        DMA_SRC_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Communication Mode Select"]
    #[inline(always)]
    pub fn dma_dst_mode(&self) -> DMA_DST_MODE_R {
        DMA_DST_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Source Communication Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma_src_mode(&mut self) -> DMA_SRC_MODE_W<2> {
        DMA_SRC_MODE_W::new(self)
    }
    #[doc = "Bit 3 - Destination Communication Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma_dst_mode(&mut self) -> DMA_DST_MODE_W<3> {
        DMA_DST_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_mode](index.html) module"]
pub struct DMAC_MODE_SPEC;
impl crate::RegisterSpec for DMAC_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_mode::R](R) reader structure"]
impl crate::Readable for DMAC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_mode::W](W) writer structure"]
impl crate::Writable for DMAC_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_mode%s to value 0"]
impl crate::Resettable for DMAC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
