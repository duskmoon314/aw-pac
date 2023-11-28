#[doc = "Register `gp_datah_intc` reader"]
pub type R = crate::R<GP_DATAH_INTC_SPEC>;
#[doc = "Register `gp_datah_intc` writer"]
pub type W = crate::W<GP_DATAH_INTC_SPEC>;
#[doc = "Field `ch_hig_irq_en[0-1]` reader - Channel Voltage High Available Interrupt Enable"]
pub type CH_HIG_IRQ_EN_R = crate::BitReader<CH_HIG_IRQ_EN_A>;
#[doc = "Channel Voltage High Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_HIG_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CH_HIG_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CH_HIG_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_HIG_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH_HIG_IRQ_EN_A {
        match self.bits {
            false => CH_HIG_IRQ_EN_A::DISABLE,
            true => CH_HIG_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH_HIG_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CH_HIG_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `ch_hig_irq_en[0-1]` writer - Channel Voltage High Available Interrupt Enable"]
pub type CH_HIG_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, CH_HIG_IRQ_EN_A>;
impl<'a, REG> CH_HIG_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH_HIG_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CH_HIG_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Channel Voltage High Available Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_hig_irq_en` field"]
    #[inline(always)]
    pub fn ch_hig_irq_en(&self, n: u8) -> CH_HIG_IRQ_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_HIG_IRQ_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel Voltage High Available Interrupt Enable"]
    #[inline(always)]
    pub fn ch0_hig_irq_en(&self) -> CH_HIG_IRQ_EN_R {
        CH_HIG_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Voltage High Available Interrupt Enable"]
    #[inline(always)]
    pub fn ch1_hig_irq_en(&self) -> CH_HIG_IRQ_EN_R {
        CH_HIG_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Channel Voltage High Available Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_hig_irq_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hig_irq_en(&mut self, n: u8) -> CH_HIG_IRQ_EN_W<GP_DATAH_INTC_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_HIG_IRQ_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - Channel Voltage High Available Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_hig_irq_en(&mut self) -> CH_HIG_IRQ_EN_W<GP_DATAH_INTC_SPEC> {
        CH_HIG_IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Voltage High Available Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_hig_irq_en(&mut self) -> CH_HIG_IRQ_EN_W<GP_DATAH_INTC_SPEC> {
        CH_HIG_IRQ_EN_W::new(self, 1)
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
#[doc = "GPADC Data High Interrupt Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_datah_intc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_datah_intc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_DATAH_INTC_SPEC;
impl crate::RegisterSpec for GP_DATAH_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_datah_intc::R`](R) reader structure"]
impl crate::Readable for GP_DATAH_INTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_datah_intc::W`](W) writer structure"]
impl crate::Writable for GP_DATAH_INTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_datah_intc to value 0"]
impl crate::Resettable for GP_DATAH_INTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
