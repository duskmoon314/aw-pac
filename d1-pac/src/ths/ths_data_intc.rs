#[doc = "Register `ths_data_intc` reader"]
pub struct R(crate::R<THS_DATA_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_DATA_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_DATA_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_DATA_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ths_data_intc` writer"]
pub struct W(crate::W<THS_DATA_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_DATA_INTC_SPEC>;
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
impl From<crate::W<THS_DATA_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_DATA_INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ths_data_irq_en` reader - Enable the interrupt of sensor_data update\n\nIf enabled, when the measured sensor_data is updated, it will generate an interrupt."]
pub type THS_DATA_IRQ_EN_R = crate::BitReader<THS_DATA_IRQ_EN_A>;
#[doc = "Enable the interrupt of sensor_data update\n\nIf enabled, when the measured sensor_data is updated, it will generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THS_DATA_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<THS_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: THS_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl THS_DATA_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THS_DATA_IRQ_EN_A {
        match self.bits {
            false => THS_DATA_IRQ_EN_A::DISABLE,
            true => THS_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THS_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THS_DATA_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `ths_data_irq_en` writer - Enable the interrupt of sensor_data update\n\nIf enabled, when the measured sensor_data is updated, it will generate an interrupt."]
pub type THS_DATA_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, THS_DATA_INTC_SPEC, THS_DATA_IRQ_EN_A, O>;
impl<'a, const O: u8> THS_DATA_IRQ_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(THS_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(THS_DATA_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the interrupt of sensor_data update\n\nIf enabled, when the measured sensor_data is updated, it will generate an interrupt."]
    #[inline(always)]
    pub fn ths_data_irq_en(&self) -> THS_DATA_IRQ_EN_R {
        THS_DATA_IRQ_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt of sensor_data update\n\nIf enabled, when the measured sensor_data is updated, it will generate an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ths_data_irq_en(&mut self) -> THS_DATA_IRQ_EN_W<0> {
        THS_DATA_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Data Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_data_intc](index.html) module"]
pub struct THS_DATA_INTC_SPEC;
impl crate::RegisterSpec for THS_DATA_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_data_intc::R](R) reader structure"]
impl crate::Readable for THS_DATA_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_data_intc::W](W) writer structure"]
impl crate::Writable for THS_DATA_INTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_data_intc to value 0"]
impl crate::Resettable for THS_DATA_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
