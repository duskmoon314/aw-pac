#[doc = "Register `GP_DATAL_INTC` reader"]
pub struct R(crate::R<GP_DATAL_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_DATAL_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_DATAL_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_DATAL_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_DATAL_INTC` writer"]
pub struct W(crate::W<GP_DATAL_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_DATAL_INTC_SPEC>;
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
impl From<crate::W<GP_DATAL_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_DATAL_INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Voltage Low Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_LOW_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CH_LOW_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CH_LOW_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `CH(0-1)_LOW_IRQ_EN` reader - Channel Voltage Low Available Interrupt Enable"]
pub struct CH_LOW_IRQ_EN_R(crate::FieldReader<bool>);
impl CH_LOW_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_LOW_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_LOW_IRQ_EN_A {
        match self.bits {
            false => CH_LOW_IRQ_EN_A::DISABLE,
            true => CH_LOW_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CH_LOW_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CH_LOW_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for CH_LOW_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-1)_LOW_IRQ_EN` const generic writer - Channel Voltage Low Available Interrupt Enable"]
pub struct CH_LOW_IRQ_EN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> CH_LOW_IRQ_EN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_LOW_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH_LOW_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CH_LOW_IRQ_EN_A::ENABLE)
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
    #[doc = "Channel Voltage Low Available Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn ch_low_irq_en(&self, n: usize) -> CH_LOW_IRQ_EN_R {
        CH_LOW_IRQ_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Enable"]
    #[inline(always)]
    pub fn ch0_low_irq_en(&self) -> CH_LOW_IRQ_EN_R {
        CH_LOW_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Enable"]
    #[inline(always)]
    pub fn ch1_low_irq_en(&self) -> CH_LOW_IRQ_EN_R {
        CH_LOW_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Voltage Low Available Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn ch_low_irq_en<const O: usize>(&mut self) -> CH_LOW_IRQ_EN_W<O> {
        CH_LOW_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 0 - Channel Voltage Low Available Interrupt Enable"]
    #[inline(always)]
    pub fn ch0_low_irq_en(&mut self) -> CH_LOW_IRQ_EN_W<0> {
        CH_LOW_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 1 - Channel Voltage Low Available Interrupt Enable"]
    #[inline(always)]
    pub fn ch1_low_irq_en(&mut self) -> CH_LOW_IRQ_EN_W<1> {
        CH_LOW_IRQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Data Low Interrupt Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_datal_intc](index.html) module"]
pub struct GP_DATAL_INTC_SPEC;
impl crate::RegisterSpec for GP_DATAL_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_datal_intc::R](R) reader structure"]
impl crate::Readable for GP_DATAL_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_datal_intc::W](W) writer structure"]
impl crate::Writable for GP_DATAL_INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_DATAL_INTC to value 0"]
impl crate::Resettable for GP_DATAL_INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
