#[doc = "Register `ths_alarmo_ints` reader"]
pub type R = crate::R<THS_ALARMO_INTS_SPEC>;
#[doc = "Register `ths_alarmo_ints` writer"]
pub type W = crate::W<THS_ALARMO_INTS_SPEC>;
#[doc = "Field `alarm_off_sts` reader - Alarm interrupt off pending for sensor\n\nWrite 1 to clear the pending status."]
pub type ALARM_OFF_STS_R = crate::BitReader<ALARM_OFF_STS_A>;
#[doc = "Alarm interrupt off pending for sensor\n\nWrite 1 to clear the pending status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM_OFF_STS_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Pending"]
    PENDING = 1,
}
impl From<ALARM_OFF_STS_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_OFF_STS_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_OFF_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALARM_OFF_STS_A {
        match self.bits {
            false => ALARM_OFF_STS_A::NO_EFFECT,
            true => ALARM_OFF_STS_A::PENDING,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ALARM_OFF_STS_A::NO_EFFECT
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ALARM_OFF_STS_A::PENDING
    }
}
#[doc = "Field `alarm_off_sts` writer - Alarm interrupt off pending for sensor\n\nWrite 1 to clear the pending status."]
pub type ALARM_OFF_STS_W<'a, REG> = crate::BitWriter1C<'a, REG, ALARM_OFF_STS_A>;
impl<'a, REG> ALARM_OFF_STS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM_OFF_STS_A::NO_EFFECT)
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM_OFF_STS_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm interrupt off pending for sensor\n\nWrite 1 to clear the pending status."]
    #[inline(always)]
    pub fn alarm_off_sts(&self) -> ALARM_OFF_STS_R {
        ALARM_OFF_STS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm interrupt off pending for sensor\n\nWrite 1 to clear the pending status."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_off_sts(&mut self) -> ALARM_OFF_STS_W<THS_ALARMO_INTS_SPEC> {
        ALARM_OFF_STS_W::new(self, 0)
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
#[doc = "THS_ALARM0_INTS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_alarmo_ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_alarmo_ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_ALARMO_INTS_SPEC;
impl crate::RegisterSpec for THS_ALARMO_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_alarmo_ints::R`](R) reader structure"]
impl crate::Readable for THS_ALARMO_INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_alarmo_ints::W`](W) writer structure"]
impl crate::Writable for THS_ALARMO_INTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets ths_alarmo_ints to value 0"]
impl crate::Resettable for THS_ALARMO_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
