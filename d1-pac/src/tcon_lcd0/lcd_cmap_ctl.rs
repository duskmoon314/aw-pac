#[doc = "Register `lcd_cmap_ctl` reader"]
pub struct R(crate::R<LCD_CMAP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CMAP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CMAP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CMAP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cmap_ctl` writer"]
pub struct W(crate::W<LCD_CMAP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CMAP_CTL_SPEC>;
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
impl From<crate::W<LCD_CMAP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CMAP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `out_format` reader - Set the pixel output format in color map function."]
pub type OUT_FORMAT_R = crate::BitReader<OUT_FORMAT_A>;
#[doc = "Set the pixel output format in color map function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT_FORMAT_A {
    #[doc = "0: 4 pixel output mode: Out0 -> Out1 -> Out2 -> Out3"]
    P4 = 0,
    #[doc = "1: 2 pixel output mode: Out0 -> Out1"]
    P2 = 1,
}
impl From<OUT_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: OUT_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT_FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_FORMAT_A {
        match self.bits {
            false => OUT_FORMAT_A::P4,
            true => OUT_FORMAT_A::P2,
        }
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == OUT_FORMAT_A::P4
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == OUT_FORMAT_A::P2
    }
}
#[doc = "Field `out_format` writer - Set the pixel output format in color map function."]
pub type OUT_FORMAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_CMAP_CTL_SPEC, OUT_FORMAT_A, O>;
impl<'a, const O: u8> OUT_FORMAT_W<'a, O> {
    #[doc = "4 pixel output mode: Out0 -> Out1 -> Out2 -> Out3"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(OUT_FORMAT_A::P4)
    }
    #[doc = "2 pixel output mode: Out0 -> Out1"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(OUT_FORMAT_A::P2)
    }
}
#[doc = "Field `color_map_en` reader - Enable the color map function. This module only works when X is divided by 4."]
pub type COLOR_MAP_EN_R = crate::BitReader<COLOR_MAP_EN_A>;
#[doc = "Enable the color map function. This module only works when X is divided by 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLOR_MAP_EN_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<COLOR_MAP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COLOR_MAP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COLOR_MAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLOR_MAP_EN_A {
        match self.bits {
            false => COLOR_MAP_EN_A::BYPASS,
            true => COLOR_MAP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == COLOR_MAP_EN_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COLOR_MAP_EN_A::ENABLE
    }
}
#[doc = "Field `color_map_en` writer - Enable the color map function. This module only works when X is divided by 4."]
pub type COLOR_MAP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_CMAP_CTL_SPEC, COLOR_MAP_EN_A, O>;
impl<'a, const O: u8> COLOR_MAP_EN_W<'a, O> {
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(COLOR_MAP_EN_A::BYPASS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COLOR_MAP_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Set the pixel output format in color map function."]
    #[inline(always)]
    pub fn out_format(&self) -> OUT_FORMAT_R {
        OUT_FORMAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the color map function. This module only works when X is divided by 4."]
    #[inline(always)]
    pub fn color_map_en(&self) -> COLOR_MAP_EN_R {
        COLOR_MAP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set the pixel output format in color map function."]
    #[inline(always)]
    #[must_use]
    pub fn out_format(&mut self) -> OUT_FORMAT_W<0> {
        OUT_FORMAT_W::new(self)
    }
    #[doc = "Bit 31 - Enable the color map function. This module only works when X is divided by 4."]
    #[inline(always)]
    #[must_use]
    pub fn color_map_en(&mut self) -> COLOR_MAP_EN_W<31> {
        COLOR_MAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Color Map Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cmap_ctl](index.html) module"]
pub struct LCD_CMAP_CTL_SPEC;
impl crate::RegisterSpec for LCD_CMAP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cmap_ctl::R](R) reader structure"]
impl crate::Readable for LCD_CMAP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cmap_ctl::W](W) writer structure"]
impl crate::Writable for LCD_CMAP_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cmap_ctl to value 0"]
impl crate::Resettable for LCD_CMAP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
