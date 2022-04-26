#[doc = "Register `SMHC_A12A` reader"]
pub struct R(crate::R<SMHC_A12A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_A12A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_A12A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_A12A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_A12A` writer"]
pub struct W(crate::W<SMHC_A12A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_A12A_SPEC>;
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
impl From<crate::W<SMHC_A12A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_A12A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD_A12A` reader - "]
pub struct SD_A12A_R(crate::FieldReader<u16>);
impl SD_A12A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SD_A12A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_A12A_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD_A12A` writer - "]
pub struct SD_A12A_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_A12A_W<'a> {
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
    pub fn sd_a12a(&self) -> SD_A12A_R {
        SD_A12A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sd_a12a(&mut self) -> SD_A12A_W {
        SD_A12A_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto Command 12 Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_a12a](index.html) module"]
pub struct SMHC_A12A_SPEC;
impl crate::RegisterSpec for SMHC_A12A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_a12a::R](R) reader structure"]
impl crate::Readable for SMHC_A12A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_a12a::W](W) writer structure"]
impl crate::Writable for SMHC_A12A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_A12A to value 0"]
impl crate::Resettable for SMHC_A12A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
