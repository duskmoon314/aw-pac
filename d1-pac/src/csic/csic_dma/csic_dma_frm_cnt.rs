#[doc = "Register `csic_dma_frm_cnt` reader"]
pub type R = crate::R<CSIC_DMA_FRM_CNT_SPEC>;
#[doc = "Register `csic_dma_frm_cnt` writer"]
pub type W = crate::W<CSIC_DMA_FRM_CNT_SPEC>;
#[doc = "Field `frm_cnt` reader - Counter value of frame. When frame done comes, the internal counter value add 1, and when the reg full, it is cleared to 0 . When parser sent a sync signal, it is cleared to 0."]
pub type FRM_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `pclk_dma_clr_distance` reader - Frame cnt clear cycle\n\nN*T_{SYNC}"]
pub type PCLK_DMA_CLR_DISTANCE_R = crate::FieldReader<u16>;
#[doc = "Field `pclk_dma_clr_distance` writer - Frame cnt clear cycle\n\nN*T_{SYNC}"]
pub type PCLK_DMA_CLR_DISTANCE_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `frm_cnt_clr` reader - When the bit set to 1, Frame cnt is cleared to 0."]
pub type FRM_CNT_CLR_R = crate::BitReader;
#[doc = "Field `frm_cnt_clr` writer - When the bit set to 1, Frame cnt is cleared to 0."]
pub type FRM_CNT_CLR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Counter value of frame. When frame done comes, the internal counter value add 1, and when the reg full, it is cleared to 0 . When parser sent a sync signal, it is cleared to 0."]
    #[inline(always)]
    pub fn frm_cnt(&self) -> FRM_CNT_R {
        FRM_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - Frame cnt clear cycle\n\nN*T_{SYNC}"]
    #[inline(always)]
    pub fn pclk_dma_clr_distance(&self) -> PCLK_DMA_CLR_DISTANCE_R {
        PCLK_DMA_CLR_DISTANCE_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - When the bit set to 1, Frame cnt is cleared to 0."]
    #[inline(always)]
    pub fn frm_cnt_clr(&self) -> FRM_CNT_CLR_R {
        FRM_CNT_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:30 - Frame cnt clear cycle\n\nN*T_{SYNC}"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dma_clr_distance(&mut self) -> PCLK_DMA_CLR_DISTANCE_W<CSIC_DMA_FRM_CNT_SPEC> {
        PCLK_DMA_CLR_DISTANCE_W::new(self, 16)
    }
    #[doc = "Bit 31 - When the bit set to 1, Frame cnt is cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn frm_cnt_clr(&mut self) -> FRM_CNT_CLR_W<CSIC_DMA_FRM_CNT_SPEC> {
        FRM_CNT_CLR_W::new(self, 31)
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
#[doc = "CSIC DMA Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_frm_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_frm_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_FRM_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FRM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_frm_cnt::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_FRM_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_frm_cnt::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_FRM_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000_0000;
}
#[doc = "`reset()` method sets csic_dma_frm_cnt to value 0x0001_0000"]
impl crate::Resettable for CSIC_DMA_FRM_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
