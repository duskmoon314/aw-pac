#[doc = "Register `SMHC_TCBCNT` reader"]
pub struct R(crate::R<SMHC_TCBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_TCBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_TCBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_TCBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_TCBCNT` writer"]
pub struct W(crate::W<SMHC_TCBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_TCBCNT_SPEC>;
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
impl From<crate::W<SMHC_TCBCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_TCBCNT_SPEC>) -> Self {
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
#[doc = "Transferred Byte Count between Controller and Card\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_tcbcnt](index.html) module"]
pub struct SMHC_TCBCNT_SPEC;
impl crate::RegisterSpec for SMHC_TCBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_tcbcnt::R](R) reader structure"]
impl crate::Readable for SMHC_TCBCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_tcbcnt::W](W) writer structure"]
impl crate::Writable for SMHC_TCBCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_TCBCNT to value 0"]
impl crate::Resettable for SMHC_TCBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
