#[doc = "Register `fre_up_lim` reader"]
pub struct R(crate::R<FRE_UP_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRE_UP_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRE_UP_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRE_UP_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fre_up_lim` writer"]
pub struct W(crate::W<FRE_UP_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRE_UP_LIM_SPEC>;
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
impl From<crate::W<FRE_UP_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRE_UP_LIM_SPEC>) -> Self {
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
#[doc = "Frequency Up Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fre_up_lim](index.html) module"]
pub struct FRE_UP_LIM_SPEC;
impl crate::RegisterSpec for FRE_UP_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fre_up_lim::R](R) reader structure"]
impl crate::Readable for FRE_UP_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fre_up_lim::W](W) writer structure"]
impl crate::Writable for FRE_UP_LIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fre_up_lim to value 0"]
impl crate::Resettable for FRE_UP_LIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
