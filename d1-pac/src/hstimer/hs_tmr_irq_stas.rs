#[doc = "Register `HS_TMR_IRQ_STAS` reader"]
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
#[doc = "Register `HS_TMR_IRQ_STAS` writer"]
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
#[doc = "HSTimer IRQ Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `HS_TMR(0-1)_IRQ_PEND` reader - HSTimer IRQ Pending"]
pub struct HS_TMR_IRQ_PEND_R(crate::FieldReader<bool, HS_TMR_IRQ_PEND_A>);
impl HS_TMR_IRQ_PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS_TMR_IRQ_PEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HS_TMR_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == HS_TMR_IRQ_PEND_A::PENDING
    }
}
impl core::ops::Deref for HS_TMR_IRQ_PEND_R {
    type Target = crate::FieldReader<bool, HS_TMR_IRQ_PEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `HS_TMR(0-1)_IRQ_PEND` writer - HSTimer IRQ Pending"]
pub struct HS_TMR_IRQ_PEND_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> HS_TMR_IRQ_PEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_TMR_IRQ_PEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `HS_TMR(0-1)_IRQ_PEND` const generic writer - HSTimer IRQ Pending"]
pub struct HS_TMR_IRQ_PEND_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> HS_TMR_IRQ_PEND_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_TMR_IRQ_PEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "HSTimer IRQ Pending"]
    #[inline(always)]
    pub unsafe fn hs_tmr_irq_pend(&self, n: usize) -> HS_TMR_IRQ_PEND_R {
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
    pub unsafe fn hs_tmr_irq_pend(&mut self, n: usize) -> HS_TMR_IRQ_PEND_W {
        HS_TMR_IRQ_PEND_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - HSTimer IRQ Pending"]
    #[inline(always)]
    pub fn hs_tmr0_irq_pend(&mut self) -> HS_TMR_IRQ_PEND_CGW<0> {
        HS_TMR_IRQ_PEND_CGW { w: self }
    }
    #[doc = "Bit 1 - HSTimer IRQ Pending"]
    #[inline(always)]
    pub fn hs_tmr1_irq_pend(&mut self) -> HS_TMR_IRQ_PEND_CGW<1> {
        HS_TMR_IRQ_PEND_CGW { w: self }
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
}
#[doc = "`reset()` method sets HS_TMR_IRQ_STAS to value 0"]
impl crate::Resettable for HS_TMR_IRQ_STAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
