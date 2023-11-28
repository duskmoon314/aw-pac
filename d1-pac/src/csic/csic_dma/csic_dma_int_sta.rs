#[doc = "Register `csic_dma_int_sta` reader"]
pub type R = crate::R<CSIC_DMA_INT_STA_SPEC>;
#[doc = "Register `csic_dma_int_sta` writer"]
pub type W = crate::W<CSIC_DMA_INT_STA_SPEC>;
#[doc = "Field `cd_pd` reader - Capture done"]
pub type CD_PD_R = crate::BitReader;
#[doc = "Field `cd_pd` writer - Capture done"]
pub type CD_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `fd_pd` reader - Frame done"]
pub type FD_PD_R = crate::BitReader;
#[doc = "Field `fd_pd` writer - Frame done"]
pub type FD_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `fifo0_of_pd` reader - FIFO 0 overflow"]
pub type FIFO0_OF_PD_R = crate::BitReader;
#[doc = "Field `fifo0_of_pd` writer - FIFO 0 overflow"]
pub type FIFO0_OF_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `fifo1_of_pd` reader - FIFO 1 overflow"]
pub type FIFO1_OF_PD_R = crate::BitReader;
#[doc = "Field `fifo1_of_pd` writer - FIFO 1 overflow"]
pub type FIFO1_OF_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `fifo2_of_pd` reader - FIFO 2 overflow"]
pub type FIFO2_OF_PD_R = crate::BitReader;
#[doc = "Field `fifo2_of_pd` writer - FIFO 2 overflow"]
pub type FIFO2_OF_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `lc_pd` reader - Line counter flag"]
pub type LC_PD_R = crate::BitReader;
#[doc = "Field `lc_pd` writer - Line counter flag"]
pub type LC_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `li_of_pd` reader - Line information FIFO (16 lines) overflow"]
pub type LI_OF_PD_R = crate::BitReader;
#[doc = "Field `li_of_pd` writer - Line information FIFO (16 lines) overflow"]
pub type LI_OF_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `vs_pd` reader - vsync flag"]
pub type VS_PD_R = crate::BitReader;
#[doc = "Field `vs_pd` writer - vsync flag"]
pub type VS_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `clr_frame_cnt_int` reader - Set a INT when clear Frame cnt."]
pub type CLR_FRAME_CNT_INT_R = crate::BitReader;
#[doc = "Field `clr_frame_cnt_int` writer - Set a INT when clear Frame cnt."]
pub type CLR_FRAME_CNT_INT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `video_input_to_int_pd` reader - Set an INT Pending when no video input exceeds the setting threshold time."]
pub type VIDEO_INPUT_TO_INT_PD_R = crate::BitReader;
#[doc = "Field `video_input_to_int_pd` writer - Set an INT Pending when no video input exceeds the setting threshold time."]
pub type VIDEO_INPUT_TO_INT_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `buf_addr_fifo_int_pd` reader - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type BUF_ADDR_FIFO_INT_PD_R = crate::BitReader;
#[doc = "Field `buf_addr_fifo_int_pd` writer - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type BUF_ADDR_FIFO_INT_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `stored_frm_cnt_int_pd` reader - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type STORED_FRM_CNT_INT_PD_R = crate::BitReader;
#[doc = "Field `stored_frm_cnt_int_pd` writer - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type STORED_FRM_CNT_INT_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `frm_lost_int_pd` reader - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
pub type FRM_LOST_INT_PD_R = crate::BitReader;
#[doc = "Field `frm_lost_int_pd` writer - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
pub type FRM_LOST_INT_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture done"]
    #[inline(always)]
    pub fn cd_pd(&self) -> CD_PD_R {
        CD_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame done"]
    #[inline(always)]
    pub fn fd_pd(&self) -> FD_PD_R {
        FD_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO 0 overflow"]
    #[inline(always)]
    pub fn fifo0_of_pd(&self) -> FIFO0_OF_PD_R {
        FIFO0_OF_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO 1 overflow"]
    #[inline(always)]
    pub fn fifo1_of_pd(&self) -> FIFO1_OF_PD_R {
        FIFO1_OF_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 2 overflow"]
    #[inline(always)]
    pub fn fifo2_of_pd(&self) -> FIFO2_OF_PD_R {
        FIFO2_OF_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line counter flag"]
    #[inline(always)]
    pub fn lc_pd(&self) -> LC_PD_R {
        LC_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line information FIFO (16 lines) overflow"]
    #[inline(always)]
    pub fn li_of_pd(&self) -> LI_OF_PD_R {
        LI_OF_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vsync flag"]
    #[inline(always)]
    pub fn vs_pd(&self) -> VS_PD_R {
        VS_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Set a INT when clear Frame cnt."]
    #[inline(always)]
    pub fn clr_frame_cnt_int(&self) -> CLR_FRAME_CNT_INT_R {
        CLR_FRAME_CNT_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set an INT Pending when no video input exceeds the setting threshold time."]
    #[inline(always)]
    pub fn video_input_to_int_pd(&self) -> VIDEO_INPUT_TO_INT_PD_R {
        VIDEO_INPUT_TO_INT_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    pub fn buf_addr_fifo_int_pd(&self) -> BUF_ADDR_FIFO_INT_PD_R {
        BUF_ADDR_FIFO_INT_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    pub fn stored_frm_cnt_int_pd(&self) -> STORED_FRM_CNT_INT_PD_R {
        STORED_FRM_CNT_INT_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    pub fn frm_lost_int_pd(&self) -> FRM_LOST_INT_PD_R {
        FRM_LOST_INT_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture done"]
    #[inline(always)]
    #[must_use]
    pub fn cd_pd(&mut self) -> CD_PD_W<CSIC_DMA_INT_STA_SPEC> {
        CD_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Frame done"]
    #[inline(always)]
    #[must_use]
    pub fn fd_pd(&mut self) -> FD_PD_W<CSIC_DMA_INT_STA_SPEC> {
        FD_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO 0 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo0_of_pd(&mut self) -> FIFO0_OF_PD_W<CSIC_DMA_INT_STA_SPEC> {
        FIFO0_OF_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO 1 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo1_of_pd(&mut self) -> FIFO1_OF_PD_W<CSIC_DMA_INT_STA_SPEC> {
        FIFO1_OF_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - FIFO 2 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn fifo2_of_pd(&mut self) -> FIFO2_OF_PD_W<CSIC_DMA_INT_STA_SPEC> {
        FIFO2_OF_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Line counter flag"]
    #[inline(always)]
    #[must_use]
    pub fn lc_pd(&mut self) -> LC_PD_W<CSIC_DMA_INT_STA_SPEC> {
        LC_PD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Line information FIFO (16 lines) overflow"]
    #[inline(always)]
    #[must_use]
    pub fn li_of_pd(&mut self) -> LI_OF_PD_W<CSIC_DMA_INT_STA_SPEC> {
        LI_OF_PD_W::new(self, 6)
    }
    #[doc = "Bit 7 - vsync flag"]
    #[inline(always)]
    #[must_use]
    pub fn vs_pd(&mut self) -> VS_PD_W<CSIC_DMA_INT_STA_SPEC> {
        VS_PD_W::new(self, 7)
    }
    #[doc = "Bit 11 - Set a INT when clear Frame cnt."]
    #[inline(always)]
    #[must_use]
    pub fn clr_frame_cnt_int(&mut self) -> CLR_FRAME_CNT_INT_W<CSIC_DMA_INT_STA_SPEC> {
        CLR_FRAME_CNT_INT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set an INT Pending when no video input exceeds the setting threshold time."]
    #[inline(always)]
    #[must_use]
    pub fn video_input_to_int_pd(&mut self) -> VIDEO_INPUT_TO_INT_PD_W<CSIC_DMA_INT_STA_SPEC> {
        VIDEO_INPUT_TO_INT_PD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn buf_addr_fifo_int_pd(&mut self) -> BUF_ADDR_FIFO_INT_PD_W<CSIC_DMA_INT_STA_SPEC> {
        BUF_ADDR_FIFO_INT_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn stored_frm_cnt_int_pd(&mut self) -> STORED_FRM_CNT_INT_PD_W<CSIC_DMA_INT_STA_SPEC> {
        STORED_FRM_CNT_INT_PD_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn frm_lost_int_pd(&mut self) -> FRM_LOST_INT_PD_W<CSIC_DMA_INT_STA_SPEC> {
        FRM_LOST_INT_PD_W::new(self, 15)
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
#[doc = "CSIC DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_int_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_int_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_INT_STA_SPEC;
impl crate::RegisterSpec for CSIC_DMA_INT_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_int_sta::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_INT_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_int_sta::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_INT_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf8ff;
}
#[doc = "`reset()` method sets csic_dma_int_sta to value 0"]
impl crate::Resettable for CSIC_DMA_INT_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
