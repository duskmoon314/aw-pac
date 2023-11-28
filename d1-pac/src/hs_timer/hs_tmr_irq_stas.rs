#[doc = "Register `hs_tmr_irq_stas` reader"]
pub type R = crate::R<HS_TMR_IRQ_STAS_SPEC>;
#[doc = "Register `hs_tmr_irq_stas` writer"]
pub type W = crate::W<HS_TMR_IRQ_STAS_SPEC>;
#[doc = "Field `hs_tmr_irq_pend[0-1]` reader - HSTimer IRQ Pending"]
pub type HS_TMR_IRQ_PEND_R = crate::BitReader<HS_TMR_IRQ_PEND_A>;
#[doc = "HSTimer IRQ Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl HS_TMR_IRQ_PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HS_TMR_IRQ_PEND_A {
        match self.bits {
            false => HS_TMR_IRQ_PEND_A::NO_EFFECT,
            true => HS_TMR_IRQ_PEND_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HS_TMR_IRQ_PEND_A::NO_EFFECT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == HS_TMR_IRQ_PEND_A::PENDING
    }
}
#[doc = "Field `hs_tmr_irq_pend[0-1]` writer - HSTimer IRQ Pending"]
pub type HS_TMR_IRQ_PEND_W<'a, REG> = crate::BitWriter1C<'a, REG, HS_TMR_IRQ_PEND_A>;
impl<'a, REG> HS_TMR_IRQ_PEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HS_TMR_IRQ_PEND_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(HS_TMR_IRQ_PEND_A::PENDING)
    }
}
impl R {
    #[doc = "HSTimer IRQ Pending\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `hs_tmr0_irq_pend` field"]
    #[inline(always)]
    pub fn hs_tmr_irq_pend(&self, n: u8) -> HS_TMR_IRQ_PEND_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
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
    #[doc = "HSTimer IRQ Pending\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `hs_tmr0_irq_pend` field"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr_irq_pend(&mut self, n: u8) -> HS_TMR_IRQ_PEND_W<HS_TMR_IRQ_STAS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HS_TMR_IRQ_PEND_W::new(self, n)
    }
    #[doc = "Bit 0 - HSTimer IRQ Pending"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr0_irq_pend(&mut self) -> HS_TMR_IRQ_PEND_W<HS_TMR_IRQ_STAS_SPEC> {
        HS_TMR_IRQ_PEND_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSTimer IRQ Pending"]
    #[inline(always)]
    #[must_use]
    pub fn hs_tmr1_irq_pend(&mut self) -> HS_TMR_IRQ_PEND_W<HS_TMR_IRQ_STAS_SPEC> {
        HS_TMR_IRQ_PEND_W::new(self, 1)
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
#[doc = "HS Timer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_tmr_irq_stas::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_tmr_irq_stas::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_TMR_IRQ_STAS_SPEC;
impl crate::RegisterSpec for HS_TMR_IRQ_STAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_tmr_irq_stas::R`](R) reader structure"]
impl crate::Readable for HS_TMR_IRQ_STAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_tmr_irq_stas::W`](W) writer structure"]
impl crate::Writable for HS_TMR_IRQ_STAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets hs_tmr_irq_stas to value 0"]
impl crate::Resettable for HS_TMR_IRQ_STAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
