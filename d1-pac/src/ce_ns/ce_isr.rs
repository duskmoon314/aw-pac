#[doc = "Register `ce_isr` reader"]
pub struct R(crate::R<CE_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ce_isr` writer"]
pub struct W(crate::W<CE_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_ISR_SPEC>;
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
impl From<crate::W<CE_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `task_pending[0-3]` reader - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
pub type TASK_PENDING_R = crate::FieldReader<u8, TASK_PENDING_A>;
#[doc = "Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TASK_PENDING_A {
    #[doc = "0: Task not finished"]
    NOT_FINISHED = 0,
    #[doc = "1: Task finished"]
    FINISHED = 1,
}
impl From<TASK_PENDING_A> for u8 {
    #[inline(always)]
    fn from(variant: TASK_PENDING_A) -> Self {
        variant as _
    }
}
impl TASK_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TASK_PENDING_A> {
        match self.bits {
            0 => Some(TASK_PENDING_A::NOT_FINISHED),
            1 => Some(TASK_PENDING_A::FINISHED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FINISHED`"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == TASK_PENDING_A::NOT_FINISHED
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == TASK_PENDING_A::FINISHED
    }
}
#[doc = "Field `task_pending[0-3]` writer - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
pub type TASK_PENDING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CE_ISR_SPEC, u8, TASK_PENDING_A, 4, O>;
impl<'a, const O: u8> TASK_PENDING_W<'a, O> {
    #[doc = "Task not finished"]
    #[inline(always)]
    pub fn not_finished(self) -> &'a mut W {
        self.variant(TASK_PENDING_A::NOT_FINISHED)
    }
    #[doc = "Task finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(TASK_PENDING_A::FINISHED)
    }
}
impl R {
    #[doc = "Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    pub unsafe fn task_pending(&self, n: u8) -> TASK_PENDING_R {
        TASK_PENDING_R::new(((self.bits >> n) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    pub fn task0_pending(&self) -> TASK_PENDING_R {
        TASK_PENDING_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    pub fn task1_pending(&self) -> TASK_PENDING_R {
        TASK_PENDING_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 2:5 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    pub fn task2_pending(&self) -> TASK_PENDING_R {
        TASK_PENDING_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 3:6 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    pub fn task3_pending(&self) -> TASK_PENDING_R {
        TASK_PENDING_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn task_pending<const O: u8>(&mut self) -> TASK_PENDING_W<O> {
        TASK_PENDING_W::new(self)
    }
    #[doc = "Bits 0:3 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task0_pending(&mut self) -> TASK_PENDING_W<0> {
        TASK_PENDING_W::new(self)
    }
    #[doc = "Bits 1:4 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task1_pending(&mut self) -> TASK_PENDING_W<1> {
        TASK_PENDING_W::new(self)
    }
    #[doc = "Bits 2:5 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task2_pending(&mut self) -> TASK_PENDING_W<2> {
        TASK_PENDING_W::new(self)
    }
    #[doc = "Bits 3:6 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task3_pending(&mut self) -> TASK_PENDING_W<3> {
        TASK_PENDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_isr](index.html) module"]
pub struct CE_ISR_SPEC;
impl crate::RegisterSpec for CE_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_isr::R](R) reader structure"]
impl crate::Readable for CE_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_isr::W](W) writer structure"]
impl crate::Writable for CE_ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets ce_isr to value 0"]
impl crate::Resettable for CE_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
