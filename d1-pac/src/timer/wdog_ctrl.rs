#[doc = "Register `wdog_ctrl` reader"]
pub type R = crate::R<WDOG_CTRL_SPEC>;
#[doc = "Register `wdog_ctrl` writer"]
pub type W = crate::W<WDOG_CTRL_SPEC>;
#[doc = "Field `wdog_restart` reader - Watchdog Restart"]
pub type WDOG_RESTART_R = crate::BitReader<WDOG_RESTART_A>;
#[doc = "Watchdog Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG_RESTART_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RESTART = 1,
}
impl From<WDOG_RESTART_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_RESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG_RESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDOG_RESTART_A {
        match self.bits {
            false => WDOG_RESTART_A::NO_EFFECT,
            true => WDOG_RESTART_A::RESTART,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == WDOG_RESTART_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == WDOG_RESTART_A::RESTART
    }
}
#[doc = "Field `wdog_restart` writer - Watchdog Restart"]
pub type WDOG_RESTART_W<'a, REG> = crate::BitWriter1S<'a, REG, WDOG_RESTART_A>;
impl<'a, REG> WDOG_RESTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_RESTART_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_RESTART_A::RESTART)
    }
}
#[doc = "Field `wdog_key_field` writer - Watchdog Key Field"]
pub type WDOG_KEY_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdog_restart(&self) -> WDOG_RESTART_R {
        WDOG_RESTART_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_restart(&mut self) -> WDOG_RESTART_W<WDOG_CTRL_SPEC> {
        WDOG_RESTART_W::new(self, 0)
    }
    #[doc = "Bits 1:12 - Watchdog Key Field"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_key_field(&mut self) -> WDOG_KEY_FIELD_W<WDOG_CTRL_SPEC> {
        WDOG_KEY_FIELD_W::new(self, 1)
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
#[doc = "Watchdog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_CTRL_SPEC;
impl crate::RegisterSpec for WDOG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_ctrl::R`](R) reader structure"]
impl crate::Readable for WDOG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog_ctrl::W`](W) writer structure"]
impl crate::Writable for WDOG_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets wdog_ctrl to value 0"]
impl crate::Resettable for WDOG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
