#[doc = "Register `tve_notch_dac_delay` reader"]
pub type R = crate::R<TVE_NOTCH_DAC_DELAY_SPEC>;
#[doc = "Register `tve_notch_dac_delay` writer"]
pub type W = crate::W<TVE_NOTCH_DAC_DELAY_SPEC>;
#[doc = "Field `c_delay_before_dither` reader - "]
pub type C_DELAY_BEFORE_DITHER_R = crate::FieldReader;
#[doc = "Field `c_delay_before_dither` writer - "]
pub type C_DELAY_BEFORE_DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    pub const fn variant(&self) -> NOTCH_EN_A {
        match self.bits {
            false => NOTCH_EN_A::BYPASSED,
            true => NOTCH_EN_A::OPERATING,
        }
    }
    #[doc = "The luma notch filter is bypassed"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == NOTCH_EN_A::BYPASSED
    }
    #[doc = "The luma notch filter is operating"]
    #[inline(always)]
    pub fn is_operating(&self) -> bool {
        *self == NOTCH_EN_A::OPERATING
    }
}
#[doc = "Field `notch_en` writer - Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed."]
pub type NOTCH_EN_W<'a, REG> = crate::BitWriter<'a, REG, NOTCH_EN_A>;
impl<'a, REG> NOTCH_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The luma notch filter is bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_EN_A::BYPASSED)
    }
    #[doc = "The luma notch filter is operating"]
    #[inline(always)]
    pub fn operating(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> LUMA_FILTER_BYPASS_A {
        match self.bits {
            false => LUMA_FILTER_BYPASS_A::ENABLE,
            true => LUMA_FILTER_BYPASS_A::BYPASS,
        }
    }
    #[doc = "Luma Filter Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LUMA_FILTER_BYPASS_A::ENABLE
    }
    #[doc = "Luma Filter bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == LUMA_FILTER_BYPASS_A::BYPASS
    }
}
#[doc = "Field `luma_filter_bypass` writer - "]
pub type LUMA_FILTER_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, LUMA_FILTER_BYPASS_A>;
impl<'a, REG> LUMA_FILTER_BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Luma Filter Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LUMA_FILTER_BYPASS_A::ENABLE)
    }
    #[doc = "Luma Filter bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CHROMA_FILTER_STAGE__BYPASS_A {
        match self.bits {
            false => CHROMA_FILTER_STAGE__BYPASS_A::ENABLE,
            true => CHROMA_FILTER_STAGE__BYPASS_A::BYPASS,
        }
    }
    #[doc = "Chroma Filter Stage \\[i\\] Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_FILTER_STAGE__BYPASS_A::ENABLE
    }
    #[doc = "Chroma Filter Stage \\[i\\] bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CHROMA_FILTER_STAGE__BYPASS_A::BYPASS
    }
}
#[doc = "Field `chroma_filter_stage__bypass[3,2,1]` writer - "]
pub type CHROMA_FILTER_STAGE__BYPASS_W<'a, REG> =
    crate::BitWriter<'a, REG, CHROMA_FILTER_STAGE__BYPASS_A>;
impl<'a, REG> CHROMA_FILTER_STAGE__BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Chroma Filter Stage \\[i\\] Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_FILTER_STAGE__BYPASS_A::ENABLE)
    }
    #[doc = "Chroma Filter Stage \\[i\\] bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CHROMA_HD_MODE_FILTER_EN_A {
        match self.bits {
            false => CHROMA_HD_MODE_FILTER_EN_A::DISABLE,
            true => CHROMA_HD_MODE_FILTER_EN_A::ENABLE,
        }
    }
    #[doc = "Chroma HD Filter Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHROMA_HD_MODE_FILTER_EN_A::DISABLE
    }
    #[doc = "Chroma HD Filter Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_HD_MODE_FILTER_EN_A::ENABLE
    }
}
#[doc = "Field `chroma_hd_mode_filter_en` writer - "]
pub type CHROMA_HD_MODE_FILTER_EN_W<'a, REG> =
    crate::BitWriter<'a, REG, CHROMA_HD_MODE_FILTER_EN_A>;
impl<'a, REG> CHROMA_HD_MODE_FILTER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Chroma HD Filter Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_HD_MODE_FILTER_EN_A::DISABLE)
    }
    #[doc = "Chroma HD Filter Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CHROMA_FILTER_1_444_EN_A {
        match self.bits {
            false => CHROMA_FILTER_1_444_EN_A::DISABLE,
            true => CHROMA_FILTER_1_444_EN_A::ENABLE,
        }
    }
    #[doc = "Chroma Filter 1 444 Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHROMA_FILTER_1_444_EN_A::DISABLE
    }
    #[doc = "Chroma Filter 1 444 Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_FILTER_1_444_EN_A::ENABLE
    }
}
#[doc = "Field `chroma_filter_1_444_en` writer - "]
pub type CHROMA_FILTER_1_444_EN_W<'a, REG> = crate::BitWriter<'a, REG, CHROMA_FILTER_1_444_EN_A>;
impl<'a, REG> CHROMA_FILTER_1_444_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Chroma Filter 1 444 Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_FILTER_1_444_EN_A::DISABLE)
    }
    #[doc = "Chroma Filter 1 444 Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> HD_MODE_CR_FILTER_BYPASS_A {
        match self.bits {
            false => HD_MODE_CR_FILTER_BYPASS_A::ENABLE,
            true => HD_MODE_CR_FILTER_BYPASS_A::DISABLE,
        }
    }
    #[doc = "Bypass Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HD_MODE_CR_FILTER_BYPASS_A::ENABLE
    }
    #[doc = "Bypass Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HD_MODE_CR_FILTER_BYPASS_A::DISABLE
    }
}
#[doc = "Field `hd_mode_cr_filter_bypass` writer - "]
pub type HD_MODE_CR_FILTER_BYPASS_W<'a, REG> =
    crate::BitWriter<'a, REG, HD_MODE_CR_FILTER_BYPASS_A>;
impl<'a, REG> HD_MODE_CR_FILTER_BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HD_MODE_CR_FILTER_BYPASS_A::ENABLE)
    }
    #[doc = "Bypass Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> HD_MODE_CB_FILTER_BYPASS_A {
        match self.bits {
            false => HD_MODE_CB_FILTER_BYPASS_A::ENABLE,
            true => HD_MODE_CB_FILTER_BYPASS_A::DISABLE,
        }
    }
    #[doc = "Bypass Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HD_MODE_CB_FILTER_BYPASS_A::ENABLE
    }
    #[doc = "Bypass Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HD_MODE_CB_FILTER_BYPASS_A::DISABLE
    }
}
#[doc = "Field `hd_mode_cb_filter_bypass` writer - "]
pub type HD_MODE_CB_FILTER_BYPASS_W<'a, REG> =
    crate::BitWriter<'a, REG, HD_MODE_CB_FILTER_BYPASS_A>;
impl<'a, REG> HD_MODE_CB_FILTER_BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HD_MODE_CB_FILTER_BYPASS_A::ENABLE)
    }
    #[doc = "Bypass Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HD_MODE_CB_FILTER_BYPASS_A::DISABLE)
    }
}
#[doc = "Field `y_delay_before_dither` reader - "]
pub type Y_DELAY_BEFORE_DITHER_R = crate::FieldReader;
#[doc = "Field `y_delay_before_dither` writer - "]
pub type Y_DELAY_BEFORE_DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    pub const fn variant(&self) -> LUMA_FILTER_LTI_ENABLE_A {
        match self.bits {
            false => LUMA_FILTER_LTI_ENABLE_A::DISABLE,
            true => LUMA_FILTER_LTI_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable Luma filter lti"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LUMA_FILTER_LTI_ENABLE_A::DISABLE
    }
    #[doc = "Enable Luma filter lti"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LUMA_FILTER_LTI_ENABLE_A::ENABLE
    }
}
#[doc = "Field `luma_filter_lti_enable` writer - "]
pub type LUMA_FILTER_LTI_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, LUMA_FILTER_LTI_ENABLE_A>;
impl<'a, REG> LUMA_FILTER_LTI_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Luma filter lti"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LUMA_FILTER_LTI_ENABLE_A::DISABLE)
    }
    #[doc = "Enable Luma filter lti"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CHROMA_FILTER_ACTIVE_VALID_A {
        match self.bits {
            false => CHROMA_FILTER_ACTIVE_VALID_A::DISABLE,
            true => CHROMA_FILTER_ACTIVE_VALID_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHROMA_FILTER_ACTIVE_VALID_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHROMA_FILTER_ACTIVE_VALID_A::ENABLE
    }
}
#[doc = "Field `chroma_filter_active_valid` writer - "]
pub type CHROMA_FILTER_ACTIVE_VALID_W<'a, REG> =
    crate::BitWriter<'a, REG, CHROMA_FILTER_ACTIVE_VALID_A>;
impl<'a, REG> CHROMA_FILTER_ACTIVE_VALID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_FILTER_ACTIVE_VALID_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `chroma_filter_stage_3_bypass` field"]
    #[inline(always)]
    pub fn chroma_filter_stage__bypass(&self, n: u8) -> CHROMA_FILTER_STAGE__BYPASS_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CHROMA_FILTER_STAGE__BYPASS_R::new(((self.bits >> (n + 18)) & 1) != 0)
    }
    #[doc = "Bit 18 - chroma_filter_stage_3_bypass"]
    #[inline(always)]
    pub fn chroma_filter_stage_3_bypass(&self) -> CHROMA_FILTER_STAGE__BYPASS_R {
        CHROMA_FILTER_STAGE__BYPASS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - chroma_filter_stage_2_bypass"]
    #[inline(always)]
    pub fn chroma_filter_stage_2_bypass(&self) -> CHROMA_FILTER_STAGE__BYPASS_R {
        CHROMA_FILTER_STAGE__BYPASS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - chroma_filter_stage_1_bypass"]
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
    pub fn c_delay_before_dither(&mut self) -> C_DELAY_BEFORE_DITHER_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        C_DELAY_BEFORE_DITHER_W::new(self, 12)
    }
    #[doc = "Bit 16 - Luma notch filter on/off selection\n\nNote: This bit selects if the luma notch filter is operating or bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn notch_en(&mut self) -> NOTCH_EN_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        NOTCH_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn luma_filter_bypass(&mut self) -> LUMA_FILTER_BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        LUMA_FILTER_BYPASS_W::new(self, 17)
    }
    #[doc = "\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `chroma_filter_stage_3_bypass` field"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage__bypass(
        &mut self,
        n: u8,
    ) -> CHROMA_FILTER_STAGE__BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CHROMA_FILTER_STAGE__BYPASS_W::new(self, n + 18)
    }
    #[doc = "Bit 18 - chroma_filter_stage_3_bypass"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage_3_bypass(
        &mut self,
    ) -> CHROMA_FILTER_STAGE__BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self, 18)
    }
    #[doc = "Bit 19 - chroma_filter_stage_2_bypass"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage_2_bypass(
        &mut self,
    ) -> CHROMA_FILTER_STAGE__BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self, 19)
    }
    #[doc = "Bit 20 - chroma_filter_stage_1_bypass"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_stage_1_bypass(
        &mut self,
    ) -> CHROMA_FILTER_STAGE__BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        CHROMA_FILTER_STAGE__BYPASS_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_hd_mode_filter_en(
        &mut self,
    ) -> CHROMA_HD_MODE_FILTER_EN_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        CHROMA_HD_MODE_FILTER_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_1_444_en(&mut self) -> CHROMA_FILTER_1_444_EN_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        CHROMA_FILTER_1_444_EN_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn hd_mode_cr_filter_bypass(
        &mut self,
    ) -> HD_MODE_CR_FILTER_BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        HD_MODE_CR_FILTER_BYPASS_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn hd_mode_cb_filter_bypass(
        &mut self,
    ) -> HD_MODE_CB_FILTER_BYPASS_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        HD_MODE_CB_FILTER_BYPASS_W::new(self, 24)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn y_delay_before_dither(&mut self) -> Y_DELAY_BEFORE_DITHER_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        Y_DELAY_BEFORE_DITHER_W::new(self, 25)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn luma_filter_lti_enable(&mut self) -> LUMA_FILTER_LTI_ENABLE_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        LUMA_FILTER_LTI_ENABLE_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_filter_active_valid(
        &mut self,
    ) -> CHROMA_FILTER_ACTIVE_VALID_W<TVE_NOTCH_DAC_DELAY_SPEC> {
        CHROMA_FILTER_ACTIVE_VALID_W::new(self, 31)
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
#[doc = "TV Encoder Notch and DAC Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_notch_dac_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_notch_dac_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_NOTCH_DAC_DELAY_SPEC;
impl crate::RegisterSpec for TVE_NOTCH_DAC_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_notch_dac_delay::R`](R) reader structure"]
impl crate::Readable for TVE_NOTCH_DAC_DELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_notch_dac_delay::W`](W) writer structure"]
impl crate::Writable for TVE_NOTCH_DAC_DELAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_notch_dac_delay to value 0x0201_4924"]
impl crate::Resettable for TVE_NOTCH_DAC_DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0201_4924;
}
