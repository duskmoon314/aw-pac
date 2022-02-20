#[doc = "Register `RXDMA_BL` reader"]
pub struct R(crate::R<RXDMA_BL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_BL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_BL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_BL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_BL` writer"]
pub struct W(crate::W<RXDMA_BL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_BL_SPEC>;
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
impl From<crate::W<RXDMA_BL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_BL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buffer_length` reader - "]
pub struct BUFFER_LENGTH_R(crate::FieldReader<u16, u16>);
impl BUFFER_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUFFER_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFFER_LENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buffer_length` writer - "]
pub struct BUFFER_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn buffer_length(&self) -> BUFFER_LENGTH_R {
        BUFFER_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn buffer_length(&mut self) -> BUFFER_LENGTH_W {
        BUFFER_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Buffer Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_bl](index.html) module"]
pub struct RXDMA_BL_SPEC;
impl crate::RegisterSpec for RXDMA_BL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_bl::R](R) reader structure"]
impl crate::Readable for RXDMA_BL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_bl::W](W) writer structure"]
impl crate::Writable for RXDMA_BL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_BL to value 0"]
impl crate::Resettable for RXDMA_BL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
