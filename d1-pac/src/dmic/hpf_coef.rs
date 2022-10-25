#[doc = "Register `hpf_coef` reader"]
pub struct R(crate::R<HPF_COEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPF_COEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPF_COEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPF_COEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hpf_coef` writer"]
pub struct W(crate::W<HPF_COEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPF_COEF_SPEC>;
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
impl From<crate::W<HPF_COEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPF_COEF_SPEC>) -> Self {
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
#[doc = "High Pass Filter Coefficient Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpf_coef](index.html) module"]
pub struct HPF_COEF_SPEC;
impl crate::RegisterSpec for HPF_COEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpf_coef::R](R) reader structure"]
impl crate::Readable for HPF_COEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpf_coef::W](W) writer structure"]
impl crate::Writable for HPF_COEF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hpf_coef to value 0"]
impl crate::Resettable for HPF_COEF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
