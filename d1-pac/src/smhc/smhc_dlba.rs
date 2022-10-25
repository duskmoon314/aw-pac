#[doc = "Register `smhc_dlba` reader"]
pub struct R(crate::R<SMHC_DLBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_DLBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_DLBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_DLBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_dlba` writer"]
pub struct W(crate::W<SMHC_DLBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_DLBA_SPEC>;
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
impl From<crate::W<SMHC_DLBA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_DLBA_SPEC>) -> Self {
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
#[doc = "Descriptor List Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_dlba](index.html) module"]
pub struct SMHC_DLBA_SPEC;
impl crate::RegisterSpec for SMHC_DLBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_dlba::R](R) reader structure"]
impl crate::Readable for SMHC_DLBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_dlba::W](W) writer structure"]
impl crate::Writable for SMHC_DLBA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_dlba to value 0"]
impl crate::Resettable for SMHC_DLBA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
