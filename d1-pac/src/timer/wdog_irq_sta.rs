#[doc = "Register `wdog_irq_sta` reader"]
pub type R = crate::R<WDOG_IRQ_STA_SPEC>;
#[doc = "Register `wdog_irq_sta` writer"]
pub type W = crate::W<WDOG_IRQ_STA_SPEC>;
#[doc = "Field `wdog_irq_pend` reader - "]
pub type WDOG_IRQ_PEND_R = crate::BitReader<WDOG_IRQ_PEND_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: Indicates that the interval value of the watchdog is reached."]
    PENDING = 1,
}
impl From<WDOG_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDOG_IRQ_PEND_A {
        match self.bits {
            false => WDOG_IRQ_PEND_A::NO_EFFECT,
            true => WDOG_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == WDOG_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Indicates that the interval value of the watchdog is reached."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == WDOG_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `wdog_irq_pend` writer - "]
pub type WDOG_IRQ_PEND_W<'a, REG> = crate::BitWriter<'a, REG, WDOG_IRQ_PEND_A>;
impl<'a, REG> WDOG_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Indicates that the interval value of the watchdog is reached."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdog_irq_pend(&self) -> WDOG_IRQ_PEND_R {
        WDOG_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_irq_pend(&mut self) -> WDOG_IRQ_PEND_W<WDOG_IRQ_STA_SPEC> {
        WDOG_IRQ_PEND_W::new(self, 0)
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
#[doc = "Watchdog Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_irq_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_irq_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_IRQ_STA_SPEC;
impl crate::RegisterSpec for WDOG_IRQ_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_irq_sta::R`](R) reader structure"]
impl crate::Readable for WDOG_IRQ_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog_irq_sta::W`](W) writer structure"]
impl crate::Writable for WDOG_IRQ_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_irq_sta to value 0"]
impl crate::Resettable for WDOG_IRQ_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
