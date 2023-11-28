#[doc = "Register `wdog_irq_en` reader"]
pub type R = crate::R<WDOG_IRQ_EN_SPEC>;
#[doc = "Register `wdog_irq_en` writer"]
pub type W = crate::W<WDOG_IRQ_EN_SPEC>;
#[doc = "Field `wdog_irq_en` reader - "]
pub type WDOG_IRQ_EN_R = crate::BitReader<WDOG_IRQ_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WDOG_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDOG_IRQ_EN_A {
        match self.bits {
            false => WDOG_IRQ_EN_A::DISABLED,
            true => WDOG_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOG_IRQ_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDOG_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `wdog_irq_en` writer - "]
pub type WDOG_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, WDOG_IRQ_EN_A>;
impl<'a, REG> WDOG_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_IRQ_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdog_irq_en(&self) -> WDOG_IRQ_EN_R {
        WDOG_IRQ_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_irq_en(&mut self) -> WDOG_IRQ_EN_W<WDOG_IRQ_EN_SPEC> {
        WDOG_IRQ_EN_W::new(self, 0)
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
#[doc = "Watchdog IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_IRQ_EN_SPEC;
impl crate::RegisterSpec for WDOG_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_irq_en::R`](R) reader structure"]
impl crate::Readable for WDOG_IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog_irq_en::W`](W) writer structure"]
impl crate::Writable for WDOG_IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_irq_en to value 0"]
impl crate::Resettable for WDOG_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
