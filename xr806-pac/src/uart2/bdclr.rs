#[doc = "Register `BDCLR` reader"]
pub struct R(crate::R<BDCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCLR` writer"]
pub struct W(crate::W<BDCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCLR_SPEC>;
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
impl From<crate::W<BDCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDCL` reader - "]
pub struct BDCL_R(crate::FieldReader<u8, u8>);
impl BDCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BDCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDCL` writer - "]
pub struct BDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> BDCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdcl(&self) -> BDCL_R {
        BDCL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdcl(&mut self) -> BDCL_W {
        BDCL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baudrate Detection Counter Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdclr](index.html) module"]
pub struct BDCLR_SPEC;
impl crate::RegisterSpec for BDCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdclr::R](R) reader structure"]
impl crate::Readable for BDCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdclr::W](W) writer structure"]
impl crate::Writable for BDCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDCLR to value 0"]
impl crate::Resettable for BDCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
