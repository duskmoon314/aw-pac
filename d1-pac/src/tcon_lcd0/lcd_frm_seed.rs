#[doc = "Register `lcd_frm_seed%s` reader"]
pub struct R(crate::R<LCD_FRM_SEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_FRM_SEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_FRM_SEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_FRM_SEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_frm_seed%s` writer"]
pub struct W(crate::W<LCD_FRM_SEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_FRM_SEED_SPEC>;
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
impl From<crate::W<LCD_FRM_SEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_FRM_SEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `seed_value` reader - Set the seed used in dither function\n\nNote: Avoid setting it to 0."]
pub type SEED_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `seed_value` writer - Set the seed used in dither function\n\nNote: Avoid setting it to 0."]
pub type SEED_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_FRM_SEED_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:24 - Set the seed used in dither function\n\nNote: Avoid setting it to 0."]
    #[inline(always)]
    pub fn seed_value(&self) -> SEED_VALUE_R {
        SEED_VALUE_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Set the seed used in dither function\n\nNote: Avoid setting it to 0."]
    #[inline(always)]
    #[must_use]
    pub fn seed_value(&mut self) -> SEED_VALUE_W<0> {
        SEED_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD FRM Seed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_frm_seed](index.html) module"]
pub struct LCD_FRM_SEED_SPEC;
impl crate::RegisterSpec for LCD_FRM_SEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_frm_seed::R](R) reader structure"]
impl crate::Readable for LCD_FRM_SEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_frm_seed::W](W) writer structure"]
impl crate::Writable for LCD_FRM_SEED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_frm_seed%s to value 0"]
impl crate::Resettable for LCD_FRM_SEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
