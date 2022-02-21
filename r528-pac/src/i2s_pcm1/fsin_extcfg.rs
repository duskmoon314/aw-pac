#[doc = "Register `FsinEXTCFG` reader"]
pub struct R(crate::R<FSINEXTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSINEXTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSINEXTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSINEXTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FsinEXTCFG` writer"]
pub struct W(crate::W<FSINEXTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSINEXTCFG_SPEC>;
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
impl From<crate::W<FSINEXTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSINEXTCFG_SPEC>) -> Self {
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
#[doc = "ASRC Input Sample Pulse Extend Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsin_extcfg](index.html) module"]
pub struct FSINEXTCFG_SPEC;
impl crate::RegisterSpec for FSINEXTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsin_extcfg::R](R) reader structure"]
impl crate::Readable for FSINEXTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsin_extcfg::W](W) writer structure"]
impl crate::Writable for FSINEXTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FsinEXTCFG to value 0"]
impl crate::Resettable for FSINEXTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
