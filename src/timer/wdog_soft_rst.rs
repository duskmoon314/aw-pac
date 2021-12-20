#[doc = "Register `wdog_soft_rst` reader"]
pub struct R(crate::R<WDOG_SOFT_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_SOFT_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_SOFT_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_SOFT_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_soft_rst` writer"]
pub struct W(crate::W<WDOG_SOFT_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_SOFT_RST_SPEC>;
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
impl From<crate::W<WDOG_SOFT_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_SOFT_RST_SPEC>) -> Self {
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
#[doc = "Watchdog Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_soft_rst](index.html) module"]
pub struct WDOG_SOFT_RST_SPEC;
impl crate::RegisterSpec for WDOG_SOFT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_soft_rst::R](R) reader structure"]
impl crate::Readable for WDOG_SOFT_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_soft_rst::W](W) writer structure"]
impl crate::Writable for WDOG_SOFT_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdog_soft_rst to value 0"]
impl crate::Resettable for WDOG_SOFT_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
