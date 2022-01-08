#[doc = "Register `LED_T01_TIMING_CTRL` reader"]
pub struct R(crate::R<LED_T01_TIMING_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LED_T01_TIMING_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LED_T01_TIMING_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LED_T01_TIMING_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LED_T01_TIMING_CTRL` writer"]
pub struct W(crate::W<LED_T01_TIMING_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LED_T01_TIMING_CTRL_SPEC>;
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
impl From<crate::W<LED_T01_TIMING_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LED_T01_TIMING_CTRL_SPEC>) -> Self {
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
#[doc = "LEDC T0 T1 Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led_t01_timing_ctrl](index.html) module"]
pub struct LED_T01_TIMING_CTRL_SPEC;
impl crate::RegisterSpec for LED_T01_TIMING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [led_t01_timing_ctrl::R](R) reader structure"]
impl crate::Readable for LED_T01_TIMING_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [led_t01_timing_ctrl::W](W) writer structure"]
impl crate::Writable for LED_T01_TIMING_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LED_T01_TIMING_CTRL to value 0"]
impl crate::Resettable for LED_T01_TIMING_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
