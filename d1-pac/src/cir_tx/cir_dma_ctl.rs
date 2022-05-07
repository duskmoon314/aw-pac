#[doc = "Register `CIR_DMA_CTL` reader"]
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
#[doc = "Register `CIR_DMA_CTL` writer"]
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
#[doc = "Handshake Configuration\n\nValue on reset: 165"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA` reader - Handshake Configuration"]
pub struct DMA_R(crate::FieldReader<u8>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DMA_A::WAITING_CYCLE
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        **self == DMA_A::HANDSHAKE
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Handshake Configuration"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
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
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
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
}
#[doc = "`reset()` method sets CIR_DMA_CTL to value 0xa5"]
impl crate::Resettable for CIR_DMA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa5
    }
}
