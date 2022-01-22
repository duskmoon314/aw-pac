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
#[doc = "Field `WDOG_OUTPUT_CONFIG` reader - Configure the valid time for the watchdog reset signal."]
pub struct WDOG_OUTPUT_CONFIG_R(crate::FieldReader<u16, u16>);
impl WDOG_OUTPUT_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WDOG_OUTPUT_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_OUTPUT_CONFIG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_OUTPUT_CONFIG` writer - Configure the valid time for the watchdog reset signal."]
pub struct WDOG_OUTPUT_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_OUTPUT_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
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
    pub fn wdog_output_config(&mut self) -> WDOG_OUTPUT_CONFIG_W {
        WDOG_OUTPUT_CONFIG_W { w: self }
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
}
#[doc = "`reset()` method sets wdog_output_cfg to value 0"]
impl crate::Resettable for WDOG_OUTPUT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
