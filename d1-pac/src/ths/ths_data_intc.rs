#[doc = "Register `ths_data_intc` reader"]
pub type R = crate::R<THS_DATA_INTC_SPEC>;
#[doc = "Register `ths_data_intc` writer"]
pub type W = crate::W<THS_DATA_INTC_SPEC>;
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
    pub const fn variant(&self) -> THS_DATA_IRQ_EN_A {
        match self.bits {
            false => THS_DATA_IRQ_EN_A::DISABLE,
            true => THS_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THS_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THS_DATA_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `ths_data_irq_en` writer - Enable the interrupt of sensor_data update\n\nIf enabled, when the measured sensor_data is updated, it will generate an interrupt."]
pub type THS_DATA_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, THS_DATA_IRQ_EN_A>;
impl<'a, REG> THS_DATA_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(THS_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub fn ths_data_irq_en(&mut self) -> THS_DATA_IRQ_EN_W<THS_DATA_INTC_SPEC> {
        THS_DATA_IRQ_EN_W::new(self, 0)
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
#[doc = "THS Data Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_data_intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_data_intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_DATA_INTC_SPEC;
impl crate::RegisterSpec for THS_DATA_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_data_intc::R`](R) reader structure"]
impl crate::Readable for THS_DATA_INTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_data_intc::W`](W) writer structure"]
impl crate::Writable for THS_DATA_INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_data_intc to value 0"]
impl crate::Resettable for THS_DATA_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
