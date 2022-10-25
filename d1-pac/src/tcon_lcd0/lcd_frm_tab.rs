#[doc = "Register `lcd_frm_tab%s` reader"]
pub struct R(crate::R<LCD_FRM_TAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_FRM_TAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_FRM_TAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_FRM_TAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_frm_tab%s` writer"]
pub struct W(crate::W<LCD_FRM_TAB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_FRM_TAB_SPEC>;
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
impl From<crate::W<LCD_FRM_TAB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_FRM_TAB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frm_table_value` reader - Set the data used in dither function"]
pub type FRM_TABLE_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `frm_table_value` writer - Set the data used in dither function"]
pub type FRM_TABLE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_FRM_TAB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Set the data used in dither function"]
    #[inline(always)]
    pub fn frm_table_value(&self) -> FRM_TABLE_VALUE_R {
        FRM_TABLE_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the data used in dither function"]
    #[inline(always)]
    #[must_use]
    pub fn frm_table_value(&mut self) -> FRM_TABLE_VALUE_W<0> {
        FRM_TABLE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD FRM Table Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_frm_tab](index.html) module"]
pub struct LCD_FRM_TAB_SPEC;
impl crate::RegisterSpec for LCD_FRM_TAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_frm_tab::R](R) reader structure"]
impl crate::Readable for LCD_FRM_TAB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_frm_tab::W](W) writer structure"]
impl crate::Writable for LCD_FRM_TAB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_frm_tab%s to value 0"]
impl crate::Resettable for LCD_FRM_TAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
