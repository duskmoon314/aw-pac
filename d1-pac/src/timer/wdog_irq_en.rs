#[doc = "Register `wdog_irq_en` reader"]
pub struct R(crate::R<WDOG_IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wdog_irq_en` writer"]
pub struct W(crate::W<WDOG_IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_IRQ_EN_SPEC>;
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
impl From<crate::W<WDOG_IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<WDOG_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `wdog_irq_en` reader - "]
pub struct WDOG_IRQ_EN_R(crate::FieldReader<bool, WDOG_IRQ_EN_A>);
impl WDOG_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_IRQ_EN_A {
        match self.bits {
            false => WDOG_IRQ_EN_A::DISABLED,
            true => WDOG_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WDOG_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WDOG_IRQ_EN_A::ENABLED
    }
}
impl core::ops::Deref for WDOG_IRQ_EN_R {
    type Target = crate::FieldReader<bool, WDOG_IRQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wdog_irq_en` writer - "]
pub struct WDOG_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDOG_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDOG_IRQ_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdog_irq_en(&self) -> WDOG_IRQ_EN_R {
        WDOG_IRQ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdog_irq_en(&mut self) -> WDOG_IRQ_EN_W {
        WDOG_IRQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_irq_en](index.html) module"]
pub struct WDOG_IRQ_EN_SPEC;
impl crate::RegisterSpec for WDOG_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog_irq_en::R](R) reader structure"]
impl crate::Readable for WDOG_IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_irq_en::W](W) writer structure"]
impl crate::Writable for WDOG_IRQ_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets wdog_irq_en to value 0"]
impl crate::Resettable for WDOG_IRQ_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
