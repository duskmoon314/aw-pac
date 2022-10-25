#[doc = "Register `mclaim` reader"]
pub struct R(crate::R<MCLAIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLAIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLAIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLAIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mclaim` writer"]
pub struct W(crate::W<MCLAIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLAIM_SPEC>;
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
impl From<crate::W<MCLAIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLAIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mclaim` reader - "]
pub type MCLAIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `mclaim` writer - "]
pub type MCLAIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCLAIM_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn mclaim(&self) -> MCLAIM_R {
        MCLAIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn mclaim(&mut self) -> MCLAIM_W<0> {
        MCLAIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Machine Mode Claim/Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclaim](index.html) module"]
pub struct MCLAIM_SPEC;
impl crate::RegisterSpec for MCLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclaim::R](R) reader structure"]
impl crate::Readable for MCLAIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclaim::W](W) writer structure"]
impl crate::Writable for MCLAIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mclaim to value 0"]
impl crate::Resettable for MCLAIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
