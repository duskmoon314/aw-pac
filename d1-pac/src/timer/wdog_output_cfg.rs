#[doc = "Register `wdog_output_cfg` reader"]
pub struct R(crate::R<WDOG_OUTPUT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_OUTPUT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_OUTPUT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_OUTPUT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_output_cfg` writer"]
pub struct W(crate::W<WDOG_OUTPUT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_OUTPUT_CFG_SPEC>;
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
impl From<crate::W<WDOG_OUTPUT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_OUTPUT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wdog_output_config` reader - Configure the valid time for the watchdog reset signal."]
pub type WDOG_OUTPUT_CONFIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wdog_output_config` writer - Configure the valid time for the watchdog reset signal."]
pub type WDOG_OUTPUT_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDOG_OUTPUT_CFG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Configure the valid time for the watchdog reset signal."]
    #[inline(always)]
    pub fn wdog_output_config(&self) -> WDOG_OUTPUT_CONFIG_R {
        WDOG_OUTPUT_CONFIG_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configure the valid time for the watchdog reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn wdog_output_config(&mut self) -> WDOG_OUTPUT_CONFIG_W<0> {
        WDOG_OUTPUT_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_output_cfg](index.html) module"]
pub struct WDOG_OUTPUT_CFG_SPEC;
impl crate::RegisterSpec for WDOG_OUTPUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_output_cfg::R](R) reader structure"]
impl crate::Readable for WDOG_OUTPUT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_output_cfg::W](W) writer structure"]
impl crate::Writable for WDOG_OUTPUT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_output_cfg to value 0"]
impl crate::Resettable for WDOG_OUTPUT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
