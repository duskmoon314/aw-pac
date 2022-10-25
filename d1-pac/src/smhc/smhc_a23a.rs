#[doc = "Register `smhc_a23a` reader"]
pub struct R(crate::R<SMHC_A23A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_A23A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_A23A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_A23A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_a23a` writer"]
pub struct W(crate::W<SMHC_A23A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_A23A_SPEC>;
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
impl From<crate::W<SMHC_A23A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_A23A_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto Command 23 Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_a23a](index.html) module"]
pub struct SMHC_A23A_SPEC;
impl crate::RegisterSpec for SMHC_A23A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_a23a::R](R) reader structure"]
impl crate::Readable for SMHC_A23A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_a23a::W](W) writer structure"]
impl crate::Writable for SMHC_A23A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_a23a to value 0"]
impl crate::Resettable for SMHC_A23A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
