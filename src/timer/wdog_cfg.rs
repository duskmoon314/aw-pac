#[doc = "Register `wdog_cfg` reader"]
pub struct R(crate::R<WDOG_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_cfg` writer"]
pub struct W(crate::W<WDOG_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_CFG_SPEC>;
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
impl From<crate::W<WDOG_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_CFG_SPEC>) -> Self {
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
#[doc = "Watchdog Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_cfg](index.html) module"]
pub struct WDOG_CFG_SPEC;
impl crate::RegisterSpec for WDOG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_cfg::R](R) reader structure"]
impl crate::Readable for WDOG_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_cfg::W](W) writer structure"]
impl crate::Writable for WDOG_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdog_cfg to value 0"]
impl crate::Resettable for WDOG_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
