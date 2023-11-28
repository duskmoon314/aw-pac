#[doc = "Register `tmr_irq_sta` reader"]
pub type R = crate::R<TMR_IRQ_STA_SPEC>;
#[doc = "Register `tmr_irq_sta` writer"]
pub type W = crate::W<TMR_IRQ_STA_SPEC>;
#[doc = "Field `tmr0_irq_pend` reader - "]
pub type TMR0_IRQ_PEND_R = crate::BitReader<TMR0_IRQ_PEND_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR0_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: Indicates that the interval value of the timer 0 is reached. Write 1 to clear the pending status."]
    PENDING = 1,
}
impl From<TMR0_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR0_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR0_IRQ_PEND_A {
        match self.bits {
            false => TMR0_IRQ_PEND_A::NO_EFFECT,
            true => TMR0_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TMR0_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Indicates that the interval value of the timer 0 is reached. Write 1 to clear the pending status."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TMR0_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `tmr0_irq_pend` writer - "]
pub type TMR0_IRQ_PEND_W<'a, REG> = crate::BitWriter<'a, REG, TMR0_IRQ_PEND_A>;
impl<'a, REG> TMR0_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TMR0_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Indicates that the interval value of the timer 0 is reached. Write 1 to clear the pending status."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TMR0_IRQ_PEND_A::PENDING)
    }
}
#[doc = "Field `tmr1_irq_pend` reader - "]
pub type TMR1_IRQ_PEND_R = crate::BitReader<TMR1_IRQ_PEND_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR1_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: Indicates that the interval value of the timer 1 is reached. Write 1 to clear the pending status."]
    PENDING = 1,
}
impl From<TMR1_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR1_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR1_IRQ_PEND_A {
        match self.bits {
            false => TMR1_IRQ_PEND_A::NO_EFFECT,
            true => TMR1_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TMR1_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Indicates that the interval value of the timer 1 is reached. Write 1 to clear the pending status."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TMR1_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `tmr1_irq_pend` writer - "]
pub type TMR1_IRQ_PEND_W<'a, REG> = crate::BitWriter<'a, REG, TMR1_IRQ_PEND_A>;
impl<'a, REG> TMR1_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Indicates that the interval value of the timer 1 is reached. Write 1 to clear the pending status."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmr0_irq_pend(&self) -> TMR0_IRQ_PEND_R {
        TMR0_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmr1_irq_pend(&self) -> TMR1_IRQ_PEND_R {
        TMR1_IRQ_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmr0_irq_pend(&mut self) -> TMR0_IRQ_PEND_W<TMR_IRQ_STA_SPEC> {
        TMR0_IRQ_PEND_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_irq_pend(&mut self) -> TMR1_IRQ_PEND_W<TMR_IRQ_STA_SPEC> {
        TMR1_IRQ_PEND_W::new(self, 1)
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
#[doc = "Timer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr_irq_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr_irq_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMR_IRQ_STA_SPEC;
impl crate::RegisterSpec for TMR_IRQ_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr_irq_sta::R`](R) reader structure"]
impl crate::Readable for TMR_IRQ_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmr_irq_sta::W`](W) writer structure"]
impl crate::Writable for TMR_IRQ_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tmr_irq_sta to value 0"]
impl crate::Resettable for TMR_IRQ_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
