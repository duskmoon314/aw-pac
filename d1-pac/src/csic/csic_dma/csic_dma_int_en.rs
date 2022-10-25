#[doc = "Register `csic_dma_int_en` reader"]
pub struct R(crate::R<CSIC_DMA_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_int_en` writer"]
pub struct W(crate::W<CSIC_DMA_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSIC_DMA_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cd_int_en` reader - Capture done\n\nIndicates the CSI has completed capturing the image data. For still capture, the bit is set when one frame data has been written to buffer. For video capture, the bit is set when the last frame has been written to buffer after video capture has been disabled. For CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means field end."]
pub type CD_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cd_int_en` writer - Capture done\n\nIndicates the CSI has completed capturing the image data. For still capture, the bit is set when one frame data has been written to buffer. For video capture, the bit is set when the last frame has been written to buffer after video capture has been disabled. For CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means field end."]
pub type CD_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `fd_int_en` reader - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
pub type FD_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `fd_int_en` writer - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
pub type FD_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `fifo0_of_int_en` reader - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
pub type FIFO0_OF_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `fifo0_of_int_en` writer - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
pub type FIFO0_OF_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `fifo1_of_int_en` reader - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
pub type FIFO1_OF_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `fifo1_of_int_en` writer - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
pub type FIFO1_OF_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `fifo2_of_int_en` reader - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
pub type FIFO2_OF_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `fifo2_of_int_en` writer - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
pub type FIFO2_OF_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `lc_int_en` reader - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
pub type LC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `lc_int_en` writer - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
pub type LC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `hb_of_int_en` reader - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
pub type HB_OF_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `hb_of_int_en` writer - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
pub type HB_OF_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `vs_int_en` reader - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
pub type VS_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `vs_int_en` writer - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
pub type VS_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `clr_frame_cnt_int_en` reader - Set a INT when clear Frame cnt."]
pub type CLR_FRAME_CNT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `clr_frame_cnt_int_en` writer - Set a INT when clear Frame cnt."]
pub type CLR_FRAME_CNT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `video_input_to_int_en` reader - Set an INT when no video input exceeds the setting threshold time"]
pub type VIDEO_INPUT_TO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `video_input_to_int_en` writer - Set an INT when no video input exceeds the setting threshold time"]
pub type VIDEO_INPUT_TO_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `buf_addr_fifo_int_en` reader - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type BUF_ADDR_FIFO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `buf_addr_fifo_int_en` writer - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type BUF_ADDR_FIFO_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `stored_frm_cnt_int_en` reader - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type STORED_FRM_CNT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `stored_frm_cnt_int_en` writer - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
pub type STORED_FRM_CNT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
#[doc = "Field `frm_lost_int_en` reader - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
pub type FRM_LOST_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `frm_lost_int_en` writer - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
pub type FRM_LOST_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_INT_EN_SPEC, bool, O>;
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
    pub fn cd_int_en(&mut self) -> CD_INT_EN_W<0> {
        CD_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Frame done\n\nIndicates the CSI has finished capturing an image frame. Applies to video capture mode. The bit is set after each completed frame capturing data is written to buffer as long as video capture remains enabled."]
    #[inline(always)]
    #[must_use]
    pub fn fd_int_en(&mut self) -> FD_INT_EN_W<1> {
        FD_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - FIFO 0 overflow\n\nThe bit is set when the FIFO 0 became overflow."]
    #[inline(always)]
    #[must_use]
    pub fn fifo0_of_int_en(&mut self) -> FIFO0_OF_INT_EN_W<2> {
        FIFO0_OF_INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - FIFO 1 overflow\n\nThe bit is set when the FIFO 1 became overflow."]
    #[inline(always)]
    #[must_use]
    pub fn fifo1_of_int_en(&mut self) -> FIFO1_OF_INT_EN_W<3> {
        FIFO1_OF_INT_EN_W::new(self)
    }
    #[doc = "Bit 4 - FIFO 2 overflow\n\nThe bit is set when the FIFO 2 became overflow."]
    #[inline(always)]
    #[must_use]
    pub fn fifo2_of_int_en(&mut self) -> FIFO2_OF_INT_EN_W<4> {
        FIFO2_OF_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Line counter flag The bit is set when the specific line has been written to dram every frame. The line number is set in the line counter register."]
    #[inline(always)]
    #[must_use]
    pub fn lc_int_en(&mut self) -> LC_INT_EN_W<5> {
        LC_INT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Hblank FIFO overflow The bit is set when 3 FIFOs still overflow after the hblank."]
    #[inline(always)]
    #[must_use]
    pub fn hb_of_int_en(&mut self) -> HB_OF_INT_EN_W<6> {
        HB_OF_INT_EN_W::new(self)
    }
    #[doc = "Bit 7 - vsync flag\n\nThe bit is set when vsync come. And at this time load the buffer address for the coming frame. So after this irq come, changing the buffer address could only effect next frame"]
    #[inline(always)]
    #[must_use]
    pub fn vs_int_en(&mut self) -> VS_INT_EN_W<7> {
        VS_INT_EN_W::new(self)
    }
    #[doc = "Bit 11 - Set a INT when clear Frame cnt."]
    #[inline(always)]
    #[must_use]
    pub fn clr_frame_cnt_int_en(&mut self) -> CLR_FRAME_CNT_INT_EN_W<11> {
        CLR_FRAME_CNT_INT_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set an INT when no video input exceeds the setting threshold time"]
    #[inline(always)]
    #[must_use]
    pub fn video_input_to_int_en(&mut self) -> VIDEO_INPUT_TO_INT_EN_W<12> {
        VIDEO_INPUT_TO_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - Set an INT when content in BUF Address FIFO less than CSIC_DMA_BUFA_FIFO_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn buf_addr_fifo_int_en(&mut self) -> BUF_ADDR_FIFO_INT_EN_W<13> {
        BUF_ADDR_FIFO_INT_EN_W::new(self)
    }
    #[doc = "Bit 14 - Set an INT when the value of CSIC_DMA_STORED_FRM_CNT reaches CSIC_DMA_STORED_FRM_THRESHOLD, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn stored_frm_cnt_int_en(&mut self) -> STORED_FRM_CNT_INT_EN_W<14> {
        STORED_FRM_CNT_INT_EN_W::new(self)
    }
    #[doc = "Bit 15 - Set an INT when frame starts with empty Buffer Address FIFO, only use in BUF Address FIFO MODE."]
    #[inline(always)]
    #[must_use]
    pub fn frm_lost_int_en(&mut self) -> FRM_LOST_INT_EN_W<15> {
        FRM_LOST_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_int_en](index.html) module"]
pub struct CSIC_DMA_INT_EN_SPEC;
impl crate::RegisterSpec for CSIC_DMA_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_int_en::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_int_en::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_int_en to value 0"]
impl crate::Resettable for CSIC_DMA_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
