#[doc = "Register `lcd_gamma_table%s` reader"]
pub struct R(crate::R<LCD_GAMMA_TABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_GAMMA_TABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_GAMMA_TABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_GAMMA_TABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_gamma_table%s` writer"]
pub struct W(crate::W<LCD_GAMMA_TABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_GAMMA_TABLE_SPEC>;
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
impl From<crate::W<LCD_GAMMA_TABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_GAMMA_TABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `blue_comp` reader - Blue Component"]
pub type BLUE_COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `blue_comp` writer - Blue Component"]
pub type BLUE_COMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_GAMMA_TABLE_SPEC, u8, u8, 8, O>;
#[doc = "Field `green_comp` reader - Green Component"]
pub type GREEN_COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `green_comp` writer - Green Component"]
pub type GREEN_COMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_GAMMA_TABLE_SPEC, u8, u8, 8, O>;
#[doc = "Field `red_comp` reader - Red Component"]
pub type RED_COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `red_comp` writer - Red Component"]
pub type RED_COMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_GAMMA_TABLE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Blue Component"]
    #[inline(always)]
    pub fn blue_comp(&self) -> BLUE_COMP_R {
        BLUE_COMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Component"]
    #[inline(always)]
    pub fn green_comp(&self) -> GREEN_COMP_R {
        GREEN_COMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red Component"]
    #[inline(always)]
    pub fn red_comp(&self) -> RED_COMP_R {
        RED_COMP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue Component"]
    #[inline(always)]
    #[must_use]
    pub fn blue_comp(&mut self) -> BLUE_COMP_W<0> {
        BLUE_COMP_W::new(self)
    }
    #[doc = "Bits 8:15 - Green Component"]
    #[inline(always)]
    #[must_use]
    pub fn green_comp(&mut self) -> GREEN_COMP_W<8> {
        GREEN_COMP_W::new(self)
    }
    #[doc = "Bits 16:23 - Red Component"]
    #[inline(always)]
    #[must_use]
    pub fn red_comp(&mut self) -> RED_COMP_W<16> {
        RED_COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Gamma Table Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_gamma_table](index.html) module"]
pub struct LCD_GAMMA_TABLE_SPEC;
impl crate::RegisterSpec for LCD_GAMMA_TABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_gamma_table::R](R) reader structure"]
impl crate::Readable for LCD_GAMMA_TABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_gamma_table::W](W) writer structure"]
impl crate::Writable for LCD_GAMMA_TABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_gamma_table%s to value 0"]
impl crate::Resettable for LCD_GAMMA_TABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
