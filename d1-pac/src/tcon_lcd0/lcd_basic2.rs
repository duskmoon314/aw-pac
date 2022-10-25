#[doc = "Register `lcd_basic2` reader"]
pub struct R(crate::R<LCD_BASIC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_BASIC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_BASIC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_BASIC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_basic2` writer"]
pub struct W(crate::W<LCD_BASIC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_BASIC2_SPEC>;
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
impl From<crate::W<LCD_BASIC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_BASIC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vbp` reader - Tvbp = (VBP +1) * Thsync"]
pub type VBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `vbp` writer - Tvbp = (VBP +1) * Thsync"]
pub type VBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC2_SPEC, u16, u16, 12, O>;
#[doc = "Field `vt` reader - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
pub type VT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `vt` writer - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
pub type VT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC2_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:11 - Tvbp = (VBP +1) * Thsync"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
    #[inline(always)]
    pub fn vt(&self) -> VT_R {
        VT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Tvbp = (VBP +1) * Thsync"]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<0> {
        VBP_W::new(self)
    }
    #[doc = "Bits 16:28 - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
    #[inline(always)]
    #[must_use]
    pub fn vt(&mut self) -> VT_W<16> {
        VT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Basic Timing Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_basic2](index.html) module"]
pub struct LCD_BASIC2_SPEC;
impl crate::RegisterSpec for LCD_BASIC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_basic2::R](R) reader structure"]
impl crate::Readable for LCD_BASIC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_basic2::W](W) writer structure"]
impl crate::Writable for LCD_BASIC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic2 to value 0"]
impl crate::Resettable for LCD_BASIC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
