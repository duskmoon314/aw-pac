#[doc = "Register `THS_ALARM_CTRL` reader"]
pub struct R(crate::R<THS_ALARM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_ALARM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_ALARM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_ALARM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THS_ALARM_CTRL` writer"]
pub struct W(crate::W<THS_ALARM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_ALARM_CTRL_SPEC>;
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
impl From<crate::W<THS_ALARM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_ALARM_CTRL_SPEC>) -> Self {
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
#[doc = "THS Alarm Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_alarm_ctrl](index.html) module"]
pub struct THS_ALARM_CTRL_SPEC;
impl crate::RegisterSpec for THS_ALARM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_alarm_ctrl::R](R) reader structure"]
impl crate::Readable for THS_ALARM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_alarm_ctrl::W](W) writer structure"]
impl crate::Writable for THS_ALARM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THS_ALARM_CTRL to value 0"]
impl crate::Resettable for THS_ALARM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
