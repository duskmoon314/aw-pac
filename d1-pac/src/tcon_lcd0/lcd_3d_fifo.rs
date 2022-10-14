#[doc = "Register `lcd_3d_fifo` reader"]
pub struct R(crate::R<LCD_3D_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_3D_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_3D_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_3D_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_3d_fifo` writer"]
pub struct W(crate::W<LCD_3D_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_3D_FIFO_SPEC>;
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
impl From<crate::W<LCD_3D_FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_3D_FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `setting` reader - Set the work mode of 3D FIFO"]
pub type SETTING_R = crate::FieldReader<u8, SETTING_A>;
#[doc = "Set the work mode of 3D FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETTING_A {
    #[doc = "0: Bypass"]
    B_YPASS = 0,
    #[doc = "1: Used as normal FIFO"]
    N_ORMAL = 1,
    #[doc = "2: Used as 3D interlace FIFO"]
    I_NTERLACE_3D = 2,
}
impl From<SETTING_A> for u8 {
    #[inline(always)]
    fn from(variant: SETTING_A) -> Self {
        variant as _
    }
}
impl SETTING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETTING_A> {
        match self.bits {
            0 => Some(SETTING_A::B_YPASS),
            1 => Some(SETTING_A::N_ORMAL),
            2 => Some(SETTING_A::I_NTERLACE_3D),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_YPASS`"]
    #[inline(always)]
    pub fn is_b_ypass(&self) -> bool {
        *self == SETTING_A::B_YPASS
    }
    #[doc = "Checks if the value of the field is `N_ORMAL`"]
    #[inline(always)]
    pub fn is_n_ormal(&self) -> bool {
        *self == SETTING_A::N_ORMAL
    }
    #[doc = "Checks if the value of the field is `I_NTERLACE_3D`"]
    #[inline(always)]
    pub fn is_i_nterlace_3d(&self) -> bool {
        *self == SETTING_A::I_NTERLACE_3D
    }
}
#[doc = "Field `setting` writer - Set the work mode of 3D FIFO"]
pub type SETTING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_3D_FIFO_SPEC, u8, SETTING_A, 2, O>;
impl<'a, const O: u8> SETTING_W<'a, O> {
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn b_ypass(self) -> &'a mut W {
        self.variant(SETTING_A::B_YPASS)
    }
    #[doc = "Used as normal FIFO"]
    #[inline(always)]
    pub fn n_ormal(self) -> &'a mut W {
        self.variant(SETTING_A::N_ORMAL)
    }
    #[doc = "Used as 3D interlace FIFO"]
    #[inline(always)]
    pub fn i_nterlace_3d(self) -> &'a mut W {
        self.variant(SETTING_A::I_NTERLACE_3D)
    }
}
#[doc = "Field `half_line_size` reader - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
pub type HALF_LINE_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `half_line_size` writer - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
pub type HALF_LINE_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_3D_FIFO_SPEC, u16, u16, 10, O>;
#[doc = "Field `bist_en` reader - Enable the 3D fifo bist test function"]
pub type BIST_EN_R = crate::BitReader<BIST_EN_A>;
#[doc = "Enable the 3D fifo bist test function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_EN_A {
    #[doc = "0: Disable"]
    D_ISABLE = 0,
    #[doc = "1: Enable"]
    E_NABLE = 1,
}
impl From<BIST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIST_EN_A {
        match self.bits {
            false => BIST_EN_A::D_ISABLE,
            true => BIST_EN_A::E_NABLE,
        }
    }
    #[doc = "Checks if the value of the field is `D_ISABLE`"]
    #[inline(always)]
    pub fn is_d_isable(&self) -> bool {
        *self == BIST_EN_A::D_ISABLE
    }
    #[doc = "Checks if the value of the field is `E_NABLE`"]
    #[inline(always)]
    pub fn is_e_nable(&self) -> bool {
        *self == BIST_EN_A::E_NABLE
    }
}
#[doc = "Field `bist_en` writer - Enable the 3D fifo bist test function"]
pub type BIST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_3D_FIFO_SPEC, BIST_EN_A, O>;
impl<'a, const O: u8> BIST_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn d_isable(self) -> &'a mut W {
        self.variant(BIST_EN_A::D_ISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn e_nable(self) -> &'a mut W {
        self.variant(BIST_EN_A::E_NABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set the work mode of 3D FIFO"]
    #[inline(always)]
    pub fn setting(&self) -> SETTING_R {
        SETTING_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:13 - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
    #[inline(always)]
    pub fn half_line_size(&self) -> HALF_LINE_SIZE_R {
        HALF_LINE_SIZE_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable the 3D fifo bist test function"]
    #[inline(always)]
    pub fn bist_en(&self) -> BIST_EN_R {
        BIST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set the work mode of 3D FIFO"]
    #[inline(always)]
    pub fn setting(&mut self) -> SETTING_W<0> {
        SETTING_W::new(self)
    }
    #[doc = "Bits 4:13 - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
    #[inline(always)]
    pub fn half_line_size(&mut self) -> HALF_LINE_SIZE_W<4> {
        HALF_LINE_SIZE_W::new(self)
    }
    #[doc = "Bit 31 - Enable the 3D fifo bist test function"]
    #[inline(always)]
    pub fn bist_en(&mut self) -> BIST_EN_W<31> {
        BIST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD 3D FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_3d_fifo](index.html) module"]
pub struct LCD_3D_FIFO_SPEC;
impl crate::RegisterSpec for LCD_3D_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_3d_fifo::R](R) reader structure"]
impl crate::Readable for LCD_3D_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_3d_fifo::W](W) writer structure"]
impl crate::Writable for LCD_3D_FIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lcd_3d_fifo to value 0"]
impl crate::Resettable for LCD_3D_FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
