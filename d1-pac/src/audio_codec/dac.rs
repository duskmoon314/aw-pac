#[doc = "Register `dac` reader"]
pub struct R(crate::R<DAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dac` writer"]
pub struct W(crate::W<DAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SPEC>;
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
impl From<crate::W<DAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lineout_vol_ctrl` reader - LINEOUT Volume Control.\n\nTotal 30 level from 0x1F to 0x02 with the volume 0 dB to -43.5 dB, -1.5 dB/step, mute when 00000 or 00001"]
pub type LINEOUT_VOL_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lineout_vol_ctrl` writer - LINEOUT Volume Control.\n\nTotal 30 level from 0x1F to 0x02 with the volume 0 dB to -43.5 dB, -1.5 dB/step, mute when 00000 or 00001"]
pub type LINEOUT_VOL_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC_SPEC, u8, u8, 5, O>;
#[doc = "Field `lineoutr_diffen` reader - Right Channel LINEOUT Output Control"]
pub type LINEOUTR_DIFFEN_R = crate::BitReader<LINEOUTR_DIFFEN_A>;
#[doc = "Right Channel LINEOUT Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEOUTR_DIFFEN_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DIFFERENTIAL = 1,
}
impl From<LINEOUTR_DIFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEOUTR_DIFFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEOUTR_DIFFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEOUTR_DIFFEN_A {
        match self.bits {
            false => LINEOUTR_DIFFEN_A::SINGLE,
            true => LINEOUTR_DIFFEN_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == LINEOUTR_DIFFEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == LINEOUTR_DIFFEN_A::DIFFERENTIAL
    }
}
#[doc = "Field `lineoutr_diffen` writer - Right Channel LINEOUT Output Control"]
pub type LINEOUTR_DIFFEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DAC_SPEC, LINEOUTR_DIFFEN_A, O>;
impl<'a, const O: u8> LINEOUTR_DIFFEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(LINEOUTR_DIFFEN_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(LINEOUTR_DIFFEN_A::DIFFERENTIAL)
    }
}
#[doc = "Field `lineoutl_diffen` reader - Left Channel LINEOUT Output Control"]
pub type LINEOUTL_DIFFEN_R = crate::BitReader<LINEOUTL_DIFFEN_A>;
#[doc = "Left Channel LINEOUT Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEOUTL_DIFFEN_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DIFFERENTIAL = 1,
}
impl From<LINEOUTL_DIFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEOUTL_DIFFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEOUTL_DIFFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEOUTL_DIFFEN_A {
        match self.bits {
            false => LINEOUTL_DIFFEN_A::SINGLE,
            true => LINEOUTL_DIFFEN_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == LINEOUTL_DIFFEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == LINEOUTL_DIFFEN_A::DIFFERENTIAL
    }
}
#[doc = "Field `lineoutl_diffen` writer - Left Channel LINEOUT Output Control"]
pub type LINEOUTL_DIFFEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DAC_SPEC, LINEOUTL_DIFFEN_A, O>;
impl<'a, const O: u8> LINEOUTL_DIFFEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(LINEOUTL_DIFFEN_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(LINEOUTL_DIFFEN_A::DIFFERENTIAL)
    }
}
#[doc = "Field `rmute` reader - DACR to Right Channel LINEOUT Mute Control"]
pub type RMUTE_R = crate::BitReader<RMUTE_A>;
#[doc = "DACR to Right Channel LINEOUT Mute Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMUTE_A {
    #[doc = "0: `0`"]
    MUTE = 0,
    #[doc = "1: `1`"]
    UNMUTE = 1,
}
impl From<RMUTE_A> for bool {
    #[inline(always)]
    fn from(variant: RMUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl RMUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMUTE_A {
        match self.bits {
            false => RMUTE_A::MUTE,
            true => RMUTE_A::UNMUTE,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RMUTE_A::MUTE
    }
    #[doc = "Checks if the value of the field is `UNMUTE`"]
    #[inline(always)]
    pub fn is_unmute(&self) -> bool {
        *self == RMUTE_A::UNMUTE
    }
}
#[doc = "Field `rmute` writer - DACR to Right Channel LINEOUT Mute Control"]
pub type RMUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SPEC, RMUTE_A, O>;
impl<'a, const O: u8> RMUTE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(RMUTE_A::MUTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn unmute(self) -> &'a mut W {
        self.variant(RMUTE_A::UNMUTE)
    }
}
#[doc = "Field `lineoutren` reader - Right Channel LINEOUT Enable"]
pub type LINEOUTREN_R = crate::BitReader<LINEOUTREN_A>;
#[doc = "Right Channel LINEOUT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEOUTREN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LINEOUTREN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEOUTREN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEOUTREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEOUTREN_A {
        match self.bits {
            false => LINEOUTREN_A::DISABLE,
            true => LINEOUTREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LINEOUTREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LINEOUTREN_A::ENABLE
    }
}
#[doc = "Field `lineoutren` writer - Right Channel LINEOUT Enable"]
pub type LINEOUTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SPEC, LINEOUTREN_A, O>;
impl<'a, const O: u8> LINEOUTREN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LINEOUTREN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LINEOUTREN_A::ENABLE)
    }
}
#[doc = "Field `lmute` reader - DACL to Left Channel LINEOUT Mute Control"]
pub type LMUTE_R = crate::BitReader<LMUTE_A>;
#[doc = "DACL to Left Channel LINEOUT Mute Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMUTE_A {
    #[doc = "0: `0`"]
    MUTE = 0,
    #[doc = "1: `1`"]
    UNMUTE = 1,
}
impl From<LMUTE_A> for bool {
    #[inline(always)]
    fn from(variant: LMUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl LMUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMUTE_A {
        match self.bits {
            false => LMUTE_A::MUTE,
            true => LMUTE_A::UNMUTE,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == LMUTE_A::MUTE
    }
    #[doc = "Checks if the value of the field is `UNMUTE`"]
    #[inline(always)]
    pub fn is_unmute(&self) -> bool {
        *self == LMUTE_A::UNMUTE
    }
}
#[doc = "Field `lmute` writer - DACL to Left Channel LINEOUT Mute Control"]
pub type LMUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SPEC, LMUTE_A, O>;
impl<'a, const O: u8> LMUTE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(LMUTE_A::MUTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn unmute(self) -> &'a mut W {
        self.variant(LMUTE_A::UNMUTE)
    }
}
#[doc = "Field `lineoutlen` reader - Left Channel LINEOUT Enable"]
pub type LINEOUTLEN_R = crate::BitReader<LINEOUTLEN_A>;
#[doc = "Left Channel LINEOUT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEOUTLEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LINEOUTLEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEOUTLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEOUTLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEOUTLEN_A {
        match self.bits {
            false => LINEOUTLEN_A::DISABLE,
            true => LINEOUTLEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LINEOUTLEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LINEOUTLEN_A::ENABLE
    }
}
#[doc = "Field `lineoutlen` writer - Left Channel LINEOUT Enable"]
pub type LINEOUTLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SPEC, LINEOUTLEN_A, O>;
impl<'a, const O: u8> LINEOUTLEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LINEOUTLEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LINEOUTLEN_A::ENABLE)
    }
}
#[doc = "Field `dacr_en` reader - DACR Enable"]
pub type DACR_EN_R = crate::BitReader<DACR_EN_A>;
#[doc = "DACR Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACR_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DACR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DACR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACR_EN_A {
        match self.bits {
            false => DACR_EN_A::DISABLE,
            true => DACR_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACR_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DACR_EN_A::ENABLE
    }
}
#[doc = "Field `dacr_en` writer - DACR Enable"]
pub type DACR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SPEC, DACR_EN_A, O>;
impl<'a, const O: u8> DACR_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACR_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DACR_EN_A::ENABLE)
    }
}
#[doc = "Field `dacl_en` reader - DACL Enable"]
pub type DACL_EN_R = crate::BitReader<DACL_EN_A>;
#[doc = "DACL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DACL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DACL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACL_EN_A {
        match self.bits {
            false => DACL_EN_A::DISABLE,
            true => DACL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DACL_EN_A::ENABLE
    }
}
#[doc = "Field `dacl_en` writer - DACL Enable"]
pub type DACL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SPEC, DACL_EN_A, O>;
impl<'a, const O: u8> DACL_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DACL_EN_A::ENABLE)
    }
}
#[doc = "Field `iopdacs` reader - OPDAC L/R Bias Current Select"]
pub type IOPDACS_R = crate::FieldReader<u8, IOPDACS_A>;
#[doc = "OPDAC L/R Bias Current Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOPDACS_A {
    #[doc = "0: `0`"]
    C6U = 0,
    #[doc = "1: `1`"]
    C7U = 1,
    #[doc = "2: `10`"]
    C8U = 2,
    #[doc = "3: `11`"]
    C9U = 3,
}
impl From<IOPDACS_A> for u8 {
    #[inline(always)]
    fn from(variant: IOPDACS_A) -> Self {
        variant as _
    }
}
impl IOPDACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPDACS_A {
        match self.bits {
            0 => IOPDACS_A::C6U,
            1 => IOPDACS_A::C7U,
            2 => IOPDACS_A::C8U,
            3 => IOPDACS_A::C9U,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `C6U`"]
    #[inline(always)]
    pub fn is_c6u(&self) -> bool {
        *self == IOPDACS_A::C6U
    }
    #[doc = "Checks if the value of the field is `C7U`"]
    #[inline(always)]
    pub fn is_c7u(&self) -> bool {
        *self == IOPDACS_A::C7U
    }
    #[doc = "Checks if the value of the field is `C8U`"]
    #[inline(always)]
    pub fn is_c8u(&self) -> bool {
        *self == IOPDACS_A::C8U
    }
    #[doc = "Checks if the value of the field is `C9U`"]
    #[inline(always)]
    pub fn is_c9u(&self) -> bool {
        *self == IOPDACS_A::C9U
    }
}
#[doc = "Field `iopdacs` writer - OPDAC L/R Bias Current Select"]
pub type IOPDACS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DAC_SPEC, u8, IOPDACS_A, 2, O>;
impl<'a, const O: u8> IOPDACS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn c6u(self) -> &'a mut W {
        self.variant(IOPDACS_A::C6U)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn c7u(self) -> &'a mut W {
        self.variant(IOPDACS_A::C7U)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn c8u(self) -> &'a mut W {
        self.variant(IOPDACS_A::C8U)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn c9u(self) -> &'a mut W {
        self.variant(IOPDACS_A::C9U)
    }
}
#[doc = "Field `ilineoutamps` reader - LINEOUT L/R AMP Bias Current Select"]
pub type ILINEOUTAMPS_R = crate::FieldReader<u8, ILINEOUTAMPS_A>;
#[doc = "LINEOUT L/R AMP Bias Current Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ILINEOUTAMPS_A {
    #[doc = "0: `0`"]
    C6U = 0,
    #[doc = "1: `1`"]
    C7U = 1,
    #[doc = "2: `10`"]
    C8U = 2,
    #[doc = "3: `11`"]
    C9U = 3,
}
impl From<ILINEOUTAMPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ILINEOUTAMPS_A) -> Self {
        variant as _
    }
}
impl ILINEOUTAMPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILINEOUTAMPS_A {
        match self.bits {
            0 => ILINEOUTAMPS_A::C6U,
            1 => ILINEOUTAMPS_A::C7U,
            2 => ILINEOUTAMPS_A::C8U,
            3 => ILINEOUTAMPS_A::C9U,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `C6U`"]
    #[inline(always)]
    pub fn is_c6u(&self) -> bool {
        *self == ILINEOUTAMPS_A::C6U
    }
    #[doc = "Checks if the value of the field is `C7U`"]
    #[inline(always)]
    pub fn is_c7u(&self) -> bool {
        *self == ILINEOUTAMPS_A::C7U
    }
    #[doc = "Checks if the value of the field is `C8U`"]
    #[inline(always)]
    pub fn is_c8u(&self) -> bool {
        *self == ILINEOUTAMPS_A::C8U
    }
    #[doc = "Checks if the value of the field is `C9U`"]
    #[inline(always)]
    pub fn is_c9u(&self) -> bool {
        *self == ILINEOUTAMPS_A::C9U
    }
}
#[doc = "Field `ilineoutamps` writer - LINEOUT L/R AMP Bias Current Select"]
pub type ILINEOUTAMPS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DAC_SPEC, u8, ILINEOUTAMPS_A, 2, O>;
impl<'a, const O: u8> ILINEOUTAMPS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn c6u(self) -> &'a mut W {
        self.variant(ILINEOUTAMPS_A::C6U)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn c7u(self) -> &'a mut W {
        self.variant(ILINEOUTAMPS_A::C7U)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn c8u(self) -> &'a mut W {
        self.variant(ILINEOUTAMPS_A::C8U)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn c9u(self) -> &'a mut W {
        self.variant(ILINEOUTAMPS_A::C9U)
    }
}
#[doc = "Field `iopvrs` reader - VRA2 Buffer OP and Headphone Feedback Buffer OP Bias Current Select"]
pub type IOPVRS_R = crate::FieldReader<u8, IOPVRS_A>;
#[doc = "VRA2 Buffer OP and Headphone Feedback Buffer OP Bias Current Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOPVRS_A {
    #[doc = "0: `0`"]
    C6U = 0,
    #[doc = "1: `1`"]
    C7U = 1,
    #[doc = "2: `10`"]
    C8U = 2,
    #[doc = "3: `11`"]
    C9U = 3,
}
impl From<IOPVRS_A> for u8 {
    #[inline(always)]
    fn from(variant: IOPVRS_A) -> Self {
        variant as _
    }
}
impl IOPVRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPVRS_A {
        match self.bits {
            0 => IOPVRS_A::C6U,
            1 => IOPVRS_A::C7U,
            2 => IOPVRS_A::C8U,
            3 => IOPVRS_A::C9U,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `C6U`"]
    #[inline(always)]
    pub fn is_c6u(&self) -> bool {
        *self == IOPVRS_A::C6U
    }
    #[doc = "Checks if the value of the field is `C7U`"]
    #[inline(always)]
    pub fn is_c7u(&self) -> bool {
        *self == IOPVRS_A::C7U
    }
    #[doc = "Checks if the value of the field is `C8U`"]
    #[inline(always)]
    pub fn is_c8u(&self) -> bool {
        *self == IOPVRS_A::C8U
    }
    #[doc = "Checks if the value of the field is `C9U`"]
    #[inline(always)]
    pub fn is_c9u(&self) -> bool {
        *self == IOPVRS_A::C9U
    }
}
#[doc = "Field `iopvrs` writer - VRA2 Buffer OP and Headphone Feedback Buffer OP Bias Current Select"]
pub type IOPVRS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DAC_SPEC, u8, IOPVRS_A, 2, O>;
impl<'a, const O: u8> IOPVRS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn c6u(self) -> &'a mut W {
        self.variant(IOPVRS_A::C6U)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn c7u(self) -> &'a mut W {
        self.variant(IOPVRS_A::C7U)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn c8u(self) -> &'a mut W {
        self.variant(IOPVRS_A::C8U)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn c9u(self) -> &'a mut W {
        self.variant(IOPVRS_A::C9U)
    }
}
#[doc = "Field `current_test_select` reader - Internal Current Sink Test Enable (from MICIN3P pin)"]
pub type CURRENT_TEST_SELECT_R = crate::BitReader<CURRENT_TEST_SELECT_A>;
#[doc = "Internal Current Sink Test Enable (from MICIN3P pin)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURRENT_TEST_SELECT_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    DEBUG = 1,
}
impl From<CURRENT_TEST_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CURRENT_TEST_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CURRENT_TEST_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURRENT_TEST_SELECT_A {
        match self.bits {
            false => CURRENT_TEST_SELECT_A::NORMAL,
            true => CURRENT_TEST_SELECT_A::DEBUG,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CURRENT_TEST_SELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DEBUG`"]
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == CURRENT_TEST_SELECT_A::DEBUG
    }
}
#[doc = "Field `current_test_select` writer - Internal Current Sink Test Enable (from MICIN3P pin)"]
pub type CURRENT_TEST_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DAC_SPEC, CURRENT_TEST_SELECT_A, O>;
impl<'a, const O: u8> CURRENT_TEST_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CURRENT_TEST_SELECT_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn debug(self) -> &'a mut W {
        self.variant(CURRENT_TEST_SELECT_A::DEBUG)
    }
}
impl R {
    #[doc = "Bits 0:4 - LINEOUT Volume Control.\n\nTotal 30 level from 0x1F to 0x02 with the volume 0 dB to -43.5 dB, -1.5 dB/step, mute when 00000 or 00001"]
    #[inline(always)]
    pub fn lineout_vol_ctrl(&self) -> LINEOUT_VOL_CTRL_R {
        LINEOUT_VOL_CTRL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Right Channel LINEOUT Output Control"]
    #[inline(always)]
    pub fn lineoutr_diffen(&self) -> LINEOUTR_DIFFEN_R {
        LINEOUTR_DIFFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Left Channel LINEOUT Output Control"]
    #[inline(always)]
    pub fn lineoutl_diffen(&self) -> LINEOUTL_DIFFEN_R {
        LINEOUTL_DIFFEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - DACR to Right Channel LINEOUT Mute Control"]
    #[inline(always)]
    pub fn rmute(&self) -> RMUTE_R {
        RMUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Right Channel LINEOUT Enable"]
    #[inline(always)]
    pub fn lineoutren(&self) -> LINEOUTREN_R {
        LINEOUTREN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DACL to Left Channel LINEOUT Mute Control"]
    #[inline(always)]
    pub fn lmute(&self) -> LMUTE_R {
        LMUTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Left Channel LINEOUT Enable"]
    #[inline(always)]
    pub fn lineoutlen(&self) -> LINEOUTLEN_R {
        LINEOUTLEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DACR Enable"]
    #[inline(always)]
    pub fn dacr_en(&self) -> DACR_EN_R {
        DACR_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DACL Enable"]
    #[inline(always)]
    pub fn dacl_en(&self) -> DACL_EN_R {
        DACL_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - OPDAC L/R Bias Current Select"]
    #[inline(always)]
    pub fn iopdacs(&self) -> IOPDACS_R {
        IOPDACS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - LINEOUT L/R AMP Bias Current Select"]
    #[inline(always)]
    pub fn ilineoutamps(&self) -> ILINEOUTAMPS_R {
        ILINEOUTAMPS_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - VRA2 Buffer OP and Headphone Feedback Buffer OP Bias Current Select"]
    #[inline(always)]
    pub fn iopvrs(&self) -> IOPVRS_R {
        IOPVRS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Internal Current Sink Test Enable (from MICIN3P pin)"]
    #[inline(always)]
    pub fn current_test_select(&self) -> CURRENT_TEST_SELECT_R {
        CURRENT_TEST_SELECT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - LINEOUT Volume Control.\n\nTotal 30 level from 0x1F to 0x02 with the volume 0 dB to -43.5 dB, -1.5 dB/step, mute when 00000 or 00001"]
    #[inline(always)]
    #[must_use]
    pub fn lineout_vol_ctrl(&mut self) -> LINEOUT_VOL_CTRL_W<0> {
        LINEOUT_VOL_CTRL_W::new(self)
    }
    #[doc = "Bit 5 - Right Channel LINEOUT Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn lineoutr_diffen(&mut self) -> LINEOUTR_DIFFEN_W<5> {
        LINEOUTR_DIFFEN_W::new(self)
    }
    #[doc = "Bit 6 - Left Channel LINEOUT Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn lineoutl_diffen(&mut self) -> LINEOUTL_DIFFEN_W<6> {
        LINEOUTL_DIFFEN_W::new(self)
    }
    #[doc = "Bit 10 - DACR to Right Channel LINEOUT Mute Control"]
    #[inline(always)]
    #[must_use]
    pub fn rmute(&mut self) -> RMUTE_W<10> {
        RMUTE_W::new(self)
    }
    #[doc = "Bit 11 - Right Channel LINEOUT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lineoutren(&mut self) -> LINEOUTREN_W<11> {
        LINEOUTREN_W::new(self)
    }
    #[doc = "Bit 12 - DACL to Left Channel LINEOUT Mute Control"]
    #[inline(always)]
    #[must_use]
    pub fn lmute(&mut self) -> LMUTE_W<12> {
        LMUTE_W::new(self)
    }
    #[doc = "Bit 13 - Left Channel LINEOUT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lineoutlen(&mut self) -> LINEOUTLEN_W<13> {
        LINEOUTLEN_W::new(self)
    }
    #[doc = "Bit 14 - DACR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacr_en(&mut self) -> DACR_EN_W<14> {
        DACR_EN_W::new(self)
    }
    #[doc = "Bit 15 - DACL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacl_en(&mut self) -> DACL_EN_W<15> {
        DACL_EN_W::new(self)
    }
    #[doc = "Bits 16:17 - OPDAC L/R Bias Current Select"]
    #[inline(always)]
    #[must_use]
    pub fn iopdacs(&mut self) -> IOPDACS_W<16> {
        IOPDACS_W::new(self)
    }
    #[doc = "Bits 18:19 - LINEOUT L/R AMP Bias Current Select"]
    #[inline(always)]
    #[must_use]
    pub fn ilineoutamps(&mut self) -> ILINEOUTAMPS_W<18> {
        ILINEOUTAMPS_W::new(self)
    }
    #[doc = "Bits 20:21 - VRA2 Buffer OP and Headphone Feedback Buffer OP Bias Current Select"]
    #[inline(always)]
    #[must_use]
    pub fn iopvrs(&mut self) -> IOPVRS_W<20> {
        IOPVRS_W::new(self)
    }
    #[doc = "Bit 23 - Internal Current Sink Test Enable (from MICIN3P pin)"]
    #[inline(always)]
    #[must_use]
    pub fn current_test_select(&mut self) -> CURRENT_TEST_SELECT_W<23> {
        CURRENT_TEST_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac](index.html) module"]
pub struct DAC_SPEC;
impl crate::RegisterSpec for DAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac::R](R) reader structure"]
impl crate::Readable for DAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac::W](W) writer structure"]
impl crate::Writable for DAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dac to value 0"]
impl crate::Resettable for DAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
