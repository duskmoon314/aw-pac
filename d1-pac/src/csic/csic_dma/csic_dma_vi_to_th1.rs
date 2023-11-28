#[doc = "Register `csic_dma_vi_to_th1` reader"]
pub type R = crate::R<CSIC_DMA_VI_TO_TH1_SPEC>;
#[doc = "Register `csic_dma_vi_to_th1` writer"]
pub type W = crate::W<CSIC_DMA_VI_TO_TH1_SPEC>;
#[doc = "Field `vi_to_th1` reader - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
pub type VI_TO_TH1_R = crate::FieldReader<u32>;
#[doc = "Field `vi_to_th1` writer - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
pub type VI_TO_TH1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
    #[inline(always)]
    pub fn vi_to_th1(&self) -> VI_TO_TH1_R {
        VI_TO_TH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
    #[inline(always)]
    #[must_use]
    pub fn vi_to_th1(&mut self) -> VI_TO_TH1_W<CSIC_DMA_VI_TO_TH1_SPEC> {
        VI_TO_TH1_W::new(self, 0)
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
#[doc = "CSIC DMA Video Input Timeout Threshold1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vi_to_th1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vi_to_th1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_VI_TO_TH1_SPEC;
impl crate::RegisterSpec for CSIC_DMA_VI_TO_TH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_vi_to_th1::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_VI_TO_TH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_vi_to_th1::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_VI_TO_TH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_vi_to_th1 to value 0"]
impl crate::Resettable for CSIC_DMA_VI_TO_TH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
