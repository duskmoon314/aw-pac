#[doc = "Register `ce_isr` reader"]
pub type R = crate::R<CE_ISR_SPEC>;
#[doc = "Register `ce_isr` writer"]
pub type W = crate::W<CE_ISR_SPEC>;
#[doc = "Field `task_pending[0-3]` reader - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
pub type TASK_PENDING_R = crate::FieldReader<TASK_PENDING_A>;
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
impl crate::FieldSpec for TASK_PENDING_A {
    type Ux = u8;
}
impl TASK_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TASK_PENDING_A> {
        match self.bits {
            0 => Some(TASK_PENDING_A::NOT_FINISHED),
            1 => Some(TASK_PENDING_A::FINISHED),
            _ => None,
        }
    }
    #[doc = "Task not finished"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == TASK_PENDING_A::NOT_FINISHED
    }
    #[doc = "Task finished"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == TASK_PENDING_A::FINISHED
    }
}
#[doc = "Field `task_pending[0-3]` writer - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
pub type TASK_PENDING_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TASK_PENDING_A>;
impl<'a, REG> TASK_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Task not finished"]
    #[inline(always)]
    pub fn not_finished(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_PENDING_A::NOT_FINISHED)
    }
    #[doc = "Task finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_PENDING_A::FINISHED)
    }
}
impl R {
    #[doc = "Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `task0_pending` field"]
    #[inline(always)]
    pub fn task_pending(&self, n: u8) -> TASK_PENDING_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
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
    #[doc = "Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `task0_pending` field"]
    #[inline(always)]
    #[must_use]
    pub fn task_pending(&mut self, n: u8) -> TASK_PENDING_W<CE_ISR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TASK_PENDING_W::new(self, n)
    }
    #[doc = "Bits 0:3 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task0_pending(&mut self) -> TASK_PENDING_W<CE_ISR_SPEC> {
        TASK_PENDING_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task1_pending(&mut self) -> TASK_PENDING_W<CE_ISR_SPEC> {
        TASK_PENDING_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task2_pending(&mut self) -> TASK_PENDING_W<CE_ISR_SPEC> {
        TASK_PENDING_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - Task Channel 3-0 End Pending\n\nIt indicates whether task is completed.\n\nWrite the corresponding channel bit of the register to clear the end flag."]
    #[inline(always)]
    #[must_use]
    pub fn task3_pending(&mut self) -> TASK_PENDING_W<CE_ISR_SPEC> {
        TASK_PENDING_W::new(self, 3)
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
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_ISR_SPEC;
impl crate::RegisterSpec for CE_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_isr::R`](R) reader structure"]
impl crate::Readable for CE_ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_isr::W`](W) writer structure"]
impl crate::Writable for CE_ISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets ce_isr to value 0"]
impl crate::Resettable for CE_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
