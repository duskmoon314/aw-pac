#[doc = "Register `hs_tmr_irq_stas` reader"]
pub struct R(crate::R<HS_TMR_IRQ_STAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_TMR_IRQ_STAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_TMR_IRQ_STAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_TMR_IRQ_STAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hs_tmr_irq_stas` writer"]
pub struct W(crate::W<HS_TMR_IRQ_STAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_TMR_IRQ_STAS_SPEC>;
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
impl From<crate::W<HS_TMR_IRQ_STAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_TMR_IRQ_STAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hs_tmr_irq_pend[0-1]` reader - HSTimer IRQ Pending"]
pub type HS_TMR_IRQ_PEND_R = crate::BitReader<HS_TMR_IRQ_PEND_A>;
#[doc = "HSTimer IRQ Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_TMR_IRQ_PEND_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<HS_TMR_IRQ_PEND_A> for bool {
    #[inline(always)]
    fn from(variant: HS_TMR_IRQ_PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_TMR_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_TMR_IRQ_PEND_A {
        match self.bits {
            false => HS_TMR_IRQ_PEND_A::NO_EFFECT,
            true => HS_TMR_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HS_TMR_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == HS_TMR_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `hs_tmr_irq_pend[0-1]` writer - HSTimer IRQ Pending"]
pub type HS_TMR_IRQ_PEND_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, HS_TMR_IRQ_STAS_SPEC, HS_TMR_IRQ_PEND_A, O>;
impl<'a, const O: u8> HS_TMR_IRQ_PEND_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(HS_TMR_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(HS_TMR_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "HSTimer IRQ Pending"]
    #[inline(always)]
    pub unsafe fn hs_tmr_irq_pend(&self, n: u8) -> HS_TMR_IRQ_PEND_R {
        HS_TMR_IRQ_PEND_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - HSTimer IRQ Pending"]
    #[inline(always)]
    pub fn hs_tmr0_irq_pend(&self) -> HS_TMR_IRQ_PEND_R {
        HS_TMR_IRQ_PEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSTimer IRQ Pending"]
    #[inline(always)]
    pub fn hs_tmr1_irq_pend(&self) -> HS_TMR_IRQ_PEND_R {
        HS_TMR_IRQ_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "HSTimer IRQ Pending"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn hs_tmr_irq_pend<const O: u8>(&mut self) -> HS_TMR_IRQ_PEND_W<O> {
        HS_TMR_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 0 - HSTimer IRQ Pending"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr0_irq_pend(&mut self) -> HS_TMR_IRQ_PEND_W<0> {
        HS_TMR_IRQ_PEND_W::new(self)
    }
    #[doc = "Bit 1 - HSTimer IRQ Pending"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr1_irq_pend(&mut self) -> HS_TMR_IRQ_PEND_W<1> {
        HS_TMR_IRQ_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_tmr_irq_stas](index.html) module"]
pub struct HS_TMR_IRQ_STAS_SPEC;
impl crate::RegisterSpec for HS_TMR_IRQ_STAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_tmr_irq_stas::R](R) reader structure"]
impl crate::Readable for HS_TMR_IRQ_STAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_tmr_irq_stas::W](W) writer structure"]
impl crate::Writable for HS_TMR_IRQ_STAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets hs_tmr_irq_stas to value 0"]
impl crate::Resettable for HS_TMR_IRQ_STAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
