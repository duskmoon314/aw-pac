#[doc = "Register `alarm0_irq_en` reader"]
pub type R = crate::R<ALARM0_IRQ_EN_SPEC>;
#[doc = "Register `alarm0_irq_en` writer"]
pub type W = crate::W<ALARM0_IRQ_EN_SPEC>;
#[doc = "Field `alarm0_irq_en` reader - Alarm 0 IRQ Enable"]
pub type ALARM0_IRQ_EN_R = crate::BitReader<ALARM0_IRQ_EN_A>;
#[doc = "Alarm 0 IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM0_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ALARM0_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM0_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM0_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALARM0_IRQ_EN_A {
        match self.bits {
            false => ALARM0_IRQ_EN_A::DISABLE,
            true => ALARM0_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALARM0_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALARM0_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `alarm0_irq_en` writer - Alarm 0 IRQ Enable"]
pub type ALARM0_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, ALARM0_IRQ_EN_A>;
impl<'a, REG> ALARM0_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM0_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALARM0_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm 0 IRQ Enable"]
    #[inline(always)]
    pub fn alarm0_irq_en(&self) -> ALARM0_IRQ_EN_R {
        ALARM0_IRQ_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 0 IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0_irq_en(&mut self) -> ALARM0_IRQ_EN_W<ALARM0_IRQ_EN_SPEC> {
        ALARM0_IRQ_EN_W::new(self, 0)
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
#[doc = "Alarm 0 IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_IRQ_EN_SPEC;
impl crate::RegisterSpec for ALARM0_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_irq_en::R`](R) reader structure"]
impl crate::Readable for ALARM0_IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0_irq_en::W`](W) writer structure"]
impl crate::Writable for ALARM0_IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm0_irq_en to value 0"]
impl crate::Resettable for ALARM0_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
