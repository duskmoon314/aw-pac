#[doc = "Register `lcd_basic1` reader"]
pub struct R(crate::R<LCD_BASIC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_BASIC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_BASIC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_BASIC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_basic1` writer"]
pub struct W(crate::W<LCD_BASIC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_BASIC1_SPEC>;
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
impl From<crate::W<LCD_BASIC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_BASIC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbp` reader - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
pub type HBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `hbp` writer - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
pub type HBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC1_SPEC, u16, u16, 12, O>;
#[doc = "Field `ht` reader - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
pub type HT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ht` writer - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_BASIC1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:11 - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<0> {
        HBP_W::new(self)
    }
    #[doc = "Bits 16:28 - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<16> {
        HT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Basic Timing Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_basic1](index.html) module"]
pub struct LCD_BASIC1_SPEC;
impl crate::RegisterSpec for LCD_BASIC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_basic1::R](R) reader structure"]
impl crate::Readable for LCD_BASIC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_basic1::W](W) writer structure"]
impl crate::Writable for LCD_BASIC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic1 to value 0"]
impl crate::Resettable for LCD_BASIC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
