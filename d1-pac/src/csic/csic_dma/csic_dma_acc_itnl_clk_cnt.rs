#[doc = "Register `csic_dma_acc_itnl_clk_cnt` reader"]
pub type R = crate::R<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>;
#[doc = "Register `csic_dma_acc_itnl_clk_cnt` writer"]
pub type W = crate::W<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>;
#[doc = "Field `itnl_clk_cnt` reader - The instant value of internal frame clock counter.\n\nWhen frame done interrupt comes, the software can query this counter for judging whether it is the time for updating the double buffer address registers."]
pub type ITNL_CLK_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `acc_clk_cnt` reader - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
pub type ACC_CLK_CNT_R = crate::FieldReader;
#[doc = "Field `acc_clk_cnt` writer - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
pub type ACC_CLK_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - The instant value of internal frame clock counter.\n\nWhen frame done interrupt comes, the software can query this counter for judging whether it is the time for updating the double buffer address registers."]
    #[inline(always)]
    pub fn itnl_clk_cnt(&self) -> ITNL_CLK_CNT_R {
        ITNL_CLK_CNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn acc_clk_cnt(&self) -> ACC_CLK_CNT_R {
        ACC_CLK_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
    #[inline(always)]
    #[must_use]
    pub fn acc_clk_cnt(&mut self) -> ACC_CLK_CNT_W<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC> {
        ACC_CLK_CNT_W::new(self, 24)
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
#[doc = "CSIC DMA Accumulated And Internal Clock Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_acc_itnl_clk_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_acc_itnl_clk_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_acc_itnl_clk_cnt::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_acc_itnl_clk_cnt::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_acc_itnl_clk_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
