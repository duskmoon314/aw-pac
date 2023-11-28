#[doc = "Register `ce_tlr` reader"]
pub type R = crate::R<CE_TLR_SPEC>;
#[doc = "Register `ce_tlr` writer"]
pub type W = crate::W<CE_TLR_SPEC>;
#[doc = "Field `task_load` reader - Task Load\n\nWhen set, the CE can load the descriptor of task if the task FIFO is not full."]
pub type TASK_LOAD_R = crate::BitReader;
#[doc = "Field `task_load` writer - Task Load\n\nWhen set, the CE can load the descriptor of task if the task FIFO is not full."]
pub type TASK_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Task Load\n\nWhen set, the CE can load the descriptor of task if the task FIFO is not full."]
    #[inline(always)]
    pub fn task_load(&self) -> TASK_LOAD_R {
        TASK_LOAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Task Load\n\nWhen set, the CE can load the descriptor of task if the task FIFO is not full."]
    #[inline(always)]
    #[must_use]
    pub fn task_load(&mut self) -> TASK_LOAD_W<CE_TLR_SPEC> {
        TASK_LOAD_W::new(self, 0)
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
#[doc = "Task Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_TLR_SPEC;
impl crate::RegisterSpec for CE_TLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_tlr::R`](R) reader structure"]
impl crate::Readable for CE_TLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_tlr::W`](W) writer structure"]
impl crate::Writable for CE_TLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_tlr to value 0"]
impl crate::Resettable for CE_TLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
