#[doc = "Register `ce_icr` reader"]
pub struct R(crate::R<CE_ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ce_icr` writer"]
pub struct W(crate::W<CE_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_ICR_SPEC>;
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
impl From<crate::W<CE_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `task_irq_en[0-3]` reader - Task Channel 3-0 Interrupt Enable"]
pub type TASK_IRQ_EN_R = crate::FieldReader<u8, TASK_IRQ_EN_A>;
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
impl TASK_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TASK_IRQ_EN_A> {
        match self.bits {
            0 => Some(TASK_IRQ_EN_A::DISABLE),
            1 => Some(TASK_IRQ_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TASK_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TASK_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `task_irq_en[0-3]` writer - Task Channel 3-0 Interrupt Enable"]
pub type TASK_IRQ_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CE_ICR_SPEC, u8, TASK_IRQ_EN_A, 4, O>;
impl<'a, const O: u8> TASK_IRQ_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TASK_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TASK_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn task_irq_en(&self, n: u8) -> TASK_IRQ_EN_R {
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
    #[doc = "Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn task_irq_en<const O: u8>(&mut self) -> TASK_IRQ_EN_W<O> {
        TASK_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 0:3 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task0_irq_en(&mut self) -> TASK_IRQ_EN_W<0> {
        TASK_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 1:4 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task1_irq_en(&mut self) -> TASK_IRQ_EN_W<1> {
        TASK_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 2:5 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task2_irq_en(&mut self) -> TASK_IRQ_EN_W<2> {
        TASK_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 3:6 - Task Channel 3-0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn task3_irq_en(&mut self) -> TASK_IRQ_EN_W<3> {
        TASK_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_icr](index.html) module"]
pub struct CE_ICR_SPEC;
impl crate::RegisterSpec for CE_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_icr::R](R) reader structure"]
impl crate::Readable for CE_ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_icr::W](W) writer structure"]
impl crate::Writable for CE_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_icr to value 0"]
impl crate::Resettable for CE_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
