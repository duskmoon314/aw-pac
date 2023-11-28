#[doc = "Register `lcd_frm_ctl` reader"]
pub type R = crate::R<LCD_FRM_CTL_SPEC>;
#[doc = "Register `lcd_frm_ctl` writer"]
pub type W = crate::W<LCD_FRM_CTL_SPEC>;
#[doc = "Field `lcd_frm_test` reader - Set the test mode of dither function"]
pub type LCD_FRM_TEST_R = crate::FieldReader<LCD_FRM_TEST_A>;
#[doc = "Set the test mode of dither function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCD_FRM_TEST_A {
    #[doc = "0: FRM"]
    FRM = 0,
    #[doc = "1: Half 5/6-bit, half FRM"]
    HALF_BIT_56_FRM = 1,
    #[doc = "2: Half 8-bit, half FRM"]
    HALF_BIT_8_FRM = 2,
    #[doc = "3: Half 8-bit, half 5/6-bit"]
    HALF_BIT_8_56 = 3,
}
impl From<LCD_FRM_TEST_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_FRM_TEST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LCD_FRM_TEST_A {
    type Ux = u8;
}
impl LCD_FRM_TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FRM_TEST_A {
        match self.bits {
            0 => LCD_FRM_TEST_A::FRM,
            1 => LCD_FRM_TEST_A::HALF_BIT_56_FRM,
            2 => LCD_FRM_TEST_A::HALF_BIT_8_FRM,
            3 => LCD_FRM_TEST_A::HALF_BIT_8_56,
            _ => unreachable!(),
        }
    }
    #[doc = "FRM"]
    #[inline(always)]
    pub fn is_frm(&self) -> bool {
        *self == LCD_FRM_TEST_A::FRM
    }
    #[doc = "Half 5/6-bit, half FRM"]
    #[inline(always)]
    pub fn is_half_bit_56_frm(&self) -> bool {
        *self == LCD_FRM_TEST_A::HALF_BIT_56_FRM
    }
    #[doc = "Half 8-bit, half FRM"]
    #[inline(always)]
    pub fn is_half_bit_8_frm(&self) -> bool {
        *self == LCD_FRM_TEST_A::HALF_BIT_8_FRM
    }
    #[doc = "Half 8-bit, half 5/6-bit"]
    #[inline(always)]
    pub fn is_half_bit_8_56(&self) -> bool {
        *self == LCD_FRM_TEST_A::HALF_BIT_8_56
    }
}
#[doc = "Field `lcd_frm_test` writer - Set the test mode of dither function"]
pub type LCD_FRM_TEST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LCD_FRM_TEST_A>;
impl<'a, REG> LCD_FRM_TEST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FRM"]
    #[inline(always)]
    pub fn frm(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_TEST_A::FRM)
    }
    #[doc = "Half 5/6-bit, half FRM"]
    #[inline(always)]
    pub fn half_bit_56_frm(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_TEST_A::HALF_BIT_56_FRM)
    }
    #[doc = "Half 8-bit, half FRM"]
    #[inline(always)]
    pub fn half_bit_8_frm(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_TEST_A::HALF_BIT_8_FRM)
    }
    #[doc = "Half 8-bit, half 5/6-bit"]
    #[inline(always)]
    pub fn half_bit_8_56(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_TEST_A::HALF_BIT_8_56)
    }
}
#[doc = "Field `lcd_frm_mode_b` reader - The B component output bits in dither function"]
pub type LCD_FRM_MODE_B_R = crate::BitReader<LCD_FRM_MODE_B_A>;
#[doc = "The B component output bits in dither function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_FRM_MODE_B_A {
    #[doc = "0: 6-bit frm output"]
    B_IT6 = 0,
    #[doc = "1: 5-bit frm output"]
    B_IT5 = 1,
}
impl From<LCD_FRM_MODE_B_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_FRM_MODE_B_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_FRM_MODE_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FRM_MODE_B_A {
        match self.bits {
            false => LCD_FRM_MODE_B_A::B_IT6,
            true => LCD_FRM_MODE_B_A::B_IT5,
        }
    }
    #[doc = "6-bit frm output"]
    #[inline(always)]
    pub fn is_b_it6(&self) -> bool {
        *self == LCD_FRM_MODE_B_A::B_IT6
    }
    #[doc = "5-bit frm output"]
    #[inline(always)]
    pub fn is_b_it5(&self) -> bool {
        *self == LCD_FRM_MODE_B_A::B_IT5
    }
}
#[doc = "Field `lcd_frm_mode_b` writer - The B component output bits in dither function"]
pub type LCD_FRM_MODE_B_W<'a, REG> = crate::BitWriter<'a, REG, LCD_FRM_MODE_B_A>;
impl<'a, REG> LCD_FRM_MODE_B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "6-bit frm output"]
    #[inline(always)]
    pub fn b_it6(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_MODE_B_A::B_IT6)
    }
    #[doc = "5-bit frm output"]
    #[inline(always)]
    pub fn b_it5(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_MODE_B_A::B_IT5)
    }
}
#[doc = "Field `lcd_frm_mode_g` reader - The G component output bits in dither function"]
pub type LCD_FRM_MODE_G_R = crate::BitReader<LCD_FRM_MODE_G_A>;
#[doc = "The G component output bits in dither function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_FRM_MODE_G_A {
    #[doc = "0: 6-bit frm output"]
    B_IT6 = 0,
    #[doc = "1: 5-bit frm output"]
    B_IT5 = 1,
}
impl From<LCD_FRM_MODE_G_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_FRM_MODE_G_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_FRM_MODE_G_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FRM_MODE_G_A {
        match self.bits {
            false => LCD_FRM_MODE_G_A::B_IT6,
            true => LCD_FRM_MODE_G_A::B_IT5,
        }
    }
    #[doc = "6-bit frm output"]
    #[inline(always)]
    pub fn is_b_it6(&self) -> bool {
        *self == LCD_FRM_MODE_G_A::B_IT6
    }
    #[doc = "5-bit frm output"]
    #[inline(always)]
    pub fn is_b_it5(&self) -> bool {
        *self == LCD_FRM_MODE_G_A::B_IT5
    }
}
#[doc = "Field `lcd_frm_mode_g` writer - The G component output bits in dither function"]
pub type LCD_FRM_MODE_G_W<'a, REG> = crate::BitWriter<'a, REG, LCD_FRM_MODE_G_A>;
impl<'a, REG> LCD_FRM_MODE_G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "6-bit frm output"]
    #[inline(always)]
    pub fn b_it6(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_MODE_G_A::B_IT6)
    }
    #[doc = "5-bit frm output"]
    #[inline(always)]
    pub fn b_it5(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_MODE_G_A::B_IT5)
    }
}
#[doc = "Field `lcd_frm_mode_r` reader - The R component output bits in dither function"]
pub type LCD_FRM_MODE_R_R = crate::BitReader<LCD_FRM_MODE_R_A>;
#[doc = "The R component output bits in dither function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_FRM_MODE_R_A {
    #[doc = "0: 6-bit frm output"]
    B_IT6 = 0,
    #[doc = "1: 5-bit frm output"]
    B_IT5 = 1,
}
impl From<LCD_FRM_MODE_R_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_FRM_MODE_R_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_FRM_MODE_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FRM_MODE_R_A {
        match self.bits {
            false => LCD_FRM_MODE_R_A::B_IT6,
            true => LCD_FRM_MODE_R_A::B_IT5,
        }
    }
    #[doc = "6-bit frm output"]
    #[inline(always)]
    pub fn is_b_it6(&self) -> bool {
        *self == LCD_FRM_MODE_R_A::B_IT6
    }
    #[doc = "5-bit frm output"]
    #[inline(always)]
    pub fn is_b_it5(&self) -> bool {
        *self == LCD_FRM_MODE_R_A::B_IT5
    }
}
#[doc = "Field `lcd_frm_mode_r` writer - The R component output bits in dither function"]
pub type LCD_FRM_MODE_R_W<'a, REG> = crate::BitWriter<'a, REG, LCD_FRM_MODE_R_A>;
impl<'a, REG> LCD_FRM_MODE_R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "6-bit frm output"]
    #[inline(always)]
    pub fn b_it6(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_MODE_R_A::B_IT6)
    }
    #[doc = "5-bit frm output"]
    #[inline(always)]
    pub fn b_it5(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_MODE_R_A::B_IT5)
    }
}
#[doc = "Field `lcd_frm_en` reader - Enable the dither function"]
pub type LCD_FRM_EN_R = crate::BitReader<LCD_FRM_EN_A>;
#[doc = "Enable the dither function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_FRM_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_FRM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_FRM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_FRM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FRM_EN_A {
        match self.bits {
            false => LCD_FRM_EN_A::DISABLE,
            true => LCD_FRM_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_FRM_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_FRM_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_frm_en` writer - Enable the dither function"]
pub type LCD_FRM_EN_W<'a, REG> = crate::BitWriter<'a, REG, LCD_FRM_EN_A>;
impl<'a, REG> LCD_FRM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LCD_FRM_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set the test mode of dither function"]
    #[inline(always)]
    pub fn lcd_frm_test(&self) -> LCD_FRM_TEST_R {
        LCD_FRM_TEST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - The B component output bits in dither function"]
    #[inline(always)]
    pub fn lcd_frm_mode_b(&self) -> LCD_FRM_MODE_B_R {
        LCD_FRM_MODE_B_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The G component output bits in dither function"]
    #[inline(always)]
    pub fn lcd_frm_mode_g(&self) -> LCD_FRM_MODE_G_R {
        LCD_FRM_MODE_G_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The R component output bits in dither function"]
    #[inline(always)]
    pub fn lcd_frm_mode_r(&self) -> LCD_FRM_MODE_R_R {
        LCD_FRM_MODE_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the dither function"]
    #[inline(always)]
    pub fn lcd_frm_en(&self) -> LCD_FRM_EN_R {
        LCD_FRM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set the test mode of dither function"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_frm_test(&mut self) -> LCD_FRM_TEST_W<LCD_FRM_CTL_SPEC> {
        LCD_FRM_TEST_W::new(self, 0)
    }
    #[doc = "Bit 4 - The B component output bits in dither function"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_frm_mode_b(&mut self) -> LCD_FRM_MODE_B_W<LCD_FRM_CTL_SPEC> {
        LCD_FRM_MODE_B_W::new(self, 4)
    }
    #[doc = "Bit 5 - The G component output bits in dither function"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_frm_mode_g(&mut self) -> LCD_FRM_MODE_G_W<LCD_FRM_CTL_SPEC> {
        LCD_FRM_MODE_G_W::new(self, 5)
    }
    #[doc = "Bit 6 - The R component output bits in dither function"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_frm_mode_r(&mut self) -> LCD_FRM_MODE_R_W<LCD_FRM_CTL_SPEC> {
        LCD_FRM_MODE_R_W::new(self, 6)
    }
    #[doc = "Bit 31 - Enable the dither function"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_frm_en(&mut self) -> LCD_FRM_EN_W<LCD_FRM_CTL_SPEC> {
        LCD_FRM_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LCD FRM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_frm_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_frm_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_FRM_CTL_SPEC;
impl crate::RegisterSpec for LCD_FRM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_frm_ctl::R`](R) reader structure"]
impl crate::Readable for LCD_FRM_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_frm_ctl::W`](W) writer structure"]
impl crate::Writable for LCD_FRM_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_frm_ctl to value 0"]
impl crate::Resettable for LCD_FRM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
