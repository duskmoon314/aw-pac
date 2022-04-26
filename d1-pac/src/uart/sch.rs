#[doc = "Register `SCH` reader"]
pub struct R(crate::R<SCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCH` writer"]
pub struct W(crate::W<SCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCH_SPEC>;
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
impl From<crate::W<SCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `scratch` reader - "]
pub struct SCRATCH_R(crate::FieldReader<u8>);
impl SCRATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCRATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `scratch` writer - "]
pub struct SCRATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH_W<'a> {
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
    pub fn scratch(&self) -> SCRATCH_R {
        SCRATCH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scratch(&mut self) -> SCRATCH_W {
        SCRATCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Scratch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sch](index.html) module"]
pub struct SCH_SPEC;
impl crate::RegisterSpec for SCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sch::R](R) reader structure"]
impl crate::Readable for SCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sch::W](W) writer structure"]
impl crate::Writable for SCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCH to value 0"]
impl crate::Resettable for SCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
