#[doc = "Register `lcd_gint0` reader"]
pub struct R(crate::R<LCD_GINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_GINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_GINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_GINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_gint0` writer"]
pub struct W(crate::W<LCD_GINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_GINT0_SPEC>;
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
impl From<crate::W<LCD_GINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_GINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fsync_int_flag` reader - Asserted at the fsync signal in every frame\n\nWrite 0 to clear it."]
pub type FSYNC_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `fsync_int_flag` writer - Asserted at the fsync signal in every frame\n\nWrite 0 to clear it."]
pub type FSYNC_INT_FLAG_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `de_int_flag` reader - Asserted at the first valid line in every frame\n\nWrite 0 to clear it."]
pub type DE_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `de_int_flag` writer - Asserted at the first valid line in every frame\n\nWrite 0 to clear it."]
pub type DE_INT_FLAG_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `fsync_int_inv` reader - Enable the fsync interrupt to set signal inverse polarity.\n\nWhen FSYNC is positive, this bit must be 1.\n\nAnd vice versa."]
pub type FSYNC_INT_INV_R = crate::BitReader<bool>;
#[doc = "Field `fsync_int_inv` writer - Enable the fsync interrupt to set signal inverse polarity.\n\nWhen FSYNC is positive, this bit must be 1.\n\nAnd vice versa."]
pub type FSYNC_INT_INV_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `lcd_tri_underflow_flag` reader - Only used in dsi video mode, tri when sync by dsi but not finish\n\nWrite 0 to clear it."]
pub type LCD_TRI_UNDERFLOW_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `lcd_tri_underflow_flag` writer - Only used in dsi video mode, tri when sync by dsi but not finish\n\nWrite 0 to clear it."]
pub type LCD_TRI_UNDERFLOW_FLAG_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `lcd_tri_counter_int_flag` reader - Trigger when tri counter reaches this value\n\nWrite 0 to clear it."]
pub type LCD_TRI_COUNTER_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `lcd_tri_counter_int_flag` writer - Trigger when tri counter reaches this value\n\nWrite 0 to clear it."]
pub type LCD_TRI_COUNTER_INT_FLAG_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `lcd_tri_finish_int_flag` reader - Trigger when cpu trigger mode finished\n\nWrite 0 to clear it."]
pub type LCD_TRI_FINISH_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `lcd_tri_finish_int_flag` writer - Trigger when cpu trigger mode finished\n\nWrite 0 to clear it."]
pub type LCD_TRI_FINISH_INT_FLAG_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `lcd_line_int_flag` reader - Trigger when SY0 match the current LCD scan line\n\nWrite 0 to clear it."]
pub type LCD_LINE_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `lcd_line_int_flag` writer - Trigger when SY0 match the current LCD scan line\n\nWrite 0 to clear it."]
pub type LCD_LINE_INT_FLAG_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `lcd_vb_int_flag` reader - Asserted during vertical no-display period every frame\n\nWrite 0 to clear it."]
pub type LCD_VB_INT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `lcd_vb_int_flag` writer - Asserted during vertical no-display period every frame\n\nWrite 0 to clear it."]
pub type LCD_VB_INT_FLAG_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, LCD_GINT0_SPEC, bool, O>;
#[doc = "Field `lcd_tri_counter_int_en` reader - Enable the trigger counter interrupt"]
pub type LCD_TRI_COUNTER_INT_EN_R = crate::BitReader<LCD_TRI_COUNTER_INT_EN_A>;
#[doc = "Enable the trigger counter interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_TRI_COUNTER_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_TRI_COUNTER_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_TRI_COUNTER_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_TRI_COUNTER_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_TRI_COUNTER_INT_EN_A {
        match self.bits {
            false => LCD_TRI_COUNTER_INT_EN_A::DISABLE,
            true => LCD_TRI_COUNTER_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_TRI_COUNTER_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_TRI_COUNTER_INT_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_tri_counter_int_en` writer - Enable the trigger counter interrupt"]
pub type LCD_TRI_COUNTER_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_GINT0_SPEC, LCD_TRI_COUNTER_INT_EN_A, O>;
impl<'a, const O: u8> LCD_TRI_COUNTER_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_TRI_COUNTER_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_TRI_COUNTER_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `lcd_tri_finish_int_en` reader - Enable the trigger finish interrupt"]
pub type LCD_TRI_FINISH_INT_EN_R = crate::BitReader<LCD_TRI_FINISH_INT_EN_A>;
#[doc = "Enable the trigger finish interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_TRI_FINISH_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_TRI_FINISH_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_TRI_FINISH_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_TRI_FINISH_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_TRI_FINISH_INT_EN_A {
        match self.bits {
            false => LCD_TRI_FINISH_INT_EN_A::DISABLE,
            true => LCD_TRI_FINISH_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_TRI_FINISH_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_TRI_FINISH_INT_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_tri_finish_int_en` writer - Enable the trigger finish interrupt"]
pub type LCD_TRI_FINISH_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_GINT0_SPEC, LCD_TRI_FINISH_INT_EN_A, O>;
impl<'a, const O: u8> LCD_TRI_FINISH_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_TRI_FINISH_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_TRI_FINISH_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `lcd_line_int_en` reader - Enable the line interrupt"]
pub type LCD_LINE_INT_EN_R = crate::BitReader<LCD_LINE_INT_EN_A>;
#[doc = "Enable the line interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LINE_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_LINE_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LINE_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LINE_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LINE_INT_EN_A {
        match self.bits {
            false => LCD_LINE_INT_EN_A::DISABLE,
            true => LCD_LINE_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_LINE_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_LINE_INT_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_line_int_en` writer - Enable the line interrupt"]
pub type LCD_LINE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_GINT0_SPEC, LCD_LINE_INT_EN_A, O>;
impl<'a, const O: u8> LCD_LINE_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_LINE_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_LINE_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `lcd_vb_int_en` reader - Enable the Vb interrupt"]
pub type LCD_VB_INT_EN_R = crate::BitReader<LCD_VB_INT_EN_A>;
#[doc = "Enable the Vb interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_VB_INT_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_VB_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_VB_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_VB_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_VB_INT_EN_A {
        match self.bits {
            false => LCD_VB_INT_EN_A::DISABLE,
            true => LCD_VB_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_VB_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_VB_INT_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_vb_int_en` writer - Enable the Vb interrupt"]
pub type LCD_VB_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_GINT0_SPEC, LCD_VB_INT_EN_A, O>;
impl<'a, const O: u8> LCD_VB_INT_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_VB_INT_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_VB_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Asserted at the fsync signal in every frame\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn fsync_int_flag(&self) -> FSYNC_INT_FLAG_R {
        FSYNC_INT_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Asserted at the first valid line in every frame\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn de_int_flag(&self) -> DE_INT_FLAG_R {
        DE_INT_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the fsync interrupt to set signal inverse polarity.\n\nWhen FSYNC is positive, this bit must be 1.\n\nAnd vice versa."]
    #[inline(always)]
    pub fn fsync_int_inv(&self) -> FSYNC_INT_INV_R {
        FSYNC_INT_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - Only used in dsi video mode, tri when sync by dsi but not finish\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn lcd_tri_underflow_flag(&self) -> LCD_TRI_UNDERFLOW_FLAG_R {
        LCD_TRI_UNDERFLOW_FLAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trigger when tri counter reaches this value\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn lcd_tri_counter_int_flag(&self) -> LCD_TRI_COUNTER_INT_FLAG_R {
        LCD_TRI_COUNTER_INT_FLAG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Trigger when cpu trigger mode finished\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn lcd_tri_finish_int_flag(&self) -> LCD_TRI_FINISH_INT_FLAG_R {
        LCD_TRI_FINISH_INT_FLAG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Trigger when SY0 match the current LCD scan line\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn lcd_line_int_flag(&self) -> LCD_LINE_INT_FLAG_R {
        LCD_LINE_INT_FLAG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Asserted during vertical no-display period every frame\n\nWrite 0 to clear it."]
    #[inline(always)]
    pub fn lcd_vb_int_flag(&self) -> LCD_VB_INT_FLAG_R {
        LCD_VB_INT_FLAG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable the trigger counter interrupt"]
    #[inline(always)]
    pub fn lcd_tri_counter_int_en(&self) -> LCD_TRI_COUNTER_INT_EN_R {
        LCD_TRI_COUNTER_INT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable the trigger finish interrupt"]
    #[inline(always)]
    pub fn lcd_tri_finish_int_en(&self) -> LCD_TRI_FINISH_INT_EN_R {
        LCD_TRI_FINISH_INT_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable the line interrupt"]
    #[inline(always)]
    pub fn lcd_line_int_en(&self) -> LCD_LINE_INT_EN_R {
        LCD_LINE_INT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the Vb interrupt"]
    #[inline(always)]
    pub fn lcd_vb_int_en(&self) -> LCD_VB_INT_EN_R {
        LCD_VB_INT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asserted at the fsync signal in every frame\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn fsync_int_flag(&mut self) -> FSYNC_INT_FLAG_W<0> {
        FSYNC_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 1 - Asserted at the first valid line in every frame\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn de_int_flag(&mut self) -> DE_INT_FLAG_W<1> {
        DE_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 2 - Enable the fsync interrupt to set signal inverse polarity.\n\nWhen FSYNC is positive, this bit must be 1.\n\nAnd vice versa."]
    #[inline(always)]
    #[must_use]
    pub fn fsync_int_inv(&mut self) -> FSYNC_INT_INV_W<2> {
        FSYNC_INT_INV_W::new(self)
    }
    #[doc = "Bit 9 - Only used in dsi video mode, tri when sync by dsi but not finish\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tri_underflow_flag(&mut self) -> LCD_TRI_UNDERFLOW_FLAG_W<9> {
        LCD_TRI_UNDERFLOW_FLAG_W::new(self)
    }
    #[doc = "Bit 10 - Trigger when tri counter reaches this value\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tri_counter_int_flag(&mut self) -> LCD_TRI_COUNTER_INT_FLAG_W<10> {
        LCD_TRI_COUNTER_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 11 - Trigger when cpu trigger mode finished\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tri_finish_int_flag(&mut self) -> LCD_TRI_FINISH_INT_FLAG_W<11> {
        LCD_TRI_FINISH_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 13 - Trigger when SY0 match the current LCD scan line\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_line_int_flag(&mut self) -> LCD_LINE_INT_FLAG_W<13> {
        LCD_LINE_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 15 - Asserted during vertical no-display period every frame\n\nWrite 0 to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vb_int_flag(&mut self) -> LCD_VB_INT_FLAG_W<15> {
        LCD_VB_INT_FLAG_W::new(self)
    }
    #[doc = "Bit 26 - Enable the trigger counter interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tri_counter_int_en(&mut self) -> LCD_TRI_COUNTER_INT_EN_W<26> {
        LCD_TRI_COUNTER_INT_EN_W::new(self)
    }
    #[doc = "Bit 27 - Enable the trigger finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tri_finish_int_en(&mut self) -> LCD_TRI_FINISH_INT_EN_W<27> {
        LCD_TRI_FINISH_INT_EN_W::new(self)
    }
    #[doc = "Bit 29 - Enable the line interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_line_int_en(&mut self) -> LCD_LINE_INT_EN_W<29> {
        LCD_LINE_INT_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enable the Vb interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_vb_int_en(&mut self) -> LCD_VB_INT_EN_W<31> {
        LCD_VB_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Global Interrupt Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_gint0](index.html) module"]
pub struct LCD_GINT0_SPEC;
impl crate::RegisterSpec for LCD_GINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_gint0::R](R) reader structure"]
impl crate::Readable for LCD_GINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_gint0::W](W) writer structure"]
impl crate::Writable for LCD_GINT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xae07;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_gint0 to value 0"]
impl crate::Resettable for LCD_GINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
