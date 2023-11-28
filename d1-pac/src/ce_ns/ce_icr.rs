#[doc = "Register `ce_icr` reader"]
pub type R = crate::R<CE_ICR_SPEC>;
#[doc = "Register `ce_icr` writer"]
pub type W = crate::W<CE_ICR_SPEC>;
#[doc = "Field `task_irq_en[0-3]` reader - Task Channel 3-0 Interrupt Enable"]
pub type TASK_IRQ_EN_R = crate::FieldReader<TASK_IRQ_EN_A>;
#[doc = "Task Channel 3-0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TASK_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TASK_IRQ_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: TASK_IRQ_EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TASK_IRQ_EN_A {
    type Ux = u8;
}
impl TASK_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TASK_IRQ_EN_A> {
        match self.bits {
            0 => Some(TASK_IRQ_EN_A::DISABLE),
            1 => Some(TASK_IRQ_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TASK_IRQ_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TASK_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `task_irq_en[0-3]` writer - Task Channel 3-0 Interrupt Enable"]
pub type TASK_IRQ_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TASK_IRQ_EN_A>;
impl<'a, REG> TASK_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Task Channel 3-0 Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `task0_irq_en` field"]
    #[inline(always)]
    pub fn task_irq_en(&self, n: u8) -> TASK_IRQ_EN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TASK_IRQ_EN_R::new(((self.bits >> n) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    pub fn task0_irq_en(&self) -> TASK_IRQ_EN_R {
        TASK_IRQ_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    pub fn task1_irq_en(&self) -> TASK_IRQ_EN_R {
        TASK_IRQ_EN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 2:5 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    pub fn task2_irq_en(&self) -> TASK_IRQ_EN_R {
        TASK_IRQ_EN_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 3:6 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    pub fn task3_irq_en(&self) -> TASK_IRQ_EN_R {
        TASK_IRQ_EN_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Task Channel 3-0 Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `task0_irq_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn task_irq_en(&mut self, n: u8) -> TASK_IRQ_EN_W<CE_ICR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TASK_IRQ_EN_W::new(self, n)
    }
    #[doc = "Bits 0:3 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task0_irq_en(&mut self) -> TASK_IRQ_EN_W<CE_ICR_SPEC> {
        TASK_IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task1_irq_en(&mut self) -> TASK_IRQ_EN_W<CE_ICR_SPEC> {
        TASK_IRQ_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task2_irq_en(&mut self) -> TASK_IRQ_EN_W<CE_ICR_SPEC> {
        TASK_IRQ_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task3_irq_en(&mut self) -> TASK_IRQ_EN_W<CE_ICR_SPEC> {
        TASK_IRQ_EN_W::new(self, 3)
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
#[doc = "Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_ICR_SPEC;
impl crate::RegisterSpec for CE_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_icr::R`](R) reader structure"]
impl crate::Readable for CE_ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_icr::W`](W) writer structure"]
impl crate::Writable for CE_ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_icr to value 0"]
impl crate::Resettable for CE_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
