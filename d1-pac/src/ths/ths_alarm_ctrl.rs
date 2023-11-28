#[doc = "Register `ths_alarm_ctrl` reader"]
pub type R = crate::R<THS_ALARM_CTRL_SPEC>;
#[doc = "Register `ths_alarm_ctrl` writer"]
pub type W = crate::W<THS_ALARM_CTRL_SPEC>;
#[doc = "Field `alarm_t_hyst` reader - Thermal sensor alarm threshold for hysteresis temperature"]
pub type ALARM_T_HYST_R = crate::FieldReader<u16>;
#[doc = "Field `alarm_t_hyst` writer - Thermal sensor alarm threshold for hysteresis temperature"]
pub type ALARM_T_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `alarm_t_hot` reader - Thermal sensor alarm threshold for hot temperature"]
pub type ALARM_T_HOT_R = crate::FieldReader<u16>;
#[doc = "Field `alarm_t_hot` writer - Thermal sensor alarm threshold for hot temperature"]
pub type ALARM_T_HOT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
    pub fn alarm_t_hyst(&mut self) -> ALARM_T_HYST_W<THS_ALARM_CTRL_SPEC> {
        ALARM_T_HYST_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Thermal sensor alarm threshold for hot temperature"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_t_hot(&mut self) -> ALARM_T_HOT_W<THS_ALARM_CTRL_SPEC> {
        ALARM_T_HOT_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "THS Alarm Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarm_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarm_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_ALARM_CTRL_SPEC;
impl crate::RegisterSpec for THS_ALARM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_alarm_ctrl::R`](R) reader structure"]
impl crate::Readable for THS_ALARM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_alarm_ctrl::W`](W) writer structure"]
impl crate::Writable for THS_ALARM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_alarm_ctrl to value 0x05a0_0684"]
impl crate::Resettable for THS_ALARM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05a0_0684;
}
