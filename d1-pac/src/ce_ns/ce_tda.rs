#[doc = "Register `ce_tda` reader"]
pub type R = crate::R<CE_TDA_SPEC>;
#[doc = "Register `ce_tda` writer"]
pub type W = crate::W<CE_TDA_SPEC>;
#[doc = "Field `task` reader - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
pub type TASK_R = crate::FieldReader<u32>;
#[doc = "Field `task` writer - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
pub type TASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
    #[inline(always)]
    pub fn task(&self) -> TASK_R {
        TASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
    #[inline(always)]
    #[must_use]
    pub fn task(&mut self) -> TASK_W<CE_TDA_SPEC> {
        TASK_W::new(self, 0)
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
#[doc = "Task Descriptor Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_TDA_SPEC;
impl crate::RegisterSpec for CE_TDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_tda::R`](R) reader structure"]
impl crate::Readable for CE_TDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_tda::W`](W) writer structure"]
impl crate::Writable for CE_TDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_tda to value 0"]
impl crate::Resettable for CE_TDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
