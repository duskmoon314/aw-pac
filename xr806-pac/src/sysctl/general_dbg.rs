#[doc = "Register `GENERAL_DBG%s` reader"]
pub struct R(crate::R<GENERAL_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENERAL_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENERAL_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENERAL_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENERAL_DBG%s` writer"]
pub struct W(crate::W<GENERAL_DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENERAL_DBG_SPEC>;
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
impl From<crate::W<GENERAL_DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENERAL_DBG_SPEC>) -> Self {
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
#[doc = "General Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [general_dbg](index.html) module"]
pub struct GENERAL_DBG_SPEC;
impl crate::RegisterSpec for GENERAL_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [general_dbg::R](R) reader structure"]
impl crate::Readable for GENERAL_DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [general_dbg::W](W) writer structure"]
impl crate::Writable for GENERAL_DBG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GENERAL_DBG%s to value 0"]
impl crate::Resettable for GENERAL_DBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
