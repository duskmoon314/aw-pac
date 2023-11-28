#[doc = "Register `ths_alarm_intc` reader"]
pub type R = crate::R<THS_ALARM_INTC_SPEC>;
#[doc = "Register `ths_alarm_intc` writer"]
pub type W = crate::W<THS_ALARM_INTC_SPEC>;
#[doc = "Field `alarm_int_en` reader - Enable the alarm interrupt for the sensor"]
pub type ALARM_INT_EN_R = crate::BitReader<ALARM_INT_EN_A>;
#[doc = "Enable the alarm interrupt for the sensor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ALARM_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALARM_INT_EN_A {
        match self.bits {
            false => ALARM_INT_EN_A::DISABLE,
            true => ALARM_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALARM_INT_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALARM_INT_EN_A::ENABLE
    }
}
#[doc = "Field `alarm_int_en` writer - Enable the alarm interrupt for the sensor"]
pub type ALARM_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, ALARM_INT_EN_A>;
impl<'a, REG> ALARM_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the alarm interrupt for the sensor"]
    #[inline(always)]
    pub fn alarm_int_en(&self) -> ALARM_INT_EN_R {
        ALARM_INT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the alarm interrupt for the sensor"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_int_en(&mut self) -> ALARM_INT_EN_W<THS_ALARM_INTC_SPEC> {
        ALARM_INT_EN_W::new(self, 0)
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
#[doc = "THS Alarm Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarm_intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarm_intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_ALARM_INTC_SPEC;
impl crate::RegisterSpec for THS_ALARM_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_alarm_intc::R`](R) reader structure"]
impl crate::Readable for THS_ALARM_INTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_alarm_intc::W`](W) writer structure"]
impl crate::Writable for THS_ALARM_INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_alarm_intc to value 0"]
impl crate::Resettable for THS_ALARM_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
