#[doc = "Register `lcd_lvds_ana%s` reader"]
pub struct R(crate::R<LCD_LVDS_ANA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LVDS_ANA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LVDS_ANA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LVDS_ANA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_lvds_ana%s` writer"]
pub struct W(crate::W<LCD_LVDS_ANA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LVDS_ANA_SPEC>;
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
impl From<crate::W<LCD_LVDS_ANA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LVDS_ANA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable the bias circuit of the LVDS_Ana module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDS_EN_MB_A {
    #[doc = "0: Disable"]
    D_ISABLE = 0,
    #[doc = "1: Enable"]
    E_NABLE = 1,
}
impl From<LVDS_EN_MB_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_EN_MB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lvds_en_mb` reader - Enable the bias circuit of the LVDS_Ana module."]
pub type LVDS_EN_MB_R = crate::BitReader<LVDS_EN_MB_A>;
impl LVDS_EN_MB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_EN_MB_A {
        match self.bits {
            false => LVDS_EN_MB_A::D_ISABLE,
            true => LVDS_EN_MB_A::E_NABLE,
        }
    }
    #[doc = "Checks if the value of the field is `D_ISABLE`"]
    #[inline(always)]
    pub fn is_d_isable(&self) -> bool {
        *self == LVDS_EN_MB_A::D_ISABLE
    }
    #[doc = "Checks if the value of the field is `E_NABLE`"]
    #[inline(always)]
    pub fn is_e_nable(&self) -> bool {
        *self == LVDS_EN_MB_A::E_NABLE
    }
}
#[doc = "Field `lvds_en_mb` writer - Enable the bias circuit of the LVDS_Ana module."]
pub type LVDS_EN_MB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_ANA_SPEC, LVDS_EN_MB_A, O>;
impl<'a, const O: u8> LVDS_EN_MB_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn d_isable(self) -> &'a mut W {
        self.variant(LVDS_EN_MB_A::D_ISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn e_nable(self) -> &'a mut W {
        self.variant(LVDS_EN_MB_A::E_NABLE)
    }
}
#[doc = "Field `en_lvds` reader - Enable LVDS"]
pub type EN_LVDS_R = crate::BitReader<bool>;
#[doc = "Field `en_lvds` writer - Enable LVDS"]
pub type EN_LVDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_LVDS_ANA_SPEC, bool, O>;
#[doc = "Field `en_24m` reader - Enable the 24M clock"]
pub type EN_24M_R = crate::BitReader<bool>;
#[doc = "Field `en_24m` writer - Enable the 24M clock"]
pub type EN_24M_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_LVDS_ANA_SPEC, bool, O>;
#[doc = "Enable clock channel drive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDS_HPREN_DRVC_A {
    #[doc = "0: Disable"]
    D_ISABLE = 0,
    #[doc = "1: Enable"]
    E_NABLE = 1,
}
impl From<LVDS_HPREN_DRVC_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_HPREN_DRVC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lvds_hpren_drvc` reader - Enable clock channel drive"]
pub type LVDS_HPREN_DRVC_R = crate::BitReader<LVDS_HPREN_DRVC_A>;
impl LVDS_HPREN_DRVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_HPREN_DRVC_A {
        match self.bits {
            false => LVDS_HPREN_DRVC_A::D_ISABLE,
            true => LVDS_HPREN_DRVC_A::E_NABLE,
        }
    }
    #[doc = "Checks if the value of the field is `D_ISABLE`"]
    #[inline(always)]
    pub fn is_d_isable(&self) -> bool {
        *self == LVDS_HPREN_DRVC_A::D_ISABLE
    }
    #[doc = "Checks if the value of the field is `E_NABLE`"]
    #[inline(always)]
    pub fn is_e_nable(&self) -> bool {
        *self == LVDS_HPREN_DRVC_A::E_NABLE
    }
}
#[doc = "Field `lvds_hpren_drvc` writer - Enable clock channel drive"]
pub type LVDS_HPREN_DRVC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_ANA_SPEC, LVDS_HPREN_DRVC_A, O>;
impl<'a, const O: u8> LVDS_HPREN_DRVC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn d_isable(self) -> &'a mut W {
        self.variant(LVDS_HPREN_DRVC_A::D_ISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn e_nable(self) -> &'a mut W {
        self.variant(LVDS_HPREN_DRVC_A::E_NABLE)
    }
}
#[doc = "Field `lvds_hpren_drv` reader - Enable data channel\\[3:0\\]
drive"]
pub type LVDS_HPREN_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lvds_hpren_drv` writer - Enable data channel\\[3:0\\]
drive"]
pub type LVDS_HPREN_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_LVDS_ANA_SPEC, u8, u8, 4, O>;
#[doc = "Adjust current flowing through Rload of Rx to change the differential signals amplitude.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVDS_C_A {
    #[doc = "0: 216 mV"]
    _216_M_V = 0,
    #[doc = "1: 252 mV"]
    _252_M_V = 1,
    #[doc = "2: 276 mV"]
    _276_M_V = 2,
    #[doc = "3: 312 mV"]
    _312_M_V = 3,
    #[doc = "4: 336 mV"]
    _336_M_V = 4,
    #[doc = "5: 372 mV"]
    _372_M_V = 5,
    #[doc = "6: 395 mV"]
    _395_M_V = 6,
    #[doc = "7: 432 mV"]
    _432_M_V = 7,
}
impl From<LVDS_C_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDS_C_A) -> Self {
        variant as _
    }
}
#[doc = "Field `lvds_c` reader - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
pub type LVDS_C_R = crate::FieldReader<u8, LVDS_C_A>;
impl LVDS_C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_C_A {
        match self.bits {
            0 => LVDS_C_A::_216_M_V,
            1 => LVDS_C_A::_252_M_V,
            2 => LVDS_C_A::_276_M_V,
            3 => LVDS_C_A::_312_M_V,
            4 => LVDS_C_A::_336_M_V,
            5 => LVDS_C_A::_372_M_V,
            6 => LVDS_C_A::_395_M_V,
            7 => LVDS_C_A::_432_M_V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_216_M_V`"]
    #[inline(always)]
    pub fn is_216_m_v(&self) -> bool {
        *self == LVDS_C_A::_216_M_V
    }
    #[doc = "Checks if the value of the field is `_252_M_V`"]
    #[inline(always)]
    pub fn is_252_m_v(&self) -> bool {
        *self == LVDS_C_A::_252_M_V
    }
    #[doc = "Checks if the value of the field is `_276_M_V`"]
    #[inline(always)]
    pub fn is_276_m_v(&self) -> bool {
        *self == LVDS_C_A::_276_M_V
    }
    #[doc = "Checks if the value of the field is `_312_M_V`"]
    #[inline(always)]
    pub fn is_312_m_v(&self) -> bool {
        *self == LVDS_C_A::_312_M_V
    }
    #[doc = "Checks if the value of the field is `_336_M_V`"]
    #[inline(always)]
    pub fn is_336_m_v(&self) -> bool {
        *self == LVDS_C_A::_336_M_V
    }
    #[doc = "Checks if the value of the field is `_372_M_V`"]
    #[inline(always)]
    pub fn is_372_m_v(&self) -> bool {
        *self == LVDS_C_A::_372_M_V
    }
    #[doc = "Checks if the value of the field is `_395_M_V`"]
    #[inline(always)]
    pub fn is_395_m_v(&self) -> bool {
        *self == LVDS_C_A::_395_M_V
    }
    #[doc = "Checks if the value of the field is `_432_M_V`"]
    #[inline(always)]
    pub fn is_432_m_v(&self) -> bool {
        *self == LVDS_C_A::_432_M_V
    }
}
#[doc = "Field `lvds_c` writer - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
pub type LVDS_C_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCD_LVDS_ANA_SPEC, u8, LVDS_C_A, 3, O>;
impl<'a, const O: u8> LVDS_C_W<'a, O> {
    #[doc = "216 mV"]
    #[inline(always)]
    pub fn _216_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_216_M_V)
    }
    #[doc = "252 mV"]
    #[inline(always)]
    pub fn _252_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_252_M_V)
    }
    #[doc = "276 mV"]
    #[inline(always)]
    pub fn _276_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_276_M_V)
    }
    #[doc = "312 mV"]
    #[inline(always)]
    pub fn _312_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_312_M_V)
    }
    #[doc = "336 mV"]
    #[inline(always)]
    pub fn _336_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_336_M_V)
    }
    #[doc = "372 mV"]
    #[inline(always)]
    pub fn _372_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_372_M_V)
    }
    #[doc = "395 mV"]
    #[inline(always)]
    pub fn _395_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_395_M_V)
    }
    #[doc = "432 mV"]
    #[inline(always)]
    pub fn _432_m_v(self) -> &'a mut W {
        self.variant(LVDS_C_A::_432_M_V)
    }
}
#[doc = "Field `lvds_denc` reader - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DENC_R = crate::BitReader<bool>;
#[doc = "Field `lvds_denc` writer - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_LVDS_ANA_SPEC, bool, O>;
#[doc = "Field `lvds_den` reader - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lvds_den` writer - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_LVDS_ANA_SPEC, u8, u8, 4, O>;
#[doc = "Adjust current flowing through R of R to change the common signals amplitude.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVDS_R_A {
    #[doc = "0: 0.925 V"]
    _0_925_V = 0,
    #[doc = "1: 0.950 V"]
    _0_950_V = 1,
    #[doc = "2: 0.975 V"]
    _0_975_V = 2,
    #[doc = "3: 1.000 V"]
    _1_000_V = 3,
    #[doc = "4: 1.025 V"]
    _1_025_V = 4,
    #[doc = "5: 1.050 V"]
    _1_050_V = 5,
    #[doc = "6: 1.075 V"]
    _1_075_V = 6,
    #[doc = "7: 1.100 V"]
    _1_100_V = 7,
}
impl From<LVDS_R_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDS_R_A) -> Self {
        variant as _
    }
}
#[doc = "Field `lvds_r` reader - Adjust current flowing through R of R to change the common signals amplitude."]
pub type LVDS_R_R = crate::FieldReader<u8, LVDS_R_A>;
impl LVDS_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_R_A {
        match self.bits {
            0 => LVDS_R_A::_0_925_V,
            1 => LVDS_R_A::_0_950_V,
            2 => LVDS_R_A::_0_975_V,
            3 => LVDS_R_A::_1_000_V,
            4 => LVDS_R_A::_1_025_V,
            5 => LVDS_R_A::_1_050_V,
            6 => LVDS_R_A::_1_075_V,
            7 => LVDS_R_A::_1_100_V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_925_V`"]
    #[inline(always)]
    pub fn is_0_925_v(&self) -> bool {
        *self == LVDS_R_A::_0_925_V
    }
    #[doc = "Checks if the value of the field is `_0_950_V`"]
    #[inline(always)]
    pub fn is_0_950_v(&self) -> bool {
        *self == LVDS_R_A::_0_950_V
    }
    #[doc = "Checks if the value of the field is `_0_975_V`"]
    #[inline(always)]
    pub fn is_0_975_v(&self) -> bool {
        *self == LVDS_R_A::_0_975_V
    }
    #[doc = "Checks if the value of the field is `_1_000_V`"]
    #[inline(always)]
    pub fn is_1_000_v(&self) -> bool {
        *self == LVDS_R_A::_1_000_V
    }
    #[doc = "Checks if the value of the field is `_1_025_V`"]
    #[inline(always)]
    pub fn is_1_025_v(&self) -> bool {
        *self == LVDS_R_A::_1_025_V
    }
    #[doc = "Checks if the value of the field is `_1_050_V`"]
    #[inline(always)]
    pub fn is_1_050_v(&self) -> bool {
        *self == LVDS_R_A::_1_050_V
    }
    #[doc = "Checks if the value of the field is `_1_075_V`"]
    #[inline(always)]
    pub fn is_1_075_v(&self) -> bool {
        *self == LVDS_R_A::_1_075_V
    }
    #[doc = "Checks if the value of the field is `_1_100_V`"]
    #[inline(always)]
    pub fn is_1_100_v(&self) -> bool {
        *self == LVDS_R_A::_1_100_V
    }
}
#[doc = "Field `lvds_r` writer - Adjust current flowing through R of R to change the common signals amplitude."]
pub type LVDS_R_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCD_LVDS_ANA_SPEC, u8, LVDS_R_A, 3, O>;
impl<'a, const O: u8> LVDS_R_W<'a, O> {
    #[doc = "0.925 V"]
    #[inline(always)]
    pub fn _0_925_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_0_925_V)
    }
    #[doc = "0.950 V"]
    #[inline(always)]
    pub fn _0_950_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_0_950_V)
    }
    #[doc = "0.975 V"]
    #[inline(always)]
    pub fn _0_975_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_0_975_V)
    }
    #[doc = "1.000 V"]
    #[inline(always)]
    pub fn _1_000_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_1_000_V)
    }
    #[doc = "1.025 V"]
    #[inline(always)]
    pub fn _1_025_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_1_025_V)
    }
    #[doc = "1.050 V"]
    #[inline(always)]
    pub fn _1_050_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_1_050_V)
    }
    #[doc = "1.075 V"]
    #[inline(always)]
    pub fn _1_075_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_1_075_V)
    }
    #[doc = "1.100 V"]
    #[inline(always)]
    pub fn _1_100_v(self) -> &'a mut W {
        self.variant(LVDS_R_A::_1_100_V)
    }
}
#[doc = "LVDS clock channel direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDS_PLRC_A {
    #[doc = "0: Normal"]
    N_ORMAL = 0,
    #[doc = "1: Reverse"]
    R_EVERSE = 1,
}
impl From<LVDS_PLRC_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_PLRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lvds_plrc` reader - LVDS clock channel direction."]
pub type LVDS_PLRC_R = crate::BitReader<LVDS_PLRC_A>;
impl LVDS_PLRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDS_PLRC_A {
        match self.bits {
            false => LVDS_PLRC_A::N_ORMAL,
            true => LVDS_PLRC_A::R_EVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `N_ORMAL`"]
    #[inline(always)]
    pub fn is_n_ormal(&self) -> bool {
        *self == LVDS_PLRC_A::N_ORMAL
    }
    #[doc = "Checks if the value of the field is `R_EVERSE`"]
    #[inline(always)]
    pub fn is_r_everse(&self) -> bool {
        *self == LVDS_PLRC_A::R_EVERSE
    }
}
#[doc = "Field `lvds_plrc` writer - LVDS clock channel direction."]
pub type LVDS_PLRC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_LVDS_ANA_SPEC, LVDS_PLRC_A, O>;
impl<'a, const O: u8> LVDS_PLRC_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn n_ormal(self) -> &'a mut W {
        self.variant(LVDS_PLRC_A::N_ORMAL)
    }
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn r_everse(self) -> &'a mut W {
        self.variant(LVDS_PLRC_A::R_EVERSE)
    }
}
#[doc = "Field `lvds_plr` reader - LVDS data channel \\[3:0\\]
direction."]
pub type LVDS_PLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lvds_plr` writer - LVDS data channel \\[3:0\\]
direction."]
pub type LVDS_PLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCD_LVDS_ANA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 31 - Enable the bias circuit of the LVDS_Ana module."]
    #[inline(always)]
    pub fn lvds_en_mb(&self) -> LVDS_EN_MB_R {
        LVDS_EN_MB_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable LVDS"]
    #[inline(always)]
    pub fn en_lvds(&self) -> EN_LVDS_R {
        EN_LVDS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable the 24M clock"]
    #[inline(always)]
    pub fn en_24m(&self) -> EN_24M_R {
        EN_24M_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable clock channel drive"]
    #[inline(always)]
    pub fn lvds_hpren_drvc(&self) -> LVDS_HPREN_DRVC_R {
        LVDS_HPREN_DRVC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Enable data channel\\[3:0\\]
drive"]
    #[inline(always)]
    pub fn lvds_hpren_drv(&self) -> LVDS_HPREN_DRV_R {
        LVDS_HPREN_DRV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
    #[inline(always)]
    pub fn lvds_c(&self) -> LVDS_C_R {
        LVDS_C_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    pub fn lvds_denc(&self) -> LVDS_DENC_R {
        LVDS_DENC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    pub fn lvds_den(&self) -> LVDS_DEN_R {
        LVDS_DEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Adjust current flowing through R of R to change the common signals amplitude."]
    #[inline(always)]
    pub fn lvds_r(&self) -> LVDS_R_R {
        LVDS_R_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 4 - LVDS clock channel direction."]
    #[inline(always)]
    pub fn lvds_plrc(&self) -> LVDS_PLRC_R {
        LVDS_PLRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3 - LVDS data channel \\[3:0\\]
direction."]
    #[inline(always)]
    pub fn lvds_plr(&self) -> LVDS_PLR_R {
        LVDS_PLR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Enable the bias circuit of the LVDS_Ana module."]
    #[inline(always)]
    pub fn lvds_en_mb(&mut self) -> LVDS_EN_MB_W<31> {
        LVDS_EN_MB_W::new(self)
    }
    #[doc = "Bit 29 - Enable LVDS"]
    #[inline(always)]
    pub fn en_lvds(&mut self) -> EN_LVDS_W<29> {
        EN_LVDS_W::new(self)
    }
    #[doc = "Bit 28 - Enable the 24M clock"]
    #[inline(always)]
    pub fn en_24m(&mut self) -> EN_24M_W<28> {
        EN_24M_W::new(self)
    }
    #[doc = "Bit 24 - Enable clock channel drive"]
    #[inline(always)]
    pub fn lvds_hpren_drvc(&mut self) -> LVDS_HPREN_DRVC_W<24> {
        LVDS_HPREN_DRVC_W::new(self)
    }
    #[doc = "Bits 20:23 - Enable data channel\\[3:0\\]
drive"]
    #[inline(always)]
    pub fn lvds_hpren_drv(&mut self) -> LVDS_HPREN_DRV_W<20> {
        LVDS_HPREN_DRV_W::new(self)
    }
    #[doc = "Bits 17:19 - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
    #[inline(always)]
    pub fn lvds_c(&mut self) -> LVDS_C_W<17> {
        LVDS_C_W::new(self)
    }
    #[doc = "Bit 16 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    pub fn lvds_denc(&mut self) -> LVDS_DENC_W<16> {
        LVDS_DENC_W::new(self)
    }
    #[doc = "Bits 12:15 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    pub fn lvds_den(&mut self) -> LVDS_DEN_W<12> {
        LVDS_DEN_W::new(self)
    }
    #[doc = "Bits 8:10 - Adjust current flowing through R of R to change the common signals amplitude."]
    #[inline(always)]
    pub fn lvds_r(&mut self) -> LVDS_R_W<8> {
        LVDS_R_W::new(self)
    }
    #[doc = "Bit 4 - LVDS clock channel direction."]
    #[inline(always)]
    pub fn lvds_plrc(&mut self) -> LVDS_PLRC_W<4> {
        LVDS_PLRC_W::new(self)
    }
    #[doc = "Bits 0:3 - LVDS data channel \\[3:0\\]
direction."]
    #[inline(always)]
    pub fn lvds_plr(&mut self) -> LVDS_PLR_W<0> {
        LVDS_PLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD LVDS Analog Register \\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lvds_ana](index.html) module"]
pub struct LCD_LVDS_ANA_SPEC;
impl crate::RegisterSpec for LCD_LVDS_ANA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lvds_ana::R](R) reader structure"]
impl crate::Readable for LCD_LVDS_ANA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lvds_ana::W](W) writer structure"]
impl crate::Writable for LCD_LVDS_ANA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lcd_lvds_ana%s to value 0"]
impl crate::Resettable for LCD_LVDS_ANA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
