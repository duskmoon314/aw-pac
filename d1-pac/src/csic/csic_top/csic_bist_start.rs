#[doc = "Register `csic_bist_start` reader"]
pub struct R(crate::R<CSIC_BIST_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_BIST_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_BIST_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_BIST_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_bist_start` writer"]
pub struct W(crate::W<CSIC_BIST_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_BIST_START_SPEC>;
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
impl From<crate::W<CSIC_BIST_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_BIST_START_SPEC>) -> Self {
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
#[doc = "CSIC BIST Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_bist_start](index.html) module"]
pub struct CSIC_BIST_START_SPEC;
impl crate::RegisterSpec for CSIC_BIST_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_bist_start::R](R) reader structure"]
impl crate::Readable for CSIC_BIST_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_bist_start::W](W) writer structure"]
impl crate::Writable for CSIC_BIST_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_bist_start to value 0"]
impl crate::Resettable for CSIC_BIST_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
