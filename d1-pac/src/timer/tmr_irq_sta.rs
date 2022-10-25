#[doc = "Register `tmr_irq_sta` reader"]
pub struct R(crate::R<TMR_IRQ_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_IRQ_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_IRQ_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_IRQ_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tmr_irq_sta` writer"]
pub struct W(crate::W<TMR_IRQ_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_IRQ_STA_SPEC>;
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
impl From<crate::W<TMR_IRQ_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_IRQ_STA_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> TMR0_IRQ_PEND_A {
        match self.bits {
            false => TMR0_IRQ_PEND_A::NO_EFFECT,
            true => TMR0_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TMR0_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TMR0_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `tmr0_irq_pend` writer - "]
pub type TMR0_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TMR_IRQ_STA_SPEC, TMR0_IRQ_PEND_A, O>;
impl<'a, const O: u8> TMR0_IRQ_PEND_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TMR0_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Indicates that the interval value of the timer 0 is reached. Write 1 to clear the pending status."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> TMR1_IRQ_PEND_A {
        match self.bits {
            false => TMR1_IRQ_PEND_A::NO_EFFECT,
            true => TMR1_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TMR1_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TMR1_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `tmr1_irq_pend` writer - "]
pub type TMR1_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TMR_IRQ_STA_SPEC, TMR1_IRQ_PEND_A, O>;
impl<'a, const O: u8> TMR1_IRQ_PEND_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TMR1_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Indicates that the interval value of the timer 1 is reached. Write 1 to clear the pending status."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn tmr0_irq_pend(&mut self) -> TMR0_IRQ_PEND_W<0> {
        TMR0_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_irq_pend(&mut self) -> TMR1_IRQ_PEND_W<1> {
        TMR1_IRQ_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_irq_sta](index.html) module"]
pub struct TMR_IRQ_STA_SPEC;
impl crate::RegisterSpec for TMR_IRQ_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_irq_sta::R](R) reader structure"]
impl crate::Readable for TMR_IRQ_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr_irq_sta::W](W) writer structure"]
impl crate::Writable for TMR_IRQ_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tmr_irq_sta to value 0"]
impl crate::Resettable for TMR_IRQ_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
