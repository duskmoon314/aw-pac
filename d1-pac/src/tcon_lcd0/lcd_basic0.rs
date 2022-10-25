#[doc = "Register `lcd_basic0` reader"]
pub struct R(crate::R<LCD_BASIC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_BASIC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_BASIC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_BASIC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_basic0` writer"]
pub struct W(crate::W<LCD_BASIC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_BASIC0_SPEC>;
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
impl From<crate::W<LCD_BASIC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_BASIC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `height_y` reader - Panel height is Y+1"]
pub type HEIGHT_Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `height_y` writer - Panel height is Y+1"]
pub type HEIGHT_Y_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_BASIC0_SPEC, u16, u16, 12, O>;
#[doc = "Field `width_x` reader - Panel width is X+1"]
pub type WIDTH_X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `width_x` writer - Panel width is X+1"]
pub type WIDTH_X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Panel height is Y+1"]
    #[inline(always)]
    pub fn height_y(&self) -> HEIGHT_Y_R {
        HEIGHT_Y_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Panel width is X+1"]
    #[inline(always)]
    pub fn width_x(&self) -> WIDTH_X_R {
        WIDTH_X_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Panel height is Y+1"]
    #[inline(always)]
    #[must_use]
    pub fn height_y(&mut self) -> HEIGHT_Y_W<0> {
        HEIGHT_Y_W::new(self)
    }
    #[doc = "Bits 16:27 - Panel width is X+1"]
    #[inline(always)]
    #[must_use]
    pub fn width_x(&mut self) -> WIDTH_X_W<16> {
        WIDTH_X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Basic Timing Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_basic0](index.html) module"]
pub struct LCD_BASIC0_SPEC;
impl crate::RegisterSpec for LCD_BASIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_basic0::R](R) reader structure"]
impl crate::Readable for LCD_BASIC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_basic0::W](W) writer structure"]
impl crate::Writable for LCD_BASIC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic0 to value 0"]
impl crate::Resettable for LCD_BASIC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
