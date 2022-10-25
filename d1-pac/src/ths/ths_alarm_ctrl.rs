#[doc = "Register `ths_alarm_ctrl` reader"]
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
#[doc = "Register `ths_alarm_ctrl` writer"]
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
#[doc = "Field `alarm_t_hyst` reader - Thermal sensor alarm threshold for hysteresis temperature"]
pub type ALARM_T_HYST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `alarm_t_hyst` writer - Thermal sensor alarm threshold for hysteresis temperature"]
pub type ALARM_T_HYST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THS_ALARM_CTRL_SPEC, u16, u16, 12, O>;
#[doc = "Field `alarm_t_hot` reader - Thermal sensor alarm threshold for hot temperature"]
pub type ALARM_T_HOT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `alarm_t_hot` writer - Thermal sensor alarm threshold for hot temperature"]
pub type ALARM_T_HOT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THS_ALARM_CTRL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Thermal sensor alarm threshold for hysteresis temperature"]
    #[inline(always)]
    pub fn alarm_t_hyst(&self) -> ALARM_T_HYST_R {
        ALARM_T_HYST_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Thermal sensor alarm threshold for hot temperature"]
    #[inline(always)]
    pub fn alarm_t_hot(&self) -> ALARM_T_HOT_R {
        ALARM_T_HOT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Thermal sensor alarm threshold for hysteresis temperature"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_t_hyst(&mut self) -> ALARM_T_HYST_W<0> {
        ALARM_T_HYST_W::new(self)
    }
    #[doc = "Bits 16:27 - Thermal sensor alarm threshold for hot temperature"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_t_hot(&mut self) -> ALARM_T_HOT_W<16> {
        ALARM_T_HOT_W::new(self)
    }
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_alarm_ctrl to value 0x05a0_0684"]
impl crate::Resettable for THS_ALARM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05a0_0684;
}
