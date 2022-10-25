#[doc = "Register `lcd_gctl` reader"]
pub struct R(crate::R<LCD_GCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_GCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_GCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_GCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_gctl` writer"]
pub struct W(crate::W<LCD_GCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_GCTL_SPEC>;
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
impl From<crate::W<LCD_GCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_GCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lcd_gamma_en` reader - Enable the Gamma correction function."]
pub type LCD_GAMMA_EN_R = crate::BitReader<LCD_GAMMA_EN_A>;
#[doc = "Enable the Gamma correction function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_GAMMA_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_GAMMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_GAMMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_GAMMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_GAMMA_EN_A {
        match self.bits {
            false => LCD_GAMMA_EN_A::DISABLE,
            true => LCD_GAMMA_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_GAMMA_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_GAMMA_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_gamma_en` writer - Enable the Gamma correction function."]
pub type LCD_GAMMA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_GCTL_SPEC, LCD_GAMMA_EN_A, O>;
impl<'a, const O: u8> LCD_GAMMA_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_GAMMA_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_GAMMA_EN_A::ENABLE)
    }
}
#[doc = "Field `lcd_en` reader - When it is disabled, the module will be reset to idle state."]
pub type LCD_EN_R = crate::BitReader<LCD_EN_A>;
#[doc = "When it is disabled, the module will be reset to idle state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_EN_A {
        match self.bits {
            false => LCD_EN_A::DISABLE,
            true => LCD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_en` writer - When it is disabled, the module will be reset to idle state."]
pub type LCD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_GCTL_SPEC, LCD_EN_A, O>;
impl<'a, const O: u8> LCD_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 30 - Enable the Gamma correction function."]
    #[inline(always)]
    pub fn lcd_gamma_en(&self) -> LCD_GAMMA_EN_R {
        LCD_GAMMA_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When it is disabled, the module will be reset to idle state."]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enable the Gamma correction function."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_gamma_en(&mut self) -> LCD_GAMMA_EN_W<30> {
        LCD_GAMMA_EN_W::new(self)
    }
    #[doc = "Bit 31 - When it is disabled, the module will be reset to idle state."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_en(&mut self) -> LCD_EN_W<31> {
        LCD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_gctl](index.html) module"]
pub struct LCD_GCTL_SPEC;
impl crate::RegisterSpec for LCD_GCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_gctl::R](R) reader structure"]
impl crate::Readable for LCD_GCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_gctl::W](W) writer structure"]
impl crate::Writable for LCD_GCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_gctl to value 0"]
impl crate::Resettable for LCD_GCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
