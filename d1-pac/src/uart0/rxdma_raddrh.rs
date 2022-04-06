#[doc = "Register `RXDMA_RADDRH` reader"]
pub struct R(crate::R<RXDMA_RADDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_RADDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_RADDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_RADDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_RADDRH` writer"]
pub struct W(crate::W<RXDMA_RADDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_RADDRH_SPEC>;
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
impl From<crate::W<RXDMA_RADDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_RADDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `raddr` reader - RXDMA Current Read Address \\[33:32\\]"]
pub struct RADDR_R(crate::FieldReader<u8, u8>);
impl RADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `raddr` writer - RXDMA Current Read Address \\[33:32\\]"]
pub struct RADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RXDMA Current Read Address \\[33:32\\]"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXDMA Current Read Address \\[33:32\\]"]
    #[inline(always)]
    pub fn raddr(&mut self) -> RADDR_W {
        RADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Read Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_raddrh](index.html) module"]
pub struct RXDMA_RADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_RADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_raddrh::R](R) reader structure"]
impl crate::Readable for RXDMA_RADDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_raddrh::W](W) writer structure"]
impl crate::Writable for RXDMA_RADDRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_RADDRH to value 0"]
impl crate::Resettable for RXDMA_RADDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
