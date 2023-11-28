#[doc = "Register `hs_tmr_irq_en` reader"]
pub type R = crate::R<HS_TMR_IRQ_EN_SPEC>;
#[doc = "Register `hs_tmr_irq_en` writer"]
pub type W = crate::W<HS_TMR_IRQ_EN_SPEC>;
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
    pub const fn variant(&self) -> HS_TMR_INT_EN_A {
        match self.bits {
            false => HS_TMR_INT_EN_A::DISABLED,
            true => HS_TMR_INT_EN_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HS_TMR_INT_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HS_TMR_INT_EN_A::ENABLED
    }
}
#[doc = "Field `hs_tmr_int_en[0-1]` writer - HSTimer Interrupt Enable"]
pub type HS_TMR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, HS_TMR_INT_EN_A>;
impl<'a, REG> HS_TMR_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HS_TMR_INT_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HS_TMR_INT_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "HSTimer Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `hs_tmr0_int_en` field"]
    #[inline(always)]
    pub fn hs_tmr_int_en(&self, n: u8) -> HS_TMR_INT_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
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
    #[doc = "HSTimer Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `hs_tmr0_int_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_int_en(&mut self, n: u8) -> HS_TMR_INT_EN_W<HS_TMR_IRQ_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HS_TMR_INT_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - HSTimer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr0_int_en(&mut self) -> HS_TMR_INT_EN_W<HS_TMR_IRQ_EN_SPEC> {
        HS_TMR_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSTimer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr1_int_en(&mut self) -> HS_TMR_INT_EN_W<HS_TMR_IRQ_EN_SPEC> {
        HS_TMR_INT_EN_W::new(self, 1)
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
#[doc = "HS Timer IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_TMR_IRQ_EN_SPEC;
impl crate::RegisterSpec for HS_TMR_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_tmr_irq_en::R`](R) reader structure"]
impl crate::Readable for HS_TMR_IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_tmr_irq_en::W`](W) writer structure"]
impl crate::Writable for HS_TMR_IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hs_tmr_irq_en to value 0"]
impl crate::Resettable for HS_TMR_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
