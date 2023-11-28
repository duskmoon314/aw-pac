#[doc = "Register `ce_tpr` reader"]
pub type R = crate::R<CE_TPR_SPEC>;
#[doc = "Register `ce_tpr` writer"]
pub type W = crate::W<CE_TPR_SPEC>;
#[doc = "Field `tp_num` reader - It indicates the throughput writing to this register at last time.\n\nWriting to this register will clear it to 0."]
pub type TP_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `tp_num` writer - It indicates the throughput writing to this register at last time.\n\nWriting to this register will clear it to 0."]
pub type TP_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - It indicates the throughput writing to this register at last time.\n\nWriting to this register will clear it to 0."]
    #[inline(always)]
    pub fn tp_num(&self) -> TP_NUM_R {
        TP_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - It indicates the throughput writing to this register at last time.\n\nWriting to this register will clear it to 0."]
    #[inline(always)]
    #[must_use]
    pub fn tp_num(&mut self) -> TP_NUM_W<CE_TPR_SPEC> {
        TP_NUM_W::new(self, 0)
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
#[doc = "Throughput Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_tpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_tpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_TPR_SPEC;
impl crate::RegisterSpec for CE_TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_tpr::R`](R) reader structure"]
impl crate::Readable for CE_TPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_tpr::W`](W) writer structure"]
impl crate::Writable for CE_TPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_tpr to value 0"]
impl crate::Resettable for CE_TPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
