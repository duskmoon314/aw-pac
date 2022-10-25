#[doc = "Register `tve_notch_dac_delay` reader"]
pub struct R(crate::R<TVE_NOTCH_DAC_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_NOTCH_DAC_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_NOTCH_DAC_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_NOTCH_DAC_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_notch_dac_delay` writer"]
pub struct W(crate::W<TVE_NOTCH_DAC_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_NOTCH_DAC_DELAY_SPEC>;
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
impl From<crate::W<TVE_NOTCH_DAC_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_NOTCH_DAC_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `c_delay_before_dither` reader - "]
pub type C_DELAY_BEFORE_DITHER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `c_delay_before_dither` writer - "]
pub type C_DELAY_BEFORE_DITHER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, u8, u8, 4, O>;
#[doc = "Field `notch_en` reader - Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed."]
pub type NOTCH_EN_R = crate::BitReader<NOTCH_EN_A>;
#[doc = "Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTCH_EN_A {
    #[doc = "0: The luma notch filter is bypassed"]
    BYPASSED = 0,
    #[doc = "1: The luma notch filter is operating"]
    OPERATING = 1,
}
impl From<NOTCH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NOTCH_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTCH_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTCH_EN_A {
        match self.bits {
            false => NOTCH_EN_A::BYPASSED,
            true => NOTCH_EN_A::OPERATING,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == NOTCH_EN_A::BYPASSED
    }
    #[doc = "Checks if the value of the field is `OPERATING`"]
    #[inline(always)]
    pub fn is_operating(&self) -> bool {
        *self == NOTCH_EN_A::OPERATING
    }
}
#[doc = "Field `notch_en` writer - Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed."]
pub type NOTCH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, NOTCH_EN_A, O>;
impl<'a, const O: u8> NOTCH_EN_W<'a, O> {
    #[doc = "The luma notch filter is bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(NOTCH_EN_A::BYPASSED)
    }
    #[doc = "The luma notch filter is operating"]
    #[inline(always)]
    pub fn operating(self) -> &'a mut W {
        self.variant(NOTCH_EN_A::OPERATING)
    }
}
#[doc = "Field `luma_filter_bypass` reader - "]
pub type LUMA_FILTER_BYPASS_R = crate::BitReader<LUMA_FILTER_BYPASS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUMA_FILTER_BYPASS_A {
    #[doc = "0: Luma Filter Enable"]
    ENABLE = 0,
    #[doc = "1: Luma Filter bypass"]
    BYPASS = 1,
}
impl From<LUMA_FILTER_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: LUMA_FILTER_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl LUMA_FILTER_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUMA_FILTER_BYPASS_A {
        match self.bits {
            false => LUMA_FILTER_BYPASS_A::ENABLE,
            true => LUMA_FILTER_BYPASS_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LUMA_FILTER_BYPASS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == LUMA_FILTER_BYPASS_A::BYPASS
    }
}
#[doc = "Field `luma_filter_bypass` writer - "]
pub type LUMA_FILTER_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, LUMA_FILTER_BYPASS_A, O>;
impl<'a, const O: u8> LUMA_FILTER_BYPASS_W<'a, O> {
    #[doc = "Luma Filter Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LUMA_FILTER_BYPASS_A::ENABLE)
    }
    #[doc = "Luma Filter bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(LUMA_FILTER_BYPASS_A::BYPASS)
    }
}
#[doc = "Field `chroma_filter_stage__bypass[3,2,1]` reader - "]
pub type CHROMA_FILTER_STAGE__BYPASS_R = crate::BitReader<CHROMA_FILTER_STAGE__BYPASS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHROMA_FILTER_STAGE__BYPASS_A {
    #[doc = "0: Chroma Filter Stage \\[i\\] Enable"]
    ENABLE = 0,
    #[doc = "1: Chroma Filter Stage \\[i\\] bypass"]
    BYPASS = 1,
}
impl From<CHROMA_FILTER_STAGE__BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CHROMA_FILTER_STAGE__BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl CHROMA_FILTER_STAGE__BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHROMA_FILTER_STAGE__BYPASS_A {
        match self.bits {
            false => CHROMA_FILTER_STAGE__BYPASS_A::ENABLE,
            true => CHROMA_FILTER_STAGE__BYPASS_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_FILTER_STAGE__BYPASS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CHROMA_FILTER_STAGE__BYPASS_A::BYPASS
    }
}
#[doc = "Field `chroma_filter_stage__bypass[3,2,1]` writer - "]
pub type CHROMA_FILTER_STAGE__BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, CHROMA_FILTER_STAGE__BYPASS_A, O>;
impl<'a, const O: u8> CHROMA_FILTER_STAGE__BYPASS_W<'a, O> {
    #[doc = "Chroma Filter Stage \\[i\\] Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHROMA_FILTER_STAGE__BYPASS_A::ENABLE)
    }
    #[doc = "Chroma Filter Stage \\[i\\] bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(CHROMA_FILTER_STAGE__BYPASS_A::BYPASS)
    }
}
#[doc = "Field `chroma_hd_mode_filter_en` reader - "]
pub type CHROMA_HD_MODE_FILTER_EN_R = crate::BitReader<CHROMA_HD_MODE_FILTER_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHROMA_HD_MODE_FILTER_EN_A {
    #[doc = "0: Chroma HD Filter Disable"]
    DISABLE = 0,
    #[doc = "1: Chroma HD Filter Enable"]
    ENABLE = 1,
}
impl From<CHROMA_HD_MODE_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CHROMA_HD_MODE_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHROMA_HD_MODE_FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHROMA_HD_MODE_FILTER_EN_A {
        match self.bits {
            false => CHROMA_HD_MODE_FILTER_EN_A::DISABLE,
            true => CHROMA_HD_MODE_FILTER_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHROMA_HD_MODE_FILTER_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_HD_MODE_FILTER_EN_A::ENABLE
    }
}
#[doc = "Field `chroma_hd_mode_filter_en` writer - "]
pub type CHROMA_HD_MODE_FILTER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, CHROMA_HD_MODE_FILTER_EN_A, O>;
impl<'a, const O: u8> CHROMA_HD_MODE_FILTER_EN_W<'a, O> {
    #[doc = "Chroma HD Filter Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHROMA_HD_MODE_FILTER_EN_A::DISABLE)
    }
    #[doc = "Chroma HD Filter Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHROMA_HD_MODE_FILTER_EN_A::ENABLE)
    }
}
#[doc = "Field `chroma_filter_1_444_en` reader - "]
pub type CHROMA_FILTER_1_444_EN_R = crate::BitReader<CHROMA_FILTER_1_444_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHROMA_FILTER_1_444_EN_A {
    #[doc = "0: Chroma Filter 1 444 Disable"]
    DISABLE = 0,
    #[doc = "1: Chroma Filter 1 444 Enable"]
    ENABLE = 1,
}
impl From<CHROMA_FILTER_1_444_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CHROMA_FILTER_1_444_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHROMA_FILTER_1_444_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHROMA_FILTER_1_444_EN_A {
        match self.bits {
            false => CHROMA_FILTER_1_444_EN_A::DISABLE,
            true => CHROMA_FILTER_1_444_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHROMA_FILTER_1_444_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_FILTER_1_444_EN_A::ENABLE
    }
}
#[doc = "Field `chroma_filter_1_444_en` writer - "]
pub type CHROMA_FILTER_1_444_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, CHROMA_FILTER_1_444_EN_A, O>;
impl<'a, const O: u8> CHROMA_FILTER_1_444_EN_W<'a, O> {
    #[doc = "Chroma Filter 1 444 Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHROMA_FILTER_1_444_EN_A::DISABLE)
    }
    #[doc = "Chroma Filter 1 444 Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHROMA_FILTER_1_444_EN_A::ENABLE)
    }
}
#[doc = "Field `hd_mode_cr_filter_bypass` reader - "]
pub type HD_MODE_CR_FILTER_BYPASS_R = crate::BitReader<HD_MODE_CR_FILTER_BYPASS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HD_MODE_CR_FILTER_BYPASS_A {
    #[doc = "0: Bypass Enable"]
    ENABLE = 0,
    #[doc = "1: Bypass Disable"]
    DISABLE = 1,
}
impl From<HD_MODE_CR_FILTER_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: HD_MODE_CR_FILTER_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl HD_MODE_CR_FILTER_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HD_MODE_CR_FILTER_BYPASS_A {
        match self.bits {
            false => HD_MODE_CR_FILTER_BYPASS_A::ENABLE,
            true => HD_MODE_CR_FILTER_BYPASS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HD_MODE_CR_FILTER_BYPASS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HD_MODE_CR_FILTER_BYPASS_A::DISABLE
    }
}
#[doc = "Field `hd_mode_cr_filter_bypass` writer - "]
pub type HD_MODE_CR_FILTER_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, HD_MODE_CR_FILTER_BYPASS_A, O>;
impl<'a, const O: u8> HD_MODE_CR_FILTER_BYPASS_W<'a, O> {
    #[doc = "Bypass Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HD_MODE_CR_FILTER_BYPASS_A::ENABLE)
    }
    #[doc = "Bypass Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HD_MODE_CR_FILTER_BYPASS_A::DISABLE)
    }
}
#[doc = "Field `hd_mode_cb_filter_bypass` reader - "]
pub type HD_MODE_CB_FILTER_BYPASS_R = crate::BitReader<HD_MODE_CB_FILTER_BYPASS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HD_MODE_CB_FILTER_BYPASS_A {
    #[doc = "0: Bypass Enable"]
    ENABLE = 0,
    #[doc = "1: Bypass Disable"]
    DISABLE = 1,
}
impl From<HD_MODE_CB_FILTER_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: HD_MODE_CB_FILTER_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl HD_MODE_CB_FILTER_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HD_MODE_CB_FILTER_BYPASS_A {
        match self.bits {
            false => HD_MODE_CB_FILTER_BYPASS_A::ENABLE,
            true => HD_MODE_CB_FILTER_BYPASS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HD_MODE_CB_FILTER_BYPASS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HD_MODE_CB_FILTER_BYPASS_A::DISABLE
    }
}
#[doc = "Field `hd_mode_cb_filter_bypass` writer - "]
pub type HD_MODE_CB_FILTER_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, HD_MODE_CB_FILTER_BYPASS_A, O>;
impl<'a, const O: u8> HD_MODE_CB_FILTER_BYPASS_W<'a, O> {
    #[doc = "Bypass Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HD_MODE_CB_FILTER_BYPASS_A::ENABLE)
    }
    #[doc = "Bypass Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HD_MODE_CB_FILTER_BYPASS_A::DISABLE)
    }
}
#[doc = "Field `y_delay_before_dither` reader - "]
pub type Y_DELAY_BEFORE_DITHER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `y_delay_before_dither` writer - "]
pub type Y_DELAY_BEFORE_DITHER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, u8, u8, 3, O>;
#[doc = "Field `luma_filter_lti_enable` reader - "]
pub type LUMA_FILTER_LTI_ENABLE_R = crate::BitReader<LUMA_FILTER_LTI_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUMA_FILTER_LTI_ENABLE_A {
    #[doc = "0: Disable Luma filter lti"]
    DISABLE = 0,
    #[doc = "1: Enable Luma filter lti"]
    ENABLE = 1,
}
impl From<LUMA_FILTER_LTI_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LUMA_FILTER_LTI_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LUMA_FILTER_LTI_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUMA_FILTER_LTI_ENABLE_A {
        match self.bits {
            false => LUMA_FILTER_LTI_ENABLE_A::DISABLE,
            true => LUMA_FILTER_LTI_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LUMA_FILTER_LTI_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LUMA_FILTER_LTI_ENABLE_A::ENABLE
    }
}
#[doc = "Field `luma_filter_lti_enable` writer - "]
pub type LUMA_FILTER_LTI_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, LUMA_FILTER_LTI_ENABLE_A, O>;
impl<'a, const O: u8> LUMA_FILTER_LTI_ENABLE_W<'a, O> {
    #[doc = "Disable Luma filter lti"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LUMA_FILTER_LTI_ENABLE_A::DISABLE)
    }
    #[doc = "Enable Luma filter lti"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LUMA_FILTER_LTI_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `chroma_filter_active_valid` reader - "]
pub type CHROMA_FILTER_ACTIVE_VALID_R = crate::BitReader<CHROMA_FILTER_ACTIVE_VALID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHROMA_FILTER_ACTIVE_VALID_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CHROMA_FILTER_ACTIVE_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: CHROMA_FILTER_ACTIVE_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl CHROMA_FILTER_ACTIVE_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHROMA_FILTER_ACTIVE_VALID_A {
        match self.bits {
            false => CHROMA_FILTER_ACTIVE_VALID_A::DISABLE,
            true => CHROMA_FILTER_ACTIVE_VALID_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHROMA_FILTER_ACTIVE_VALID_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_FILTER_ACTIVE_VALID_A::ENABLE
    }
}
#[doc = "Field `chroma_filter_active_valid` writer - "]
pub type CHROMA_FILTER_ACTIVE_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_NOTCH_DAC_DELAY_SPEC, CHROMA_FILTER_ACTIVE_VALID_A, O>;
impl<'a, const O: u8> CHROMA_FILTER_ACTIVE_VALID_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHROMA_FILTER_ACTIVE_VALID_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHROMA_FILTER_ACTIVE_VALID_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn c_delay_before_dither(&self) -> C_DELAY_BEFORE_DITHER_R {
        C_DELAY_BEFORE_DITHER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed."]
    #[inline(always)]
    pub fn notch_en(&self) -> NOTCH_EN_R {
        NOTCH_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn luma_filter_bypass(&self) -> LUMA_FILTER_BYPASS_R {
        LUMA_FILTER_BYPASS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn chroma_filter_stage_3_bypass(&self) -> CHROMA_FILTER_STAGE__BYPASS_R {
        CHROMA_FILTER_STAGE__BYPASS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn chroma_filter_stage_2_bypass(&self) -> CHROMA_FILTER_STAGE__BYPASS_R {
        CHROMA_FILTER_STAGE__BYPASS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn chroma_filter_stage_1_bypass(&self) -> CHROMA_FILTER_STAGE__BYPASS_R {
        CHROMA_FILTER_STAGE__BYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn chroma_hd_mode_filter_en(&self) -> CHROMA_HD_MODE_FILTER_EN_R {
        CHROMA_HD_MODE_FILTER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn chroma_filter_1_444_en(&self) -> CHROMA_FILTER_1_444_EN_R {
        CHROMA_FILTER_1_444_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn hd_mode_cr_filter_bypass(&self) -> HD_MODE_CR_FILTER_BYPASS_R {
        HD_MODE_CR_FILTER_BYPASS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn hd_mode_cb_filter_bypass(&self) -> HD_MODE_CB_FILTER_BYPASS_R {
        HD_MODE_CB_FILTER_BYPASS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn y_delay_before_dither(&self) -> Y_DELAY_BEFORE_DITHER_R {
        Y_DELAY_BEFORE_DITHER_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn luma_filter_lti_enable(&self) -> LUMA_FILTER_LTI_ENABLE_R {
        LUMA_FILTER_LTI_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn chroma_filter_active_valid(&self) -> CHROMA_FILTER_ACTIVE_VALID_R {
        CHROMA_FILTER_ACTIVE_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn c_delay_before_dither(&mut self) -> C_DELAY_BEFORE_DITHER_W<12> {
        C_DELAY_BEFORE_DITHER_W::new(self)
    }
    #[doc = "Bit 16 - Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn notch_en(&mut self) -> NOTCH_EN_W<16> {
        NOTCH_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn luma_filter_bypass(&mut self) -> LUMA_FILTER_BYPASS_W<17> {
        LUMA_FILTER_BYPASS_W::new(self)
    }
    #[doc = ""]
    #[inline(always)]
    #[must_use]
    pub unsafe fn chroma_filter_stage__bypass<const O: u8>(
        &mut self,
    ) -> CHROMA_FILTER_STAGE__BYPASS_W<O> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage_3_bypass(&mut self) -> CHROMA_FILTER_STAGE__BYPASS_W<18> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage_2_bypass(&mut self) -> CHROMA_FILTER_STAGE__BYPASS_W<19> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage_1_bypass(&mut self) -> CHROMA_FILTER_STAGE__BYPASS_W<20> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_hd_mode_filter_en(&mut self) -> CHROMA_HD_MODE_FILTER_EN_W<21> {
        CHROMA_HD_MODE_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_1_444_en(&mut self) -> CHROMA_FILTER_1_444_EN_W<22> {
        CHROMA_FILTER_1_444_EN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn hd_mode_cr_filter_bypass(&mut self) -> HD_MODE_CR_FILTER_BYPASS_W<23> {
        HD_MODE_CR_FILTER_BYPASS_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn hd_mode_cb_filter_bypass(&mut self) -> HD_MODE_CB_FILTER_BYPASS_W<24> {
        HD_MODE_CB_FILTER_BYPASS_W::new(self)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn y_delay_before_dither(&mut self) -> Y_DELAY_BEFORE_DITHER_W<25> {
        Y_DELAY_BEFORE_DITHER_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn luma_filter_lti_enable(&mut self) -> LUMA_FILTER_LTI_ENABLE_W<30> {
        LUMA_FILTER_LTI_ENABLE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_active_valid(&mut self) -> CHROMA_FILTER_ACTIVE_VALID_W<31> {
        CHROMA_FILTER_ACTIVE_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Notch and DAC Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_notch_dac_delay](index.html) module"]
pub struct TVE_NOTCH_DAC_DELAY_SPEC;
impl crate::RegisterSpec for TVE_NOTCH_DAC_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_notch_dac_delay::R](R) reader structure"]
impl crate::Readable for TVE_NOTCH_DAC_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_notch_dac_delay::W](W) writer structure"]
impl crate::Writable for TVE_NOTCH_DAC_DELAY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_notch_dac_delay to value 0x0201_4924"]
impl crate::Resettable for TVE_NOTCH_DAC_DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0201_4924;
}
