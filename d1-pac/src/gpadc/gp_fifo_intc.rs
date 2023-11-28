#[doc = "Register `gp_fifo_intc` reader"]
pub type R = crate::R<GP_FIFO_INTC_SPEC>;
#[doc = "Register `gp_fifo_intc` writer"]
pub type W = crate::W<GP_FIFO_INTC_SPEC>;
#[doc = "Field `fifo_flush` reader - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
pub type FIFO_FLUSH_R = crate::BitReader;
#[doc = "Field `fifo_flush` writer - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
pub type FIFO_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_trig_level` reader - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
pub type FIFO_TRIG_LEVEL_R = crate::FieldReader;
#[doc = "Field `fifo_trig_level` writer - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
pub type FIFO_TRIG_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `fifo_data_irq_en` reader - ADC FIFO Data Available IRQ Enable"]
pub type FIFO_DATA_IRQ_EN_R = crate::BitReader<FIFO_DATA_IRQ_EN_A>;
#[doc = "ADC FIFO Data Available IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_DATA_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FIFO_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_DATA_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_DATA_IRQ_EN_A {
        match self.bits {
            false => FIFO_DATA_IRQ_EN_A::DISABLE,
            true => FIFO_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_DATA_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_data_irq_en` writer - ADC FIFO Data Available IRQ Enable"]
pub type FIFO_DATA_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_DATA_IRQ_EN_A>;
impl<'a, REG> FIFO_DATA_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_DATA_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `fifo_overrun_irq_en` reader - ADC FIFO Overrun IRQ Enable"]
pub type FIFO_OVERRUN_IRQ_EN_R = crate::BitReader<FIFO_OVERRUN_IRQ_EN_A>;
#[doc = "ADC FIFO Overrun IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_OVERRUN_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FIFO_OVERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_OVERRUN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_OVERRUN_IRQ_EN_A {
        match self.bits {
            false => FIFO_OVERRUN_IRQ_EN_A::DISABLE,
            true => FIFO_OVERRUN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_OVERRUN_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_OVERRUN_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_overrun_irq_en` writer - ADC FIFO Overrun IRQ Enable"]
pub type FIFO_OVERRUN_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_OVERRUN_IRQ_EN_A>;
impl<'a, REG> FIFO_OVERRUN_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERRUN_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERRUN_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `fifo_data_drq_en` reader - ADC FIFO Date DRQ Enable"]
pub type FIFO_DATA_DRQ_EN_R = crate::BitReader<FIFO_DATA_DRQ_EN_A>;
#[doc = "ADC FIFO Date DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_DATA_DRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FIFO_DATA_DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_DATA_DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_DATA_DRQ_EN_A {
        match self.bits {
            false => FIFO_DATA_DRQ_EN_A::DISABLE,
            true => FIFO_DATA_DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_DATA_DRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_DATA_DRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_data_drq_en` writer - ADC FIFO Date DRQ Enable"]
pub type FIFO_DATA_DRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_DATA_DRQ_EN_A>;
impl<'a, REG> FIFO_DATA_DRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_DATA_DRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_DATA_DRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 4 - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
    #[inline(always)]
    pub fn fifo_flush(&self) -> FIFO_FLUSH_R {
        FIFO_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
    #[inline(always)]
    pub fn fifo_trig_level(&self) -> FIFO_TRIG_LEVEL_R {
        FIFO_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - ADC FIFO Data Available IRQ Enable"]
    #[inline(always)]
    pub fn fifo_data_irq_en(&self) -> FIFO_DATA_IRQ_EN_R {
        FIFO_DATA_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    pub fn fifo_overrun_irq_en(&self) -> FIFO_OVERRUN_IRQ_EN_R {
        FIFO_OVERRUN_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC FIFO Date DRQ Enable"]
    #[inline(always)]
    pub fn fifo_data_drq_en(&self) -> FIFO_DATA_DRQ_EN_R {
        FIFO_DATA_DRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_flush(&mut self) -> FIFO_FLUSH_W<GP_FIFO_INTC_SPEC> {
        FIFO_FLUSH_W::new(self, 4)
    }
    #[doc = "Bits 8:13 - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_trig_level(&mut self) -> FIFO_TRIG_LEVEL_W<GP_FIFO_INTC_SPEC> {
        FIFO_TRIG_LEVEL_W::new(self, 8)
    }
    #[doc = "Bit 16 - ADC FIFO Data Available IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data_irq_en(&mut self) -> FIFO_DATA_IRQ_EN_W<GP_FIFO_INTC_SPEC> {
        FIFO_DATA_IRQ_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_irq_en(&mut self) -> FIFO_OVERRUN_IRQ_EN_W<GP_FIFO_INTC_SPEC> {
        FIFO_OVERRUN_IRQ_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC FIFO Date DRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data_drq_en(&mut self) -> FIFO_DATA_DRQ_EN_W<GP_FIFO_INTC_SPEC> {
        FIFO_DATA_DRQ_EN_W::new(self, 18)
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
#[doc = "GPADC FIFO Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_fifo_intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_fifo_intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_FIFO_INTC_SPEC;
impl crate::RegisterSpec for GP_FIFO_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_fifo_intc::R`](R) reader structure"]
impl crate::Readable for GP_FIFO_INTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_fifo_intc::W`](W) writer structure"]
impl crate::Writable for GP_FIFO_INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_fifo_intc to value 0x1f00"]
impl crate::Resettable for GP_FIFO_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f00;
}
