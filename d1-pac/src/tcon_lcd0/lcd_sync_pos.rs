#[doc = "Register `lcd_sync_pos` reader"]
pub type R = crate::R<LCD_SYNC_POS_SPEC>;
#[doc = "Register `lcd_sync_pos` writer"]
pub type W = crate::W<LCD_SYNC_POS_SPEC>;
#[doc = "Field `lcd_sync_line_num` reader - Set the line number of master LCD controller which is used to trigger the slave LCD controller to start working.\n\nNote:It is only set in master LCD controller.It is not necessarily to set in slave LCD controller.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
pub type LCD_SYNC_LINE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `lcd_sync_line_num` writer - Set the line number of master LCD controller which is used to trigger the slave LCD controller to start working.\n\nNote:It is only set in master LCD controller.It is not necessarily to set in slave LCD controller.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
pub type LCD_SYNC_LINE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `lcd_sync_pixel_num` reader - Set the pixel number of master LCD controller which is used to trigger the slave LCD controller to start working.This value is the number of pixels between the trigger point and the end of the line.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
pub type LCD_SYNC_PIXEL_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `lcd_sync_pixel_num` writer - Set the pixel number of master LCD controller which is used to trigger the slave LCD controller to start working.This value is the number of pixels between the trigger point and the end of the line.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
pub type LCD_SYNC_PIXEL_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Set the line number of master LCD controller which is used to trigger the slave LCD controller to start working.\n\nNote:It is only set in master LCD controller.It is not necessarily to set in slave LCD controller.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    pub fn lcd_sync_line_num(&self) -> LCD_SYNC_LINE_NUM_R {
        LCD_SYNC_LINE_NUM_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Set the pixel number of master LCD controller which is used to trigger the slave LCD controller to start working.This value is the number of pixels between the trigger point and the end of the line.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    pub fn lcd_sync_pixel_num(&self) -> LCD_SYNC_PIXEL_NUM_R {
        LCD_SYNC_PIXEL_NUM_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Set the line number of master LCD controller which is used to trigger the slave LCD controller to start working.\n\nNote:It is only set in master LCD controller.It is not necessarily to set in slave LCD controller.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_sync_line_num(&mut self) -> LCD_SYNC_LINE_NUM_W<LCD_SYNC_POS_SPEC> {
        LCD_SYNC_LINE_NUM_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Set the pixel number of master LCD controller which is used to trigger the slave LCD controller to start working.This value is the number of pixels between the trigger point and the end of the line.\n\nTri pos = Tline*LCD_Sync_Line_Num+Tpixel*(HT-LCD_Sync_Pixel_Num)\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_sync_pixel_num(&mut self) -> LCD_SYNC_PIXEL_NUM_W<LCD_SYNC_POS_SPEC> {
        LCD_SYNC_PIXEL_NUM_W::new(self, 16)
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
#[doc = "LCD Sync Position Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_sync_pos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_sync_pos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_SYNC_POS_SPEC;
impl crate::RegisterSpec for LCD_SYNC_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_sync_pos::R`](R) reader structure"]
impl crate::Readable for LCD_SYNC_POS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_sync_pos::W`](W) writer structure"]
impl crate::Writable for LCD_SYNC_POS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_sync_pos to value 0"]
impl crate::Resettable for LCD_SYNC_POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
