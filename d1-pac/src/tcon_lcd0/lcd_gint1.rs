#[doc = "Register `lcd_gint1` reader"]
pub struct R(crate::R<LCD_GINT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_GINT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_GINT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_GINT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_gint1` writer"]
pub struct W(crate::W<LCD_GINT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_GINT1_SPEC>;
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
impl From<crate::W<LCD_GINT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_GINT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lcd_line_int_num` reader - Scan line for LCD line trigger (including inactive lines).\n\nSetting it for the specified line for trigger0.\n\nNote: SY0 is writable only when LINE_TRG0 is disabled."]
pub type LCD_LINE_INT_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `lcd_line_int_num` writer - Scan line for LCD line trigger (including inactive lines).\n\nSetting it for the specified line for trigger0.\n\nNote: SY0 is writable only when LINE_TRG0 is disabled."]
pub type LCD_LINE_INT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_GINT1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 16:27 - Scan line for LCD line trigger (including inactive lines).\n\nSetting it for the specified line for trigger0.\n\nNote: SY0 is writable only when LINE_TRG0 is disabled."]
    #[inline(always)]
    pub fn lcd_line_int_num(&self) -> LCD_LINE_INT_NUM_R {
        LCD_LINE_INT_NUM_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Scan line for LCD line trigger (including inactive lines).\n\nSetting it for the specified line for trigger0.\n\nNote: SY0 is writable only when LINE_TRG0 is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_line_int_num(&mut self) -> LCD_LINE_INT_NUM_W<16> {
        LCD_LINE_INT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Global Interrupt Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_gint1](index.html) module"]
pub struct LCD_GINT1_SPEC;
impl crate::RegisterSpec for LCD_GINT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_gint1::R](R) reader structure"]
impl crate::Readable for LCD_GINT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_gint1::W](W) writer structure"]
impl crate::Writable for LCD_GINT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_gint1 to value 0"]
impl crate::Resettable for LCD_GINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
