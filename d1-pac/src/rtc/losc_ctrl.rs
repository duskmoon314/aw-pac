#[doc = "Register `losc_ctrl` reader"]
pub struct R(crate::R<LOSC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOSC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOSC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOSC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `losc_ctrl` writer"]
pub struct W(crate::W<LOSC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOSC_CTRL_SPEC>;
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
impl From<crate::W<LOSC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOSC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `losc_src_sel` reader - LOSC Clock Source Select"]
pub type LOSC_SRC_SEL_R = crate::BitReader<LOSC_SRC_SEL_A>;
#[doc = "LOSC Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOSC_SRC_SEL_A {
    #[doc = "0: Low frequency clock from 16M RC"]
    LOW = 0,
    #[doc = "1: External 32.768 kHz OSC"]
    EXTERNAL = 1,
}
impl From<LOSC_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LOSC_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LOSC_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOSC_SRC_SEL_A {
        match self.bits {
            false => LOSC_SRC_SEL_A::LOW,
            true => LOSC_SRC_SEL_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LOSC_SRC_SEL_A::LOW
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == LOSC_SRC_SEL_A::EXTERNAL
    }
}
#[doc = "Field `losc_src_sel` writer - LOSC Clock Source Select"]
pub type LOSC_SRC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, LOSC_SRC_SEL_A, O>;
impl<'a, const O: u8> LOSC_SRC_SEL_W<'a, O> {
    #[doc = "Low frequency clock from 16M RC"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LOSC_SRC_SEL_A::LOW)
    }
    #[doc = "External 32.768 kHz OSC"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(LOSC_SRC_SEL_A::EXTERNAL)
    }
}
#[doc = "Field `rtc_src_sel` reader - RTC_TIMER Clock Source Select"]
pub type RTC_SRC_SEL_R = crate::BitReader<RTC_SRC_SEL_A>;
#[doc = "RTC_TIMER Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_SRC_SEL_A {
    #[doc = "0: LOSC_SRC"]
    LOSC_SRC = 0,
    #[doc = "1: 24MDIV32K\n\nBefore switching the bit, make sure that the 24MDIV32K function is enabled, that is, the bit16 of the 32K Fanout Control Register is 1."]
    _24MDIV32K = 1,
}
impl From<RTC_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_SRC_SEL_A {
        match self.bits {
            false => RTC_SRC_SEL_A::LOSC_SRC,
            true => RTC_SRC_SEL_A::_24MDIV32K,
        }
    }
    #[doc = "Checks if the value of the field is `LOSC_SRC`"]
    #[inline(always)]
    pub fn is_losc_src(&self) -> bool {
        *self == RTC_SRC_SEL_A::LOSC_SRC
    }
    #[doc = "Checks if the value of the field is `_24MDIV32K`"]
    #[inline(always)]
    pub fn is_24mdiv32k(&self) -> bool {
        *self == RTC_SRC_SEL_A::_24MDIV32K
    }
}
#[doc = "Field `rtc_src_sel` writer - RTC_TIMER Clock Source Select"]
pub type RTC_SRC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, RTC_SRC_SEL_A, O>;
impl<'a, const O: u8> RTC_SRC_SEL_W<'a, O> {
    #[doc = "LOSC_SRC"]
    #[inline(always)]
    pub fn losc_src(self) -> &'a mut W {
        self.variant(RTC_SRC_SEL_A::LOSC_SRC)
    }
    #[doc = "24MDIV32K\n\nBefore switching the bit, make sure that the 24MDIV32K function is enabled, that is, the bit16 of the 32K Fanout Control Register is 1."]
    #[inline(always)]
    pub fn _24mdiv32k(self) -> &'a mut W {
        self.variant(RTC_SRC_SEL_A::_24MDIV32K)
    }
}
#[doc = "Field `ext_losc_gsm` reader - External 32.768 kHz Crystal GSM\n\nWhen GSM is changed, the 32K oscillation circuit will arise transient instability. If the autoswitch function (bit 15) is enabled, 32K changes to RC16M with certain probability. The GSM can influence the time of 32K starting oscillation, the more the GSM, the shorter the time of starting oscillation. So modifying GSM is not recommended.\n\nIf you need to modify the GSM, firstly disable the auto switch function (bit 15), with a delay of 50 us, then change the GSM, the 32K clock source is changed to external clock."]
pub type EXT_LOSC_GSM_R = crate::FieldReader<u8, EXT_LOSC_GSM_A>;
#[doc = "External 32.768 kHz Crystal GSM\n\nWhen GSM is changed, the 32K oscillation circuit will arise transient instability. If the autoswitch function (bit 15) is enabled, 32K changes to RC16M with certain probability. The GSM can influence the time of 32K starting oscillation, the more the GSM, the shorter the time of starting oscillation. So modifying GSM is not recommended.\n\nIf you need to modify the GSM, firstly disable the auto switch function (bit 15), with a delay of 50 us, then change the GSM, the 32K clock source is changed to external clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXT_LOSC_GSM_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "3: High"]
    HIGH = 3,
}
impl From<EXT_LOSC_GSM_A> for u8 {
    #[inline(always)]
    fn from(variant: EXT_LOSC_GSM_A) -> Self {
        variant as _
    }
}
impl EXT_LOSC_GSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXT_LOSC_GSM_A> {
        match self.bits {
            0 => Some(EXT_LOSC_GSM_A::LOW),
            3 => Some(EXT_LOSC_GSM_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EXT_LOSC_GSM_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EXT_LOSC_GSM_A::HIGH
    }
}
#[doc = "Field `ext_losc_gsm` writer - External 32.768 kHz Crystal GSM\n\nWhen GSM is changed, the 32K oscillation circuit will arise transient instability. If the autoswitch function (bit 15) is enabled, 32K changes to RC16M with certain probability. The GSM can influence the time of 32K starting oscillation, the more the GSM, the shorter the time of starting oscillation. So modifying GSM is not recommended.\n\nIf you need to modify the GSM, firstly disable the auto switch function (bit 15), with a delay of 50 us, then change the GSM, the 32K clock source is changed to external clock."]
pub type EXT_LOSC_GSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOSC_CTRL_SPEC, u8, EXT_LOSC_GSM_A, 2, O>;
impl<'a, const O: u8> EXT_LOSC_GSM_W<'a, O> {
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EXT_LOSC_GSM_A::LOW)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EXT_LOSC_GSM_A::HIGH)
    }
}
#[doc = "Field `ext_losc_en` reader - External 32.768 kHz Crystal Enable"]
pub type EXT_LOSC_EN_R = crate::BitReader<EXT_LOSC_EN_A>;
#[doc = "External 32.768 kHz Crystal Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_LOSC_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<EXT_LOSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_LOSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXT_LOSC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_LOSC_EN_A {
        match self.bits {
            false => EXT_LOSC_EN_A::DISABLE,
            true => EXT_LOSC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXT_LOSC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXT_LOSC_EN_A::ENABLE
    }
}
#[doc = "Field `ext_losc_en` writer - External 32.768 kHz Crystal Enable"]
pub type EXT_LOSC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, EXT_LOSC_EN_A, O>;
impl<'a, const O: u8> EXT_LOSC_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXT_LOSC_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXT_LOSC_EN_A::ENABLE)
    }
}
#[doc = "Field `rtc_day_acce` reader - RTC DAY access\n\nAfter writing the RTC DAY register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC DAY register, the DAY register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
pub type RTC_DAY_ACCE_R = crate::BitReader<bool>;
#[doc = "Field `rtc_day_acce` writer - RTC DAY access\n\nAfter writing the RTC DAY register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC DAY register, the DAY register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
pub type RTC_DAY_ACCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, bool, O>;
#[doc = "Field `rtc_hhmmss_acce` reader - RTC Hour Minute Second access\n\nAfter writing the RTC HH-MM-SS Register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC HH-MM-SS Register, the RTC HH-MM-SS Register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
pub type RTC_HHMMSS_ACCE_R = crate::BitReader<bool>;
#[doc = "Field `rtc_hhmmss_acce` writer - RTC Hour Minute Second access\n\nAfter writing the RTC HH-MM-SS Register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC HH-MM-SS Register, the RTC HH-MM-SS Register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
pub type RTC_HHMMSS_ACCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, bool, O>;
#[doc = "Field `losc_auto_swt_32k_sel_en` reader - LOSC auto switch 32K clk source select enable"]
pub type LOSC_AUTO_SWT_32K_SEL_EN_R = crate::BitReader<LOSC_AUTO_SWT_32K_SEL_EN_A>;
#[doc = "LOSC auto switch 32K clk source select enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOSC_AUTO_SWT_32K_SEL_EN_A {
    #[doc = "0: Disable. When the losc losts, the 32k clk source will not change to RC"]
    DISABLE = 0,
    #[doc = "1: Enable. When the losc losts, the 32k clk source will change to RC (LOSC_SRC_SEL will be changed from 1 to 0)"]
    ENABLE = 1,
}
impl From<LOSC_AUTO_SWT_32K_SEL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOSC_AUTO_SWT_32K_SEL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LOSC_AUTO_SWT_32K_SEL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOSC_AUTO_SWT_32K_SEL_EN_A {
        match self.bits {
            false => LOSC_AUTO_SWT_32K_SEL_EN_A::DISABLE,
            true => LOSC_AUTO_SWT_32K_SEL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOSC_AUTO_SWT_32K_SEL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOSC_AUTO_SWT_32K_SEL_EN_A::ENABLE
    }
}
#[doc = "Field `losc_auto_swt_32k_sel_en` writer - LOSC auto switch 32K clk source select enable"]
pub type LOSC_AUTO_SWT_32K_SEL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, LOSC_AUTO_SWT_32K_SEL_EN_A, O>;
impl<'a, const O: u8> LOSC_AUTO_SWT_32K_SEL_EN_W<'a, O> {
    #[doc = "Disable. When the losc losts, the 32k clk source will not change to RC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOSC_AUTO_SWT_32K_SEL_EN_A::DISABLE)
    }
    #[doc = "Enable. When the losc losts, the 32k clk source will change to RC (LOSC_SRC_SEL will be changed from 1 to 0)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOSC_AUTO_SWT_32K_SEL_EN_A::ENABLE)
    }
}
#[doc = "Field `losc_auto_swt_function` reader - LOSC auto switch function disable"]
pub type LOSC_AUTO_SWT_FUNCTION_R = crate::BitReader<LOSC_AUTO_SWT_FUNCTION_A>;
#[doc = "LOSC auto switch function disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOSC_AUTO_SWT_FUNCTION_A {
    #[doc = "0: Enable"]
    ENABLE = 0,
    #[doc = "1: Disable"]
    DISABLE = 1,
}
impl From<LOSC_AUTO_SWT_FUNCTION_A> for bool {
    #[inline(always)]
    fn from(variant: LOSC_AUTO_SWT_FUNCTION_A) -> Self {
        variant as u8 != 0
    }
}
impl LOSC_AUTO_SWT_FUNCTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOSC_AUTO_SWT_FUNCTION_A {
        match self.bits {
            false => LOSC_AUTO_SWT_FUNCTION_A::ENABLE,
            true => LOSC_AUTO_SWT_FUNCTION_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOSC_AUTO_SWT_FUNCTION_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOSC_AUTO_SWT_FUNCTION_A::DISABLE
    }
}
#[doc = "Field `losc_auto_swt_function` writer - LOSC auto switch function disable"]
pub type LOSC_AUTO_SWT_FUNCTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOSC_CTRL_SPEC, LOSC_AUTO_SWT_FUNCTION_A, O>;
impl<'a, const O: u8> LOSC_AUTO_SWT_FUNCTION_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOSC_AUTO_SWT_FUNCTION_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOSC_AUTO_SWT_FUNCTION_A::DISABLE)
    }
}
#[doc = "Field `key_field` writer - Key Field\n\nThis field should be filled with 0x16AA, and then the bit0 and bit1 can be written with the new value."]
pub type KEY_FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOSC_CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - LOSC Clock Source Select"]
    #[inline(always)]
    pub fn losc_src_sel(&self) -> LOSC_SRC_SEL_R {
        LOSC_SRC_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_TIMER Clock Source Select"]
    #[inline(always)]
    pub fn rtc_src_sel(&self) -> RTC_SRC_SEL_R {
        RTC_SRC_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - External 32.768 kHz Crystal GSM\n\nWhen GSM is changed, the 32K oscillation circuit will arise transient instability. If the autoswitch function (bit 15) is enabled, 32K changes to RC16M with certain probability. The GSM can influence the time of 32K starting oscillation, the more the GSM, the shorter the time of starting oscillation. So modifying GSM is not recommended.\n\nIf you need to modify the GSM, firstly disable the auto switch function (bit 15), with a delay of 50 us, then change the GSM, the 32K clock source is changed to external clock."]
    #[inline(always)]
    pub fn ext_losc_gsm(&self) -> EXT_LOSC_GSM_R {
        EXT_LOSC_GSM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - External 32.768 kHz Crystal Enable"]
    #[inline(always)]
    pub fn ext_losc_en(&self) -> EXT_LOSC_EN_R {
        EXT_LOSC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC DAY access\n\nAfter writing the RTC DAY register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC DAY register, the DAY register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
    #[inline(always)]
    pub fn rtc_day_acce(&self) -> RTC_DAY_ACCE_R {
        RTC_DAY_ACCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC Hour Minute Second access\n\nAfter writing the RTC HH-MM-SS Register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC HH-MM-SS Register, the RTC HH-MM-SS Register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
    #[inline(always)]
    pub fn rtc_hhmmss_acce(&self) -> RTC_HHMMSS_ACCE_R {
        RTC_HHMMSS_ACCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - LOSC auto switch 32K clk source select enable"]
    #[inline(always)]
    pub fn losc_auto_swt_32k_sel_en(&self) -> LOSC_AUTO_SWT_32K_SEL_EN_R {
        LOSC_AUTO_SWT_32K_SEL_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LOSC auto switch function disable"]
    #[inline(always)]
    pub fn losc_auto_swt_function(&self) -> LOSC_AUTO_SWT_FUNCTION_R {
        LOSC_AUTO_SWT_FUNCTION_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOSC Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn losc_src_sel(&mut self) -> LOSC_SRC_SEL_W<0> {
        LOSC_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 1 - RTC_TIMER Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_src_sel(&mut self) -> RTC_SRC_SEL_W<1> {
        RTC_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - External 32.768 kHz Crystal GSM\n\nWhen GSM is changed, the 32K oscillation circuit will arise transient instability. If the autoswitch function (bit 15) is enabled, 32K changes to RC16M with certain probability. The GSM can influence the time of 32K starting oscillation, the more the GSM, the shorter the time of starting oscillation. So modifying GSM is not recommended.\n\nIf you need to modify the GSM, firstly disable the auto switch function (bit 15), with a delay of 50 us, then change the GSM, the 32K clock source is changed to external clock."]
    #[inline(always)]
    #[must_use]
    pub fn ext_losc_gsm(&mut self) -> EXT_LOSC_GSM_W<2> {
        EXT_LOSC_GSM_W::new(self)
    }
    #[doc = "Bit 4 - External 32.768 kHz Crystal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_losc_en(&mut self) -> EXT_LOSC_EN_W<4> {
        EXT_LOSC_EN_W::new(self)
    }
    #[doc = "Bit 7 - RTC DAY access\n\nAfter writing the RTC DAY register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC DAY register, the DAY register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_day_acce(&mut self) -> RTC_DAY_ACCE_W<7> {
        RTC_DAY_ACCE_W::new(self)
    }
    #[doc = "Bit 8 - RTC Hour Minute Second access\n\nAfter writing the RTC HH-MM-SS Register, this bit is set and it will be cleared until the real writing operation is finished. After writing the RTC HH-MM-SS Register, the RTC HH-MM-SS Register will be refreshed for at most one second.\n\nNote: Make sure that the bit is 0 for time configuration."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hhmmss_acce(&mut self) -> RTC_HHMMSS_ACCE_W<8> {
        RTC_HHMMSS_ACCE_W::new(self)
    }
    #[doc = "Bit 14 - LOSC auto switch 32K clk source select enable"]
    #[inline(always)]
    #[must_use]
    pub fn losc_auto_swt_32k_sel_en(&mut self) -> LOSC_AUTO_SWT_32K_SEL_EN_W<14> {
        LOSC_AUTO_SWT_32K_SEL_EN_W::new(self)
    }
    #[doc = "Bit 15 - LOSC auto switch function disable"]
    #[inline(always)]
    #[must_use]
    pub fn losc_auto_swt_function(&mut self) -> LOSC_AUTO_SWT_FUNCTION_W<15> {
        LOSC_AUTO_SWT_FUNCTION_W::new(self)
    }
    #[doc = "Bits 16:31 - Key Field\n\nThis field should be filled with 0x16AA, and then the bit0 and bit1 can be written with the new value."]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<16> {
        KEY_FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [losc_ctrl](index.html) module"]
pub struct LOSC_CTRL_SPEC;
impl crate::RegisterSpec for LOSC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [losc_ctrl::R](R) reader structure"]
impl crate::Readable for LOSC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [losc_ctrl::W](W) writer structure"]
impl crate::Writable for LOSC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets losc_ctrl to value 0x4010"]
impl crate::Resettable for LOSC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4010;
}
