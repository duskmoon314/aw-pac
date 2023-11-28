#[doc = "Register `csic_dma_int_en` reader"]
pub type R = crate::R<CSIC_DMA_INT_EN_SPEC>;
#[doc = "Register `csic_dma_int_en` writer"]
pub type W = crate::W<CSIC_DMA_INT_EN_SPEC>;
#[doc = "Field `cd_int_en` reader - Capture done\n\nIndicates the CSI has completed capturing the image data. For still capture, the bit is set when one frame data has been written to buffer. For video capture, the bit is set when the last frame has been written to buffer after video capture has been disabled. For CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means field end."]
pub type CD_INT_EN_R = crate::BitReader;
#[doc = "Field `cd_int_en` writer - Capture done\n\nIndicates the CSI has completed capturing the image data. For still capture, the bit is set when one frame data has been written to buffer. For video capture, the bit is set when the last frame has been written to buffer after video capture has been disabled. For CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means field end."]
pub type CD_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fd_int_en` reader - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
pub type FD_INT_EN_R = crate::BitReader;
#[doc = "Field `fd_int_en` writer - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
pub type FD_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo0_of_int_en` reader - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
pub type FIFO0_OF_INT_EN_R = crate::BitReader;
#[doc = "Field `fifo0_of_int_en` writer - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
pub type FIFO0_OF_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo1_of_int_en` reader - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
pub type FIFO1_OF_INT_EN_R = crate::BitReader;
#[doc = "Field `fifo1_of_int_en` writer - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
pub type FIFO1_OF_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo2_of_int_en` reader - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
pub type FIFO2_OF_INT_EN_R = crate::BitReader;
#[doc = "Field `fifo2_of_int_en` writer - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
pub type FIFO2_OF_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lc_int_en` reader - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
pub type LC_INT_EN_R = crate::BitReader;
#[doc = "Field `lc_int_en` writer - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
pub type LC_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hb_of_int_en` reader - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
pub type HB_OF_INT_EN_R = crate::BitReader;
#[doc = "Field `hb_of_int_en` writer - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
pub type HB_OF_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vs_int_en` reader - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
pub type VS_INT_EN_R = crate::BitReader;
#[doc = "Field `vs_int_en` writer - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
pub type VS_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clr_frame_cnt_int_en` reader - Set a INT when clear Frame cnt."]
pub type CLR_FRAME_CNT_INT_EN_R = crate::BitReader;
#[doc = "Field `clr_frame_cnt_int_en` writer - Set a INT when clear Frame cnt."]
pub type CLR_FRAME_CNT_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `video_input_to_int_en` reader - Set an INT when no video input exceeds the setting threshold time"]
pub type VIDEO_INPUT_TO_INT_EN_R = crate::BitReader;
#[doc = "Field `video_input_to_int_en` writer - Set an INT when no video input exceeds the setting threshold time"]
pub type VIDEO_INPUT_TO_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `buf_addr_fifo_int_en` reader - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type BUF_ADDR_FIFO_INT_EN_R = crate::BitReader;
#[doc = "Field `buf_addr_fifo_int_en` writer - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type BUF_ADDR_FIFO_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stored_frm_cnt_int_en` reader - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type STORED_FRM_CNT_INT_EN_R = crate::BitReader;
#[doc = "Field `stored_frm_cnt_int_en` writer - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type STORED_FRM_CNT_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frm_lost_int_en` reader - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
pub type FRM_LOST_INT_EN_R = crate::BitReader;
#[doc = "Field `frm_lost_int_en` writer - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
pub type FRM_LOST_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture done\n\nIndicates the CSI has completed capturing the image data. For still capture, the bit is set when one frame data has been written to buffer. For video capture, the bit is set when the last frame has been written to buffer after video capture has been disabled. For CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means field end."]
    #[inline(always)]
    pub fn cd_int_en(&self) -> CD_INT_EN_R {
        CD_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
    #[inline(always)]
    pub fn fd_int_en(&self) -> FD_INT_EN_R {
        FD_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
    #[inline(always)]
    pub fn fifo0_of_int_en(&self) -> FIFO0_OF_INT_EN_R {
        FIFO0_OF_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
    #[inline(always)]
    pub fn fifo1_of_int_en(&self) -> FIFO1_OF_INT_EN_R {
        FIFO1_OF_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
    #[inline(always)]
    pub fn fifo2_of_int_en(&self) -> FIFO2_OF_INT_EN_R {
        FIFO2_OF_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
    #[inline(always)]
    pub fn lc_int_en(&self) -> LC_INT_EN_R {
        LC_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
    #[inline(always)]
    pub fn hb_of_int_en(&self) -> HB_OF_INT_EN_R {
        HB_OF_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
    #[inline(always)]
    pub fn vs_int_en(&self) -> VS_INT_EN_R {
        VS_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Set a INT when clear Frame cnt."]
    #[inline(always)]
    pub fn clr_frame_cnt_int_en(&self) -> CLR_FRAME_CNT_INT_EN_R {
        CLR_FRAME_CNT_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set an INT when no video input exceeds the setting threshold time"]
    #[inline(always)]
    pub fn video_input_to_int_en(&self) -> VIDEO_INPUT_TO_INT_EN_R {
        VIDEO_INPUT_TO_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    pub fn buf_addr_fifo_int_en(&self) -> BUF_ADDR_FIFO_INT_EN_R {
        BUF_ADDR_FIFO_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    pub fn stored_frm_cnt_int_en(&self) -> STORED_FRM_CNT_INT_EN_R {
        STORED_FRM_CNT_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    pub fn frm_lost_int_en(&self) -> FRM_LOST_INT_EN_R {
        FRM_LOST_INT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture done\n\nIndicates the CSI has completed capturing the image data. For still capture, the bit is set when one frame data has been written to buffer. For video capture, the bit is set when the last frame has been written to buffer after video capture has been disabled. For CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means field end."]
    #[inline(always)]
    #[must_use]
    pub fn cd_int_en(&mut self) -> CD_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        CD_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
    #[inline(always)]
    #[must_use]
    pub fn fd_int_en(&mut self) -> FD_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        FD_INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
    #[inline(always)]
    #[must_use]
    pub fn fifo0_of_int_en(&mut self) -> FIFO0_OF_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        FIFO0_OF_INT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
    #[inline(always)]
    #[must_use]
    pub fn fifo1_of_int_en(&mut self) -> FIFO1_OF_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        FIFO1_OF_INT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
    #[inline(always)]
    #[must_use]
    pub fn fifo2_of_int_en(&mut self) -> FIFO2_OF_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        FIFO2_OF_INT_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
    #[inline(always)]
    #[must_use]
    pub fn lc_int_en(&mut self) -> LC_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        LC_INT_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
    #[inline(always)]
    #[must_use]
    pub fn hb_of_int_en(&mut self) -> HB_OF_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        HB_OF_INT_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
    #[inline(always)]
    #[must_use]
    pub fn vs_int_en(&mut self) -> VS_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        VS_INT_EN_W::new(self, 7)
    }
    #[doc = "Bit 11 - Set a INT when clear Frame cnt."]
    #[inline(always)]
    #[must_use]
    pub fn clr_frame_cnt_int_en(&mut self) -> CLR_FRAME_CNT_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        CLR_FRAME_CNT_INT_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set an INT when no video input exceeds the setting threshold time"]
    #[inline(always)]
    #[must_use]
    pub fn video_input_to_int_en(&mut self) -> VIDEO_INPUT_TO_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        VIDEO_INPUT_TO_INT_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn buf_addr_fifo_int_en(&mut self) -> BUF_ADDR_FIFO_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        BUF_ADDR_FIFO_INT_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn stored_frm_cnt_int_en(&mut self) -> STORED_FRM_CNT_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        STORED_FRM_CNT_INT_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn frm_lost_int_en(&mut self) -> FRM_LOST_INT_EN_W<CSIC_DMA_INT_EN_SPEC> {
        FRM_LOST_INT_EN_W::new(self, 15)
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
#[doc = "CSIC DMA Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_INT_EN_SPEC;
impl crate::RegisterSpec for CSIC_DMA_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_int_en::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_int_en::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_int_en to value 0"]
impl crate::Resettable for CSIC_DMA_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
