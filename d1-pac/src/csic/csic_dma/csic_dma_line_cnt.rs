#[doc = "Register `csic_dma_line_cnt` reader"]
pub type R = crate::R<CSIC_DMA_LINE_CNT_SPEC>;
#[doc = "Register `csic_dma_line_cnt` writer"]
pub type W = crate::W<CSIC_DMA_LINE_CNT_SPEC>;
#[doc = "Field `line_cnt_num` reader - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
pub type LINE_CNT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `line_cnt_num` writer - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
pub type LINE_CNT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
    #[inline(always)]
    pub fn line_cnt_num(&self) -> LINE_CNT_NUM_R {
        LINE_CNT_NUM_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
    #[inline(always)]
    #[must_use]
    pub fn line_cnt_num(&mut self) -> LINE_CNT_NUM_W<CSIC_DMA_LINE_CNT_SPEC> {
        LINE_CNT_NUM_W::new(self, 0)
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
#[doc = "CSIC DMA LINE Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_line_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_line_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_LINE_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_LINE_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_line_cnt::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_LINE_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_line_cnt::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_LINE_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_line_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_LINE_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
