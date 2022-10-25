#[doc = "Register `lcd_lvds_if` reader"]
pub struct R(crate::R<LCD_LVDS_IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LVDS_IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LVDS_IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LVDS_IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_lvds_if` writer"]
pub struct W(crate::W<LCD_LVDS_IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LVDS_IF_SPEC>;
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
impl From<crate::W<LCD_LVDS_IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LVDS_IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lcd_lvds_data_pol` reader - Set the data polarity of LVDS"]
pub type LCD_LVDS_DATA_POL_R = crate::FieldReader<u8, LCD_LVDS_DATA_POL_A>;
#[doc = "Set the data polarity of LVDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCD_LVDS_DATA_POL_A {
    #[doc = "0: Reverse"]
    REVERSE = 0,
    #[doc = "1: Normal"]
    NORMAL = 1,
}
impl From<LCD_LVDS_DATA_POL_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_LVDS_DATA_POL_A) -> Self {
        variant as _
    }
}
impl LCD_LVDS_DATA_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCD_LVDS_DATA_POL_A> {
        match self.bits {
            0 => Some(LCD_LVDS_DATA_POL_A::REVERSE),
            1 => Some(LCD_LVDS_DATA_POL_A::NORMAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LCD_LVDS_DATA_POL_A::REVERSE
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LCD_LVDS_DATA_POL_A::NORMAL
    }
}
#[doc = "Field `lcd_lvds_data_pol` writer - Set the data polarity of LVDS"]
pub type LCD_LVDS_DATA_POL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_LVDS_IF_SPEC, u8, LCD_LVDS_DATA_POL_A, 4, O>;
impl<'a, const O: u8> LCD_LVDS_DATA_POL_W<'a, O> {
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut W {
        self.variant(LCD_LVDS_DATA_POL_A::REVERSE)
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LCD_LVDS_DATA_POL_A::NORMAL)
    }
}
#[doc = "Field `lcd_lvds_clk_pol` reader - Set the clock polarity of LVDS"]
pub type LCD_LVDS_CLK_POL_R = crate::BitReader<LCD_LVDS_CLK_POL_A>;
#[doc = "Set the clock polarity of LVDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_CLK_POL_A {
    #[doc = "0: Reverse"]
    REVERSE = 0,
    #[doc = "1: Normal"]
    NORMAL = 1,
}
impl From<LCD_LVDS_CLK_POL_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_CLK_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_CLK_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_CLK_POL_A {
        match self.bits {
            false => LCD_LVDS_CLK_POL_A::REVERSE,
            true => LCD_LVDS_CLK_POL_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LCD_LVDS_CLK_POL_A::REVERSE
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LCD_LVDS_CLK_POL_A::NORMAL
    }
}
#[doc = "Field `lcd_lvds_clk_pol` writer - Set the clock polarity of LVDS"]
pub type LCD_LVDS_CLK_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_CLK_POL_A, O>;
impl<'a, const O: u8> LCD_LVDS_CLK_POL_W<'a, O> {
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut W {
        self.variant(LCD_LVDS_CLK_POL_A::REVERSE)
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LCD_LVDS_CLK_POL_A::NORMAL)
    }
}
#[doc = "Field `lcd_lvds_clk_sel` reader - Select the clock source of LVDS"]
pub type LCD_LVDS_CLK_SEL_R = crate::BitReader<LCD_LVDS_CLK_SEL_A>;
#[doc = "Select the clock source of LVDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_CLK_SEL_A {
    #[doc = "1: LCD CLK"]
    LCD = 1,
}
impl From<LCD_LVDS_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCD_LVDS_CLK_SEL_A> {
        match self.bits {
            true => Some(LCD_LVDS_CLK_SEL_A::LCD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        *self == LCD_LVDS_CLK_SEL_A::LCD
    }
}
#[doc = "Field `lcd_lvds_clk_sel` writer - Select the clock source of LVDS"]
pub type LCD_LVDS_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_CLK_SEL_A, O>;
impl<'a, const O: u8> LCD_LVDS_CLK_SEL_W<'a, O> {
    #[doc = "LCD CLK"]
    #[inline(always)]
    pub fn lcd(self) -> &'a mut W {
        self.variant(LCD_LVDS_CLK_SEL_A::LCD)
    }
}
#[doc = "Field `lcd_lvds_correct_mode` reader - Set the LVDS correct mode"]
pub type LCD_LVDS_CORRECT_MODE_R = crate::BitReader<LCD_LVDS_CORRECT_MODE_A>;
#[doc = "Set the LVDS correct mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_CORRECT_MODE_A {
    #[doc = "0: Mode0"]
    M_ODE0 = 0,
    #[doc = "1: Mode1"]
    M_ODE1 = 1,
}
impl From<LCD_LVDS_CORRECT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_CORRECT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_CORRECT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_CORRECT_MODE_A {
        match self.bits {
            false => LCD_LVDS_CORRECT_MODE_A::M_ODE0,
            true => LCD_LVDS_CORRECT_MODE_A::M_ODE1,
        }
    }
    #[doc = "Checks if the value of the field is `M_ODE0`"]
    #[inline(always)]
    pub fn is_m_ode0(&self) -> bool {
        *self == LCD_LVDS_CORRECT_MODE_A::M_ODE0
    }
    #[doc = "Checks if the value of the field is `M_ODE1`"]
    #[inline(always)]
    pub fn is_m_ode1(&self) -> bool {
        *self == LCD_LVDS_CORRECT_MODE_A::M_ODE1
    }
}
#[doc = "Field `lcd_lvds_correct_mode` writer - Set the LVDS correct mode"]
pub type LCD_LVDS_CORRECT_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_CORRECT_MODE_A, O>;
impl<'a, const O: u8> LCD_LVDS_CORRECT_MODE_W<'a, O> {
    #[doc = "Mode0"]
    #[inline(always)]
    pub fn m_ode0(self) -> &'a mut W {
        self.variant(LCD_LVDS_CORRECT_MODE_A::M_ODE0)
    }
    #[doc = "Mode1"]
    #[inline(always)]
    pub fn m_ode1(self) -> &'a mut W {
        self.variant(LCD_LVDS_CORRECT_MODE_A::M_ODE1)
    }
}
#[doc = "Field `lcd_lvds_debug_mode` reader - Set the output signal in debug mode"]
pub type LCD_LVDS_DEBUG_MODE_R = crate::BitReader<LCD_LVDS_DEBUG_MODE_A>;
#[doc = "Set the output signal in debug mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_DEBUG_MODE_A {
    #[doc = "0: Mode0--Random data"]
    M_ODE0 = 0,
    #[doc = "1: Mode1--Output CLK period=7/2 LVDS CLK period"]
    M_ODE1_O_UTPUT = 1,
}
impl From<LCD_LVDS_DEBUG_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_DEBUG_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_DEBUG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_DEBUG_MODE_A {
        match self.bits {
            false => LCD_LVDS_DEBUG_MODE_A::M_ODE0,
            true => LCD_LVDS_DEBUG_MODE_A::M_ODE1_O_UTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `M_ODE0`"]
    #[inline(always)]
    pub fn is_m_ode0(&self) -> bool {
        *self == LCD_LVDS_DEBUG_MODE_A::M_ODE0
    }
    #[doc = "Checks if the value of the field is `M_ODE1_O_UTPUT`"]
    #[inline(always)]
    pub fn is_m_ode1_o_utput(&self) -> bool {
        *self == LCD_LVDS_DEBUG_MODE_A::M_ODE1_O_UTPUT
    }
}
#[doc = "Field `lcd_lvds_debug_mode` writer - Set the output signal in debug mode"]
pub type LCD_LVDS_DEBUG_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_DEBUG_MODE_A, O>;
impl<'a, const O: u8> LCD_LVDS_DEBUG_MODE_W<'a, O> {
    #[doc = "Mode0--Random data"]
    #[inline(always)]
    pub fn m_ode0(self) -> &'a mut W {
        self.variant(LCD_LVDS_DEBUG_MODE_A::M_ODE0)
    }
    #[doc = "Mode1--Output CLK period=7/2 LVDS CLK period"]
    #[inline(always)]
    pub fn m_ode1_o_utput(self) -> &'a mut W {
        self.variant(LCD_LVDS_DEBUG_MODE_A::M_ODE1_O_UTPUT)
    }
}
#[doc = "Field `lcd_lvds_debug_en` reader - Enable LVDS debug function"]
pub type LCD_LVDS_DEBUG_EN_R = crate::BitReader<LCD_LVDS_DEBUG_EN_A>;
#[doc = "Enable LVDS debug function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_DEBUG_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_LVDS_DEBUG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_DEBUG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_DEBUG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_DEBUG_EN_A {
        match self.bits {
            false => LCD_LVDS_DEBUG_EN_A::DISABLE,
            true => LCD_LVDS_DEBUG_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_LVDS_DEBUG_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_LVDS_DEBUG_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_lvds_debug_en` writer - Enable LVDS debug function"]
pub type LCD_LVDS_DEBUG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_DEBUG_EN_A, O>;
impl<'a, const O: u8> LCD_LVDS_DEBUG_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_LVDS_DEBUG_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_LVDS_DEBUG_EN_A::ENABLE)
    }
}
#[doc = "Field `lcd_lvds_bitwidth` reader - Set the bit width of data"]
pub type LCD_LVDS_BITWIDTH_R = crate::BitReader<LCD_LVDS_BITWIDTH_A>;
#[doc = "Set the bit width of data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_BITWIDTH_A {
    #[doc = "0: 24-bit"]
    _24_BIT = 0,
    #[doc = "1: 18-bit"]
    _18_BIT = 1,
}
impl From<LCD_LVDS_BITWIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_BITWIDTH_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_BITWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_BITWIDTH_A {
        match self.bits {
            false => LCD_LVDS_BITWIDTH_A::_24_BIT,
            true => LCD_LVDS_BITWIDTH_A::_18_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == LCD_LVDS_BITWIDTH_A::_24_BIT
    }
    #[doc = "Checks if the value of the field is `_18_BIT`"]
    #[inline(always)]
    pub fn is_18_bit(&self) -> bool {
        *self == LCD_LVDS_BITWIDTH_A::_18_BIT
    }
}
#[doc = "Field `lcd_lvds_bitwidth` writer - Set the bit width of data"]
pub type LCD_LVDS_BITWIDTH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_BITWIDTH_A, O>;
impl<'a, const O: u8> LCD_LVDS_BITWIDTH_W<'a, O> {
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(LCD_LVDS_BITWIDTH_A::_24_BIT)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn _18_bit(self) -> &'a mut W {
        self.variant(LCD_LVDS_BITWIDTH_A::_18_BIT)
    }
}
#[doc = "Field `lcd_lvds_mode` reader - Set the LVDS data mode"]
pub type LCD_LVDS_MODE_R = crate::BitReader<LCD_LVDS_MODE_A>;
#[doc = "Set the LVDS data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_MODE_A {
    #[doc = "0: NS mode"]
    NS_MODE = 0,
    #[doc = "1: JEIDA mode"]
    JEIDA_MODE = 1,
}
impl From<LCD_LVDS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_MODE_A {
        match self.bits {
            false => LCD_LVDS_MODE_A::NS_MODE,
            true => LCD_LVDS_MODE_A::JEIDA_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NS_MODE`"]
    #[inline(always)]
    pub fn is_ns_mode(&self) -> bool {
        *self == LCD_LVDS_MODE_A::NS_MODE
    }
    #[doc = "Checks if the value of the field is `JEIDA_MODE`"]
    #[inline(always)]
    pub fn is_jeida_mode(&self) -> bool {
        *self == LCD_LVDS_MODE_A::JEIDA_MODE
    }
}
#[doc = "Field `lcd_lvds_mode` writer - Set the LVDS data mode"]
pub type LCD_LVDS_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_MODE_A, O>;
impl<'a, const O: u8> LCD_LVDS_MODE_W<'a, O> {
    #[doc = "NS mode"]
    #[inline(always)]
    pub fn ns_mode(self) -> &'a mut W {
        self.variant(LCD_LVDS_MODE_A::NS_MODE)
    }
    #[doc = "JEIDA mode"]
    #[inline(always)]
    pub fn jeida_mode(self) -> &'a mut W {
        self.variant(LCD_LVDS_MODE_A::JEIDA_MODE)
    }
}
#[doc = "Field `lcd_lvds_dir` reader - Set the LVDS direction"]
pub type LCD_LVDS_DIR_R = crate::BitReader<LCD_LVDS_DIR_A>;
#[doc = "Set the LVDS direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_DIR_A {
    #[doc = "0: Normal"]
    NORMAL = 0,
    #[doc = "1: Reverse"]
    REVERSE = 1,
}
impl From<LCD_LVDS_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_DIR_A {
        match self.bits {
            false => LCD_LVDS_DIR_A::NORMAL,
            true => LCD_LVDS_DIR_A::REVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LCD_LVDS_DIR_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LCD_LVDS_DIR_A::REVERSE
    }
}
#[doc = "Field `lcd_lvds_dir` writer - Set the LVDS direction"]
pub type LCD_LVDS_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_DIR_A, O>;
impl<'a, const O: u8> LCD_LVDS_DIR_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LCD_LVDS_DIR_A::NORMAL)
    }
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut W {
        self.variant(LCD_LVDS_DIR_A::REVERSE)
    }
}
#[doc = "Field `lcd_lvds_even_odd_dir` reader - Set the order of even field and odd field"]
pub type LCD_LVDS_EVEN_ODD_DIR_R = crate::BitReader<LCD_LVDS_EVEN_ODD_DIR_A>;
#[doc = "Set the order of even field and odd field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_EVEN_ODD_DIR_A {
    #[doc = "0: normal"]
    NORMAL = 0,
    #[doc = "1: reverse"]
    REVERSE = 1,
}
impl From<LCD_LVDS_EVEN_ODD_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_EVEN_ODD_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_EVEN_ODD_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_EVEN_ODD_DIR_A {
        match self.bits {
            false => LCD_LVDS_EVEN_ODD_DIR_A::NORMAL,
            true => LCD_LVDS_EVEN_ODD_DIR_A::REVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LCD_LVDS_EVEN_ODD_DIR_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LCD_LVDS_EVEN_ODD_DIR_A::REVERSE
    }
}
#[doc = "Field `lcd_lvds_even_odd_dir` writer - Set the order of even field and odd field"]
pub type LCD_LVDS_EVEN_ODD_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_EVEN_ODD_DIR_A, O>;
impl<'a, const O: u8> LCD_LVDS_EVEN_ODD_DIR_W<'a, O> {
    #[doc = "normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LCD_LVDS_EVEN_ODD_DIR_A::NORMAL)
    }
    #[doc = "reverse"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut W {
        self.variant(LCD_LVDS_EVEN_ODD_DIR_A::REVERSE)
    }
}
#[doc = "Field `lcd_lvds_link` reader - Select work in single link mode or dual link mode"]
pub type LCD_LVDS_LINK_R = crate::BitReader<LCD_LVDS_LINK_A>;
#[doc = "Select work in single link mode or dual link mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_LINK_A {
    #[doc = "0: Single link"]
    S_INGLE = 0,
    #[doc = "1: Dual link"]
    D_UAL = 1,
}
impl From<LCD_LVDS_LINK_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_LINK_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_LINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_LINK_A {
        match self.bits {
            false => LCD_LVDS_LINK_A::S_INGLE,
            true => LCD_LVDS_LINK_A::D_UAL,
        }
    }
    #[doc = "Checks if the value of the field is `S_INGLE`"]
    #[inline(always)]
    pub fn is_s_ingle(&self) -> bool {
        *self == LCD_LVDS_LINK_A::S_INGLE
    }
    #[doc = "Checks if the value of the field is `D_UAL`"]
    #[inline(always)]
    pub fn is_d_ual(&self) -> bool {
        *self == LCD_LVDS_LINK_A::D_UAL
    }
}
#[doc = "Field `lcd_lvds_link` writer - Select work in single link mode or dual link mode"]
pub type LCD_LVDS_LINK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_LINK_A, O>;
impl<'a, const O: u8> LCD_LVDS_LINK_W<'a, O> {
    #[doc = "Single link"]
    #[inline(always)]
    pub fn s_ingle(self) -> &'a mut W {
        self.variant(LCD_LVDS_LINK_A::S_INGLE)
    }
    #[doc = "Dual link"]
    #[inline(always)]
    pub fn d_ual(self) -> &'a mut W {
        self.variant(LCD_LVDS_LINK_A::D_UAL)
    }
}
#[doc = "Field `lcd_lvds_en` reader - Enable LVDS interface"]
pub type LCD_LVDS_EN_R = crate::BitReader<LCD_LVDS_EN_A>;
#[doc = "Enable LVDS interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_LVDS_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LCD_LVDS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_LVDS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_LVDS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_LVDS_EN_A {
        match self.bits {
            false => LCD_LVDS_EN_A::DISABLE,
            true => LCD_LVDS_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LCD_LVDS_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LCD_LVDS_EN_A::ENABLE
    }
}
#[doc = "Field `lcd_lvds_en` writer - Enable LVDS interface"]
pub type LCD_LVDS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_IF_SPEC, LCD_LVDS_EN_A, O>;
impl<'a, const O: u8> LCD_LVDS_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCD_LVDS_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCD_LVDS_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Set the data polarity of LVDS"]
    #[inline(always)]
    pub fn lcd_lvds_data_pol(&self) -> LCD_LVDS_DATA_POL_R {
        LCD_LVDS_DATA_POL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Set the clock polarity of LVDS"]
    #[inline(always)]
    pub fn lcd_lvds_clk_pol(&self) -> LCD_LVDS_CLK_POL_R {
        LCD_LVDS_CLK_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 20 - Select the clock source of LVDS"]
    #[inline(always)]
    pub fn lcd_lvds_clk_sel(&self) -> LCD_LVDS_CLK_SEL_R {
        LCD_LVDS_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Set the LVDS correct mode"]
    #[inline(always)]
    pub fn lcd_lvds_correct_mode(&self) -> LCD_LVDS_CORRECT_MODE_R {
        LCD_LVDS_CORRECT_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set the output signal in debug mode"]
    #[inline(always)]
    pub fn lcd_lvds_debug_mode(&self) -> LCD_LVDS_DEBUG_MODE_R {
        LCD_LVDS_DEBUG_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable LVDS debug function"]
    #[inline(always)]
    pub fn lcd_lvds_debug_en(&self) -> LCD_LVDS_DEBUG_EN_R {
        LCD_LVDS_DEBUG_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set the bit width of data"]
    #[inline(always)]
    pub fn lcd_lvds_bitwidth(&self) -> LCD_LVDS_BITWIDTH_R {
        LCD_LVDS_BITWIDTH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set the LVDS data mode"]
    #[inline(always)]
    pub fn lcd_lvds_mode(&self) -> LCD_LVDS_MODE_R {
        LCD_LVDS_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set the LVDS direction"]
    #[inline(always)]
    pub fn lcd_lvds_dir(&self) -> LCD_LVDS_DIR_R {
        LCD_LVDS_DIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set the order of even field and odd field"]
    #[inline(always)]
    pub fn lcd_lvds_even_odd_dir(&self) -> LCD_LVDS_EVEN_ODD_DIR_R {
        LCD_LVDS_EVEN_ODD_DIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Select work in single link mode or dual link mode"]
    #[inline(always)]
    pub fn lcd_lvds_link(&self) -> LCD_LVDS_LINK_R {
        LCD_LVDS_LINK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable LVDS interface"]
    #[inline(always)]
    pub fn lcd_lvds_en(&self) -> LCD_LVDS_EN_R {
        LCD_LVDS_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the data polarity of LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_data_pol(&mut self) -> LCD_LVDS_DATA_POL_W<0> {
        LCD_LVDS_DATA_POL_W::new(self)
    }
    #[doc = "Bit 4 - Set the clock polarity of LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_clk_pol(&mut self) -> LCD_LVDS_CLK_POL_W<4> {
        LCD_LVDS_CLK_POL_W::new(self)
    }
    #[doc = "Bit 20 - Select the clock source of LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_clk_sel(&mut self) -> LCD_LVDS_CLK_SEL_W<20> {
        LCD_LVDS_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 23 - Set the LVDS correct mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_correct_mode(&mut self) -> LCD_LVDS_CORRECT_MODE_W<23> {
        LCD_LVDS_CORRECT_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Set the output signal in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_debug_mode(&mut self) -> LCD_LVDS_DEBUG_MODE_W<24> {
        LCD_LVDS_DEBUG_MODE_W::new(self)
    }
    #[doc = "Bit 25 - Enable LVDS debug function"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_debug_en(&mut self) -> LCD_LVDS_DEBUG_EN_W<25> {
        LCD_LVDS_DEBUG_EN_W::new(self)
    }
    #[doc = "Bit 26 - Set the bit width of data"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_bitwidth(&mut self) -> LCD_LVDS_BITWIDTH_W<26> {
        LCD_LVDS_BITWIDTH_W::new(self)
    }
    #[doc = "Bit 27 - Set the LVDS data mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_mode(&mut self) -> LCD_LVDS_MODE_W<27> {
        LCD_LVDS_MODE_W::new(self)
    }
    #[doc = "Bit 28 - Set the LVDS direction"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_dir(&mut self) -> LCD_LVDS_DIR_W<28> {
        LCD_LVDS_DIR_W::new(self)
    }
    #[doc = "Bit 29 - Set the order of even field and odd field"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_even_odd_dir(&mut self) -> LCD_LVDS_EVEN_ODD_DIR_W<29> {
        LCD_LVDS_EVEN_ODD_DIR_W::new(self)
    }
    #[doc = "Bit 30 - Select work in single link mode or dual link mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_link(&mut self) -> LCD_LVDS_LINK_W<30> {
        LCD_LVDS_LINK_W::new(self)
    }
    #[doc = "Bit 31 - Enable LVDS interface"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_lvds_en(&mut self) -> LCD_LVDS_EN_W<31> {
        LCD_LVDS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD LVDS Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lvds_if](index.html) module"]
pub struct LCD_LVDS_IF_SPEC;
impl crate::RegisterSpec for LCD_LVDS_IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lvds_if::R](R) reader structure"]
impl crate::Readable for LCD_LVDS_IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lvds_if::W](W) writer structure"]
impl crate::Writable for LCD_LVDS_IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_lvds_if to value 0"]
impl crate::Resettable for LCD_LVDS_IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
