#[doc = "Register `lcd_basic3` reader"]
pub struct R(crate::R<LCD_BASIC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_BASIC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_BASIC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_BASIC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_basic3` writer"]
pub struct W(crate::W<LCD_BASIC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_BASIC3_SPEC>;
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
impl From<crate::W<LCD_BASIC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_BASIC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vspw` reader - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
pub type VSPW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `vspw` writer - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
pub type VSPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC3_SPEC, u16, u16, 10, O>;
#[doc = "Field `hspw` reader - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
pub type HSPW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `hspw` writer - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
pub type HSPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC3_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
    #[inline(always)]
    pub fn vspw(&self) -> VSPW_R {
        VSPW_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
    #[inline(always)]
    pub fn hspw(&self) -> HSPW_R {
        HSPW_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn vspw(&mut self) -> VSPW_W<0> {
        VSPW_W::new(self)
    }
    #[doc = "Bits 16:25 - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hspw(&mut self) -> HSPW_W<16> {
        HSPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Basic Timing Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_basic3](index.html) module"]
pub struct LCD_BASIC3_SPEC;
impl crate::RegisterSpec for LCD_BASIC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_basic3::R](R) reader structure"]
impl crate::Readable for LCD_BASIC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_basic3::W](W) writer structure"]
impl crate::Writable for LCD_BASIC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic3 to value 0"]
impl crate::Resettable for LCD_BASIC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
