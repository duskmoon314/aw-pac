#[doc = "Register `alarm_config` reader"]
pub type R = crate::R<ALARM_CONFIG_SPEC>;
#[doc = "Register `alarm_config` writer"]
pub type W = crate::W<ALARM_CONFIG_SPEC>;
#[doc = "Field `alarm_wakeup` reader - Configuration of alarm wake up output."]
pub type ALARM_WAKEUP_R = crate::BitReader<ALARM_WAKEUP_A>;
#[doc = "Configuration of alarm wake up output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM_WAKEUP_A {
    #[doc = "0: Disable alarm wake up output"]
    DISABLE = 0,
    #[doc = "1: Enable alarm wake up output"]
    ENABLE = 1,
}
impl From<ALARM_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_WAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALARM_WAKEUP_A {
        match self.bits {
            false => ALARM_WAKEUP_A::DISABLE,
            true => ALARM_WAKEUP_A::ENABLE,
        }
    }
    #[doc = "Disable alarm wake up output"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALARM_WAKEUP_A::DISABLE
    }
    #[doc = "Enable alarm wake up output"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALARM_WAKEUP_A::ENABLE
    }
}
#[doc = "Field `alarm_wakeup` writer - Configuration of alarm wake up output."]
pub type ALARM_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG, ALARM_WAKEUP_A>;
impl<'a, REG> ALARM_WAKEUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm wake up output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM_WAKEUP_A::DISABLE)
    }
    #[doc = "Enable alarm wake up output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM_WAKEUP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Configuration of alarm wake up output."]
    #[inline(always)]
    pub fn alarm_wakeup(&self) -> ALARM_WAKEUP_R {
        ALARM_WAKEUP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration of alarm wake up output."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_wakeup(&mut self) -> ALARM_WAKEUP_W<ALARM_CONFIG_SPEC> {
        ALARM_WAKEUP_W::new(self, 0)
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
#[doc = "Alarm Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM_CONFIG_SPEC;
impl crate::RegisterSpec for ALARM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm_config::R`](R) reader structure"]
impl crate::Readable for ALARM_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm_config::W`](W) writer structure"]
impl crate::Writable for ALARM_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm_config to value 0"]
impl crate::Resettable for ALARM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
