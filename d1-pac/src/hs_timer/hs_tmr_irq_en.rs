#[doc = "Register `hs_tmr_irq_en` reader"]
pub struct R(crate::R<HS_TMR_IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_TMR_IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_TMR_IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_TMR_IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hs_tmr_irq_en` writer"]
pub struct W(crate::W<HS_TMR_IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_TMR_IRQ_EN_SPEC>;
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
impl From<crate::W<HS_TMR_IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_TMR_IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hs_tmr_int_en[0-1]` reader - HSTimer Interrupt Enable"]
pub type HS_TMR_INT_EN_R = crate::BitReader<HS_TMR_INT_EN_A>;
#[doc = "HSTimer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_TMR_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<HS_TMR_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HS_TMR_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_TMR_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_TMR_INT_EN_A {
        match self.bits {
            false => HS_TMR_INT_EN_A::DISABLED,
            true => HS_TMR_INT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HS_TMR_INT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HS_TMR_INT_EN_A::ENABLED
    }
}
#[doc = "Field `hs_tmr_int_en[0-1]` writer - HSTimer Interrupt Enable"]
pub type HS_TMR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HS_TMR_IRQ_EN_SPEC, HS_TMR_INT_EN_A, O>;
impl<'a, const O: u8> HS_TMR_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HS_TMR_INT_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HS_TMR_INT_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "HSTimer Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn hs_tmr_int_en(&self, n: u8) -> HS_TMR_INT_EN_R {
        HS_TMR_INT_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - HSTimer Interrupt Enable"]
    #[inline(always)]
    pub fn hs_tmr0_int_en(&self) -> HS_TMR_INT_EN_R {
        HS_TMR_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSTimer Interrupt Enable"]
    #[inline(always)]
    pub fn hs_tmr1_int_en(&self) -> HS_TMR_INT_EN_R {
        HS_TMR_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "HSTimer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn hs_tmr_int_en<const O: u8>(&mut self) -> HS_TMR_INT_EN_W<O> {
        HS_TMR_INT_EN_W::new(self)
    }
    #[doc = "Bit 0 - HSTimer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr0_int_en(&mut self) -> HS_TMR_INT_EN_W<0> {
        HS_TMR_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - HSTimer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr1_int_en(&mut self) -> HS_TMR_INT_EN_W<1> {
        HS_TMR_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS Timer IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_tmr_irq_en](index.html) module"]
pub struct HS_TMR_IRQ_EN_SPEC;
impl crate::RegisterSpec for HS_TMR_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_tmr_irq_en::R](R) reader structure"]
impl crate::Readable for HS_TMR_IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_tmr_irq_en::W](W) writer structure"]
impl crate::Writable for HS_TMR_IRQ_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hs_tmr_irq_en to value 0"]
impl crate::Resettable for HS_TMR_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
