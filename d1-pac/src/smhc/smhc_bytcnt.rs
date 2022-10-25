#[doc = "Register `smhc_bytcnt` reader"]
pub struct R(crate::R<SMHC_BYTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_BYTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_BYTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_BYTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_bytcnt` writer"]
pub struct W(crate::W<SMHC_BYTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_BYTCNT_SPEC>;
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
impl From<crate::W<SMHC_BYTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_BYTCNT_SPEC>) -> Self {
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
#[doc = "Byte Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_bytcnt](index.html) module"]
pub struct SMHC_BYTCNT_SPEC;
impl crate::RegisterSpec for SMHC_BYTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_bytcnt::R](R) reader structure"]
impl crate::Readable for SMHC_BYTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_bytcnt::W](W) writer structure"]
impl crate::Writable for SMHC_BYTCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_bytcnt to value 0"]
impl crate::Resettable for SMHC_BYTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
