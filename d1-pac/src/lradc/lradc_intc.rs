#[doc = "Register `lradc_intc` reader"]
pub struct R(crate::R<LRADC_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRADC_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRADC_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRADC_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lradc_intc` writer"]
pub struct W(crate::W<LRADC_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRADC_INTC_SPEC>;
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
impl From<crate::W<LRADC_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRADC_INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc0_data_irq_en` reader - ADC0 Data Interrupt Enable"]
pub type ADC0_DATA_IRQ_EN_R = crate::BitReader<ADC0_DATA_IRQ_EN_A>;
#[doc = "ADC0 Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_DATA_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC0_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_DATA_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_DATA_IRQ_EN_A {
        match self.bits {
            false => ADC0_DATA_IRQ_EN_A::DISABLE,
            true => ADC0_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_DATA_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `adc0_data_irq_en` writer - ADC0 Data Interrupt Enable"]
pub type ADC0_DATA_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_INTC_SPEC, ADC0_DATA_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC0_DATA_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_DATA_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `adc0_keydown_irq_en` reader - ADC0 Key Down Interrupt Enable"]
pub type ADC0_KEYDOWN_IRQ_EN_R = crate::BitReader<ADC0_KEYDOWN_IRQ_EN_A>;
#[doc = "ADC0 Key Down Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_KEYDOWN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC0_KEYDOWN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_KEYDOWN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_KEYDOWN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_KEYDOWN_IRQ_EN_A {
        match self.bits {
            false => ADC0_KEYDOWN_IRQ_EN_A::DISABLE,
            true => ADC0_KEYDOWN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_KEYDOWN_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_KEYDOWN_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `adc0_keydown_irq_en` writer - ADC0 Key Down Interrupt Enable"]
pub type ADC0_KEYDOWN_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_INTC_SPEC, ADC0_KEYDOWN_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC0_KEYDOWN_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_KEYDOWN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_KEYDOWN_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `adc0_hold_irq_en` reader - ADC0 Hold Key Interrupt Enable"]
pub type ADC0_HOLD_IRQ_EN_R = crate::BitReader<ADC0_HOLD_IRQ_EN_A>;
#[doc = "ADC0 Hold Key Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_HOLD_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC0_HOLD_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_HOLD_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_HOLD_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_HOLD_IRQ_EN_A {
        match self.bits {
            false => ADC0_HOLD_IRQ_EN_A::DISABLE,
            true => ADC0_HOLD_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_HOLD_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_HOLD_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `adc0_hold_irq_en` writer - ADC0 Hold Key Interrupt Enable"]
pub type ADC0_HOLD_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_INTC_SPEC, ADC0_HOLD_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC0_HOLD_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_HOLD_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_HOLD_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `adc0_alrdy_hold_irq_en` reader - ADC0 Already Hold Key Interrupt Enable"]
pub type ADC0_ALRDY_HOLD_IRQ_EN_R = crate::BitReader<ADC0_ALRDY_HOLD_IRQ_EN_A>;
#[doc = "ADC0 Already Hold Key Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_ALRDY_HOLD_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC0_ALRDY_HOLD_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_ALRDY_HOLD_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_ALRDY_HOLD_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_ALRDY_HOLD_IRQ_EN_A {
        match self.bits {
            false => ADC0_ALRDY_HOLD_IRQ_EN_A::DISABLE,
            true => ADC0_ALRDY_HOLD_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_ALRDY_HOLD_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_ALRDY_HOLD_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `adc0_alrdy_hold_irq_en` writer - ADC0 Already Hold Key Interrupt Enable"]
pub type ADC0_ALRDY_HOLD_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_INTC_SPEC, ADC0_ALRDY_HOLD_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC0_ALRDY_HOLD_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_ALRDY_HOLD_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_ALRDY_HOLD_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `adc0_keyup_irq_en` reader - ADC0 Key Up Interrupt Enable"]
pub type ADC0_KEYUP_IRQ_EN_R = crate::BitReader<ADC0_KEYUP_IRQ_EN_A>;
#[doc = "ADC0 Key Up Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_KEYUP_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC0_KEYUP_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_KEYUP_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_KEYUP_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_KEYUP_IRQ_EN_A {
        match self.bits {
            false => ADC0_KEYUP_IRQ_EN_A::DISABLE,
            true => ADC0_KEYUP_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_KEYUP_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_KEYUP_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `adc0_keyup_irq_en` writer - ADC0 Key Up Interrupt Enable"]
pub type ADC0_KEYUP_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_INTC_SPEC, ADC0_KEYUP_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC0_KEYUP_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_KEYUP_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_KEYUP_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 Data Interrupt Enable"]
    #[inline(always)]
    pub fn adc0_data_irq_en(&self) -> ADC0_DATA_IRQ_EN_R {
        ADC0_DATA_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 Key Down Interrupt Enable"]
    #[inline(always)]
    pub fn adc0_keydown_irq_en(&self) -> ADC0_KEYDOWN_IRQ_EN_R {
        ADC0_KEYDOWN_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Interrupt Enable"]
    #[inline(always)]
    pub fn adc0_hold_irq_en(&self) -> ADC0_HOLD_IRQ_EN_R {
        ADC0_HOLD_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Interrupt Enable"]
    #[inline(always)]
    pub fn adc0_alrdy_hold_irq_en(&self) -> ADC0_ALRDY_HOLD_IRQ_EN_R {
        ADC0_ALRDY_HOLD_IRQ_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC0 Key Up Interrupt Enable"]
    #[inline(always)]
    pub fn adc0_keyup_irq_en(&self) -> ADC0_KEYUP_IRQ_EN_R {
        ADC0_KEYUP_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_data_irq_en(&mut self) -> ADC0_DATA_IRQ_EN_W<0> {
        ADC0_DATA_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - ADC0 Key Down Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_keydown_irq_en(&mut self) -> ADC0_KEYDOWN_IRQ_EN_W<1> {
        ADC0_KEYDOWN_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_hold_irq_en(&mut self) -> ADC0_HOLD_IRQ_EN_W<2> {
        ADC0_HOLD_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_alrdy_hold_irq_en(&mut self) -> ADC0_ALRDY_HOLD_IRQ_EN_W<3> {
        ADC0_ALRDY_HOLD_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 4 - ADC0 Key Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_keyup_irq_en(&mut self) -> ADC0_KEYUP_IRQ_EN_W<4> {
        ADC0_KEYUP_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LRADC Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lradc_intc](index.html) module"]
pub struct LRADC_INTC_SPEC;
impl crate::RegisterSpec for LRADC_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lradc_intc::R](R) reader structure"]
impl crate::Readable for LRADC_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lradc_intc::W](W) writer structure"]
impl crate::Writable for LRADC_INTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lradc_intc to value 0"]
impl crate::Resettable for LRADC_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
