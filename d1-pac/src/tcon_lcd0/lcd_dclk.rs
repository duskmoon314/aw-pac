#[doc = "Register `lcd_dclk` reader"]
pub struct R(crate::R<LCD_DCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_DCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_DCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_DCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_dclk` writer"]
pub struct W(crate::W<LCD_DCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_DCLK_SPEC>;
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
impl From<crate::W<LCD_DCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_DCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lcd_dclk_div` reader - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
pub type LCD_DCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lcd_dclk_div` writer - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
pub type LCD_DCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_DCLK_SPEC, u8, u8, 7, O>;
#[doc = "Field `lcd_dclk_en` reader - LCD clock enable\n\n"]
pub type LCD_DCLK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lcd_dclk_en` writer - LCD clock enable\n\n"]
pub type LCD_DCLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_DCLK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:6 - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
    #[inline(always)]
    pub fn lcd_dclk_div(&self) -> LCD_DCLK_DIV_R {
        LCD_DCLK_DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - LCD clock enable\n\n"]
    #[inline(always)]
    pub fn lcd_dclk_en(&self) -> LCD_DCLK_EN_R {
        LCD_DCLK_EN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dclk_div(&mut self) -> LCD_DCLK_DIV_W<0> {
        LCD_DCLK_DIV_W::new(self)
    }
    #[doc = "Bits 28:31 - LCD clock enable\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dclk_en(&mut self) -> LCD_DCLK_EN_W<28> {
        LCD_DCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Data Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_dclk](index.html) module"]
pub struct LCD_DCLK_SPEC;
impl crate::RegisterSpec for LCD_DCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_dclk::R](R) reader structure"]
impl crate::Readable for LCD_DCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_dclk::W](W) writer structure"]
impl crate::Writable for LCD_DCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_dclk to value 0"]
impl crate::Resettable for LCD_DCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
