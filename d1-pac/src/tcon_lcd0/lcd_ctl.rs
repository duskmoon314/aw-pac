#[doc = "Register `lcd_ctl` reader"]
pub struct R(crate::R<LCD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_ctl` writer"]
pub struct W(crate::W<LCD_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CTL_SPEC>;
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
impl From<crate::W<LCD_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lcd_src_sel` reader - LCD Source Select"]
pub type LCD_SRC_SEL_R = crate::FieldReader<u8, LCD_SRC_SEL_A>;
#[doc = "LCD Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCD_SRC_SEL_A {
    #[doc = "0: DE"]
    DE = 0,
    #[doc = "1: Color Check"]
    COLOR_CHECK = 1,
    #[doc = "2: Grayscale Check"]
    GRAYSCALE_CHECK = 2,
    #[doc = "3: Black by White Check"]
    BLACK_BY_WHITE_CHECK = 3,
    #[doc = "4: Test Data all 0"]
    TEST_DATA_ALL_0 = 4,
    #[doc = "5: Test Data all 1"]
    TEST_DATA_ALL_1 = 5,
    #[doc = "6: Reversed"]
    REVERSED = 6,
    #[doc = "7: Gridding Check"]
    GRIDDING_CHECK = 7,
}
impl From<LCD_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl LCD_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_SRC_SEL_A {
        match self.bits {
            0 => LCD_SRC_SEL_A::DE,
            1 => LCD_SRC_SEL_A::COLOR_CHECK,
            2 => LCD_SRC_SEL_A::GRAYSCALE_CHECK,
            3 => LCD_SRC_SEL_A::BLACK_BY_WHITE_CHECK,
            4 => LCD_SRC_SEL_A::TEST_DATA_ALL_0,
            5 => LCD_SRC_SEL_A::TEST_DATA_ALL_1,
            6 => LCD_SRC_SEL_A::REVERSED,
            7 => LCD_SRC_SEL_A::GRIDDING_CHECK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DE`"]
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == LCD_SRC_SEL_A::DE
    }
    #[doc = "Checks if the value of the field is `COLOR_CHECK`"]
    #[inline(always)]
    pub fn is_color_check(&self) -> bool {
        *self == LCD_SRC_SEL_A::COLOR_CHECK
    }
    #[doc = "Checks if the value of the field is `GRAYSCALE_CHECK`"]
    #[inline(always)]
    pub fn is_grayscale_check(&self) -> bool {
        *self == LCD_SRC_SEL_A::GRAYSCALE_CHECK
    }
    #[doc = "Checks if the value of the field is `BLACK_BY_WHITE_CHECK`"]
    #[inline(always)]
    pub fn is_black_by_white_check(&self) -> bool {
        *self == LCD_SRC_SEL_A::BLACK_BY_WHITE_CHECK
    }
    #[doc = "Checks if the value of the field is `TEST_DATA_ALL_0`"]
    #[inline(always)]
    pub fn is_test_data_all_0(&self) -> bool {
        *self == LCD_SRC_SEL_A::TEST_DATA_ALL_0
    }
    #[doc = "Checks if the value of the field is `TEST_DATA_ALL_1`"]
    #[inline(always)]
    pub fn is_test_data_all_1(&self) -> bool {
        *self == LCD_SRC_SEL_A::TEST_DATA_ALL_1
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == LCD_SRC_SEL_A::REVERSED
    }
    #[doc = "Checks if the value of the field is `GRIDDING_CHECK`"]
    #[inline(always)]
    pub fn is_gridding_check(&self) -> bool {
        *self == LCD_SRC_SEL_A::GRIDDING_CHECK
    }
}
#[doc = "Field `lcd_src_sel` writer - LCD Source Select"]
pub type LCD_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCD_CTL_SPEC, u8, LCD_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> LCD_SRC_SEL_W<'a, O> {
    #[doc = "DE"]
    #[inline(always)]
    pub fn de(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::DE)
    }
    #[doc = "Color Check"]
    #[inline(always)]
    pub fn color_check(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::COLOR_CHECK)
    }
    #[doc = "Grayscale Check"]
    #[inline(always)]
    pub fn grayscale_check(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::GRAYSCALE_CHECK)
    }
    #[doc = "Black by White Check"]
    #[inline(always)]
    pub fn black_by_white_check(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::BLACK_BY_WHITE_CHECK)
    }
    #[doc = "Test Data all 0"]
    #[inline(always)]
    pub fn test_data_all_0(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::TEST_DATA_ALL_0)
    }
    #[doc = "Test Data all 1"]
    #[inline(always)]
    pub fn test_data_all_1(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::TEST_DATA_ALL_1)
    }
    #[doc = "Reversed"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::REVERSED)
    }
    #[doc = "Gridding Check"]
    #[inline(always)]
    pub fn gridding_check(self) -> &'a mut W {
        self.variant(LCD_SRC_SEL_A::GRIDDING_CHECK)
    }
}
#[doc = "Field `lcd_start_dly` reader - The unit of delay is T_line.\n\nNote: Valid only when LCD_EN == 1"]
pub type LCD_START_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lcd_start_dly` writer - The unit of delay is T_line.\n\nNote: Valid only when LCD_EN == 1"]
pub type LCD_START_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `lcd_interlace_en` reader - This flag is valid only when LCD_EN == 1"]
pub type LCD_INTERLACE_EN_R = crate::BitReader<LCD_INTERLACE_EN_A>;
#[doc = "This flag is valid only when LCD_EN == 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_INTERLACE_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_INTERLACE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_INTERLACE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_INTERLACE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_INTERLACE_EN_A {
        match self.bits {
            false => LCD_INTERLACE_EN_A::DISABLE,
            true => LCD_INTERLACE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_INTERLACE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_INTERLACE_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_interlace_en` writer - This flag is valid only when LCD_EN == 1"]
pub type LCD_INTERLACE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_CTL_SPEC, LCD_INTERLACE_EN_A, O>;
impl<'a, const O: u8> LCD_INTERLACE_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_INTERLACE_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_INTERLACE_EN_A::ENABLE)
    }
}
#[doc = "Field `lcd_fifo1_rst` reader - Writing 1 and then 0 to this bit will reset FIFO 1\n\nNote: 1 holding time must more than 1 DCLK"]
pub type LCD_FIFO1_RST_R = crate::BitReader<bool>;
#[doc = "Field `lcd_fifo1_rst` writer - Writing 1 and then 0 to this bit will reset FIFO 1\n\nNote: 1 holding time must more than 1 DCLK"]
pub type LCD_FIFO1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_CTL_SPEC, bool, O>;
#[doc = "Field `lcd_rb_swap` reader - Enable the function to swap red data and blue data in fifo1."]
pub type LCD_RB_SWAP_R = crate::BitReader<LCD_RB_SWAP_A>;
#[doc = "Enable the function to swap red data and blue data in fifo1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_RB_SWAP_A {
    #[doc = "0: Default"]
    D_EFAULT = 0,
    #[doc = "1: Swap RED and BLUE data at FIFO1"]
    S_WAP = 1,
}
impl From<LCD_RB_SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_RB_SWAP_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_RB_SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_RB_SWAP_A {
        match self.bits {
            false => LCD_RB_SWAP_A::D_EFAULT,
            true => LCD_RB_SWAP_A::S_WAP,
        }
    }
    #[doc = "Checks if the value of the field is `D_EFAULT`"]
    #[inline(always)]
    pub fn is_d_efault(&self) -> bool {
        *self == LCD_RB_SWAP_A::D_EFAULT
    }
    #[doc = "Checks if the value of the field is `S_WAP`"]
    #[inline(always)]
    pub fn is_s_wap(&self) -> bool {
        *self == LCD_RB_SWAP_A::S_WAP
    }
}
#[doc = "Field `lcd_rb_swap` writer - Enable the function to swap red data and blue data in fifo1."]
pub type LCD_RB_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_CTL_SPEC, LCD_RB_SWAP_A, O>;
impl<'a, const O: u8> LCD_RB_SWAP_W<'a, O> {
    #[doc = "Default"]
    #[inline(always)]
    pub fn d_efault(self) -> &'a mut W {
        self.variant(LCD_RB_SWAP_A::D_EFAULT)
    }
    #[doc = "Swap RED and BLUE data at FIFO1"]
    #[inline(always)]
    pub fn s_wap(self) -> &'a mut W {
        self.variant(LCD_RB_SWAP_A::S_WAP)
    }
}
#[doc = "Field `lcd_if` reader - Set the interface type of LCD controller"]
pub type LCD_IF_R = crate::FieldReader<u8, LCD_IF_A>;
#[doc = "Set the interface type of LCD controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCD_IF_A {
    #[doc = "0: HV (Sync + DE)"]
    HV = 0,
    #[doc = "1: 8080 I/F"]
    IF_8080 = 1,
}
impl From<LCD_IF_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_IF_A) -> Self {
        variant as _
    }
}
impl LCD_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCD_IF_A> {
        match self.bits {
            0 => Some(LCD_IF_A::HV),
            1 => Some(LCD_IF_A::IF_8080),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HV`"]
    #[inline(always)]
    pub fn is_hv(&self) -> bool {
        *self == LCD_IF_A::HV
    }
    #[doc = "Checks if the value of the field is `IF_8080`"]
    #[inline(always)]
    pub fn is_if_8080(&self) -> bool {
        *self == LCD_IF_A::IF_8080
    }
}
#[doc = "Field `lcd_if` writer - Set the interface type of LCD controller"]
pub type LCD_IF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_CTL_SPEC, u8, LCD_IF_A, 2, O>;
impl<'a, const O: u8> LCD_IF_W<'a, O> {
    #[doc = "HV (Sync + DE)"]
    #[inline(always)]
    pub fn hv(self) -> &'a mut W {
        self.variant(LCD_IF_A::HV)
    }
    #[doc = "8080 I/F"]
    #[inline(always)]
    pub fn if_8080(self) -> &'a mut W {
        self.variant(LCD_IF_A::IF_8080)
    }
}
#[doc = "Field `lcd_en` reader - It executes at the beginning of the first blank line of LCD timing."]
pub type LCD_EN_R = crate::BitReader<LCD_EN_A>;
#[doc = "It executes at the beginning of the first blank line of LCD timing.\n\nValue on reset: 0"]
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
#[doc = "Field `lcd_en` writer - It executes at the beginning of the first blank line of LCD timing."]
pub type LCD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_CTL_SPEC, LCD_EN_A, O>;
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
    #[doc = "Bits 0:2 - LCD Source Select"]
    #[inline(always)]
    pub fn lcd_src_sel(&self) -> LCD_SRC_SEL_R {
        LCD_SRC_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:8 - The unit of delay is T_line.\n\nNote: Valid only when LCD_EN == 1"]
    #[inline(always)]
    pub fn lcd_start_dly(&self) -> LCD_START_DLY_R {
        LCD_START_DLY_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - This flag is valid only when LCD_EN == 1"]
    #[inline(always)]
    pub fn lcd_interlace_en(&self) -> LCD_INTERLACE_EN_R {
        LCD_INTERLACE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Writing 1 and then 0 to this bit will reset FIFO 1\n\nNote: 1 holding time must more than 1 DCLK"]
    #[inline(always)]
    pub fn lcd_fifo1_rst(&self) -> LCD_FIFO1_RST_R {
        LCD_FIFO1_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable the function to swap red data and blue data in fifo1."]
    #[inline(always)]
    pub fn lcd_rb_swap(&self) -> LCD_RB_SWAP_R {
        LCD_RB_SWAP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Set the interface type of LCD controller"]
    #[inline(always)]
    pub fn lcd_if(&self) -> LCD_IF_R {
        LCD_IF_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - It executes at the beginning of the first blank line of LCD timing."]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LCD Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_src_sel(&mut self) -> LCD_SRC_SEL_W<0> {
        LCD_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 4:8 - The unit of delay is T_line.\n\nNote: Valid only when LCD_EN == 1"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_start_dly(&mut self) -> LCD_START_DLY_W<4> {
        LCD_START_DLY_W::new(self)
    }
    #[doc = "Bit 20 - This flag is valid only when LCD_EN == 1"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_interlace_en(&mut self) -> LCD_INTERLACE_EN_W<20> {
        LCD_INTERLACE_EN_W::new(self)
    }
    #[doc = "Bit 21 - Writing 1 and then 0 to this bit will reset FIFO 1\n\nNote: 1 holding time must more than 1 DCLK"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_fifo1_rst(&mut self) -> LCD_FIFO1_RST_W<21> {
        LCD_FIFO1_RST_W::new(self)
    }
    #[doc = "Bit 23 - Enable the function to swap red data and blue data in fifo1."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_rb_swap(&mut self) -> LCD_RB_SWAP_W<23> {
        LCD_RB_SWAP_W::new(self)
    }
    #[doc = "Bits 24:25 - Set the interface type of LCD controller"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_if(&mut self) -> LCD_IF_W<24> {
        LCD_IF_W::new(self)
    }
    #[doc = "Bit 31 - It executes at the beginning of the first blank line of LCD timing."]
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
#[doc = "LCD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_ctl](index.html) module"]
pub struct LCD_CTL_SPEC;
impl crate::RegisterSpec for LCD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_ctl::R](R) reader structure"]
impl crate::Readable for LCD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_ctl::W](W) writer structure"]
impl crate::Writable for LCD_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_ctl to value 0"]
impl crate::Resettable for LCD_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
