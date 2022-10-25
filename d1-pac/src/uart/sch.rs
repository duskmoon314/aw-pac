#[doc = "Register `sch` reader"]
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
#[doc = "Register `sch` writer"]
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
pub type SCRATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `scratch` writer - "]
pub type SCRATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCH_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn scratch(&mut self) -> SCRATCH_W<0> {
        SCRATCH_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sch to value 0"]
impl crate::Resettable for SCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
