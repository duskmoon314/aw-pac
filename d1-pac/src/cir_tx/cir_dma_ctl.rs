#[doc = "Register `cir_dma_ctl` reader"]
pub struct R(crate::R<CIR_DMA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_DMA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_DMA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_DMA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_dma_ctl` writer"]
pub struct W(crate::W<CIR_DMA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_DMA_CTL_SPEC>;
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
impl From<crate::W<CIR_DMA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_DMA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma` reader - Handshake Configuration"]
pub type DMA_R = crate::FieldReader<u8, DMA_A>;
#[doc = "Handshake Configuration\n\nValue on reset: 165"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_A {
    #[doc = "165: DMA waiting cycle mode"]
    WAITING_CYCLE = 165,
    #[doc = "234: DMA handshake mode"]
    HANDSHAKE = 234,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMA_A> {
        match self.bits {
            165 => Some(DMA_A::WAITING_CYCLE),
            234 => Some(DMA_A::HANDSHAKE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAITING_CYCLE`"]
    #[inline(always)]
    pub fn is_waiting_cycle(&self) -> bool {
        *self == DMA_A::WAITING_CYCLE
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        *self == DMA_A::HANDSHAKE
    }
}
#[doc = "Field `dma` writer - Handshake Configuration"]
pub type DMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIR_DMA_CTL_SPEC, u8, DMA_A, 8, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "DMA waiting cycle mode"]
    #[inline(always)]
    pub fn waiting_cycle(self) -> &'a mut W {
        self.variant(DMA_A::WAITING_CYCLE)
    }
    #[doc = "DMA handshake mode"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut W {
        self.variant(DMA_A::HANDSHAKE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Handshake Configuration"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Handshake Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<0> {
        DMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_dma_ctl](index.html) module"]
pub struct CIR_DMA_CTL_SPEC;
impl crate::RegisterSpec for CIR_DMA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_dma_ctl::R](R) reader structure"]
impl crate::Readable for CIR_DMA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_dma_ctl::W](W) writer structure"]
impl crate::Writable for CIR_DMA_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_dma_ctl to value 0xa5"]
impl crate::Resettable for CIR_DMA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa5;
}
