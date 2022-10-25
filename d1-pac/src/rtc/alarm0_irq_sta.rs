#[doc = "Register `alarm0_irq_sta` reader"]
pub struct R(crate::R<ALARM0_IRQ_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM0_IRQ_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM0_IRQ_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM0_IRQ_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `alarm0_irq_sta` writer"]
pub struct W(crate::W<ALARM0_IRQ_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM0_IRQ_STA_SPEC>;
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
impl From<crate::W<ALARM0_IRQ_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM0_IRQ_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `alarm0_irq_pend` reader - Alarm 0 IRQ Pending bit"]
pub type ALARM0_IRQ_PEND_R = crate::BitReader<ALARM0_IRQ_PEND_A>;
#[doc = "Alarm 0 IRQ Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM0_IRQ_PEND_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Pending, alarm 0 counter value is reached\n\nIf alarm 0 irq enable is set to 1, the pending bit will be sent to the interrupt controller."]
    PENDING = 1,
}
impl From<ALARM0_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM0_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM0_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM0_IRQ_PEND_A {
        match self.bits {
            false => ALARM0_IRQ_PEND_A::NO_EFFECT,
            true => ALARM0_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ALARM0_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ALARM0_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `alarm0_irq_pend` writer - Alarm 0 IRQ Pending bit"]
pub type ALARM0_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, ALARM0_IRQ_STA_SPEC, ALARM0_IRQ_PEND_A, O>;
impl<'a, const O: u8> ALARM0_IRQ_PEND_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ALARM0_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "Pending, alarm 0 counter value is reached\n\nIf alarm 0 irq enable is set to 1, the pending bit will be sent to the interrupt controller."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ALARM0_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm 0 IRQ Pending bit"]
    #[inline(always)]
    pub fn alarm0_irq_pend(&self) -> ALARM0_IRQ_PEND_R {
        ALARM0_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 0 IRQ Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0_irq_pend(&mut self) -> ALARM0_IRQ_PEND_W<0> {
        ALARM0_IRQ_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 0 IRQ Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm0_irq_sta](index.html) module"]
pub struct ALARM0_IRQ_STA_SPEC;
impl crate::RegisterSpec for ALARM0_IRQ_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm0_irq_sta::R](R) reader structure"]
impl crate::Readable for ALARM0_IRQ_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm0_irq_sta::W](W) writer structure"]
impl crate::Writable for ALARM0_IRQ_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets alarm0_irq_sta to value 0"]
impl crate::Resettable for ALARM0_IRQ_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
