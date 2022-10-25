#[doc = "Register `lradc_ctrl` reader"]
pub struct R(crate::R<LRADC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRADC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRADC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRADC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lradc_ctrl` writer"]
pub struct W(crate::W<LRADC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRADC_CTRL_SPEC>;
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
impl From<crate::W<LRADC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRADC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lradc_en` reader - LRADC Enable"]
pub type LRADC_EN_R = crate::BitReader<LRADC_EN_A>;
#[doc = "LRADC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRADC_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LRADC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LRADC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LRADC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRADC_EN_A {
        match self.bits {
            false => LRADC_EN_A::DISABLE,
            true => LRADC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LRADC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LRADC_EN_A::ENABLE
    }
}
#[doc = "Field `lradc_en` writer - LRADC Enable"]
pub type LRADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LRADC_CTRL_SPEC, LRADC_EN_A, O>;
impl<'a, const O: u8> LRADC_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LRADC_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LRADC_EN_A::ENABLE)
    }
}
#[doc = "Field `lradc_sample_rate` reader - LRADC Sample Rate"]
pub type LRADC_SAMPLE_RATE_R = crate::FieldReader<u8, LRADC_SAMPLE_RATE_A>;
#[doc = "LRADC Sample Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LRADC_SAMPLE_RATE_A {
    #[doc = "0: 2kHz"]
    R2K = 0,
    #[doc = "1: 1kHz"]
    R1K = 1,
    #[doc = "2: 500Hz"]
    R500 = 2,
    #[doc = "3: 250Hz"]
    R250 = 3,
}
impl From<LRADC_SAMPLE_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: LRADC_SAMPLE_RATE_A) -> Self {
        variant as _
    }
}
impl LRADC_SAMPLE_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRADC_SAMPLE_RATE_A {
        match self.bits {
            0 => LRADC_SAMPLE_RATE_A::R2K,
            1 => LRADC_SAMPLE_RATE_A::R1K,
            2 => LRADC_SAMPLE_RATE_A::R500,
            3 => LRADC_SAMPLE_RATE_A::R250,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `R2K`"]
    #[inline(always)]
    pub fn is_r2k(&self) -> bool {
        *self == LRADC_SAMPLE_RATE_A::R2K
    }
    #[doc = "Checks if the value of the field is `R1K`"]
    #[inline(always)]
    pub fn is_r1k(&self) -> bool {
        *self == LRADC_SAMPLE_RATE_A::R1K
    }
    #[doc = "Checks if the value of the field is `R500`"]
    #[inline(always)]
    pub fn is_r500(&self) -> bool {
        *self == LRADC_SAMPLE_RATE_A::R500
    }
    #[doc = "Checks if the value of the field is `R250`"]
    #[inline(always)]
    pub fn is_r250(&self) -> bool {
        *self == LRADC_SAMPLE_RATE_A::R250
    }
}
#[doc = "Field `lradc_sample_rate` writer - LRADC Sample Rate"]
pub type LRADC_SAMPLE_RATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LRADC_CTRL_SPEC, u8, LRADC_SAMPLE_RATE_A, 2, O>;
impl<'a, const O: u8> LRADC_SAMPLE_RATE_W<'a, O> {
    #[doc = "2kHz"]
    #[inline(always)]
    pub fn r2k(self) -> &'a mut W {
        self.variant(LRADC_SAMPLE_RATE_A::R2K)
    }
    #[doc = "1kHz"]
    #[inline(always)]
    pub fn r1k(self) -> &'a mut W {
        self.variant(LRADC_SAMPLE_RATE_A::R1K)
    }
    #[doc = "500Hz"]
    #[inline(always)]
    pub fn r500(self) -> &'a mut W {
        self.variant(LRADC_SAMPLE_RATE_A::R500)
    }
    #[doc = "250Hz"]
    #[inline(always)]
    pub fn r250(self) -> &'a mut W {
        self.variant(LRADC_SAMPLE_RATE_A::R250)
    }
}
#[doc = "Field `levelb_vol` reader - Level B Corresponding Data Value Setting (the real voltage value)"]
pub type LEVELB_VOL_R = crate::FieldReader<u8, LEVELB_VOL_A>;
#[doc = "Level B Corresponding Data Value Setting (the real voltage value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEVELB_VOL_A {
    #[doc = "1: 1.221 V"]
    V39 = 1,
    #[doc = "2: 1.157 V"]
    V36 = 2,
    #[doc = "3: 1.093 V"]
    V33 = 3,
}
impl From<LEVELB_VOL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEVELB_VOL_A) -> Self {
        variant as _
    }
}
impl LEVELB_VOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEVELB_VOL_A> {
        match self.bits {
            1 => Some(LEVELB_VOL_A::V39),
            2 => Some(LEVELB_VOL_A::V36),
            3 => Some(LEVELB_VOL_A::V33),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V39`"]
    #[inline(always)]
    pub fn is_v39(&self) -> bool {
        *self == LEVELB_VOL_A::V39
    }
    #[doc = "Checks if the value of the field is `V36`"]
    #[inline(always)]
    pub fn is_v36(&self) -> bool {
        *self == LEVELB_VOL_A::V36
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == LEVELB_VOL_A::V33
    }
}
#[doc = "Field `levelb_vol` writer - Level B Corresponding Data Value Setting (the real voltage value)"]
pub type LEVELB_VOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRADC_CTRL_SPEC, u8, LEVELB_VOL_A, 2, O>;
impl<'a, const O: u8> LEVELB_VOL_W<'a, O> {
    #[doc = "1.221 V"]
    #[inline(always)]
    pub fn v39(self) -> &'a mut W {
        self.variant(LEVELB_VOL_A::V39)
    }
    #[doc = "1.157 V"]
    #[inline(always)]
    pub fn v36(self) -> &'a mut W {
        self.variant(LEVELB_VOL_A::V36)
    }
    #[doc = "1.093 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut W {
        self.variant(LEVELB_VOL_A::V33)
    }
}
#[doc = "Field `lradc_channel_en` reader - LRADC Channel Enable"]
pub type LRADC_CHANNEL_EN_R = crate::BitReader<LRADC_CHANNEL_EN_A>;
#[doc = "LRADC Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRADC_CHANNEL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LRADC_CHANNEL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LRADC_CHANNEL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LRADC_CHANNEL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRADC_CHANNEL_EN_A {
        match self.bits {
            false => LRADC_CHANNEL_EN_A::DISABLE,
            true => LRADC_CHANNEL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LRADC_CHANNEL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LRADC_CHANNEL_EN_A::ENABLE
    }
}
#[doc = "Field `lradc_channel_en` writer - LRADC Channel Enable"]
pub type LRADC_CHANNEL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_CTRL_SPEC, LRADC_CHANNEL_EN_A, O>;
impl<'a, const O: u8> LRADC_CHANNEL_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LRADC_CHANNEL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LRADC_CHANNEL_EN_A::ENABLE)
    }
}
#[doc = "Field `lradc_hold_key_en` reader - LRADC Hold Key Enable"]
pub type LRADC_HOLD_KEY_EN_R = crate::BitReader<LRADC_HOLD_KEY_EN_A>;
#[doc = "LRADC Hold Key Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRADC_HOLD_KEY_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LRADC_HOLD_KEY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LRADC_HOLD_KEY_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LRADC_HOLD_KEY_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRADC_HOLD_KEY_EN_A {
        match self.bits {
            false => LRADC_HOLD_KEY_EN_A::DISABLE,
            true => LRADC_HOLD_KEY_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LRADC_HOLD_KEY_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LRADC_HOLD_KEY_EN_A::ENABLE
    }
}
#[doc = "Field `lradc_hold_key_en` writer - LRADC Hold Key Enable"]
pub type LRADC_HOLD_KEY_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LRADC_CTRL_SPEC, LRADC_HOLD_KEY_EN_A, O>;
impl<'a, const O: u8> LRADC_HOLD_KEY_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LRADC_HOLD_KEY_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LRADC_HOLD_KEY_EN_A::ENABLE)
    }
}
#[doc = "Field `levela_b_cnt` reader - Level A to B time threshold select"]
pub type LEVELA_B_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `levela_b_cnt` writer - Level A to B time threshold select"]
pub type LEVELA_B_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRADC_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `key_mode_select` reader - Key Mode Select"]
pub type KEY_MODE_SELECT_R = crate::FieldReader<u8, KEY_MODE_SELECT_A>;
#[doc = "Key Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_MODE_SELECT_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    SINGLE = 1,
    #[doc = "2: `10`"]
    CONTINUOUS = 2,
}
impl From<KEY_MODE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_MODE_SELECT_A) -> Self {
        variant as _
    }
}
impl KEY_MODE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_MODE_SELECT_A> {
        match self.bits {
            0 => Some(KEY_MODE_SELECT_A::NORMAL),
            1 => Some(KEY_MODE_SELECT_A::SINGLE),
            2 => Some(KEY_MODE_SELECT_A::CONTINUOUS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == KEY_MODE_SELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == KEY_MODE_SELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == KEY_MODE_SELECT_A::CONTINUOUS
    }
}
#[doc = "Field `key_mode_select` writer - Key Mode Select"]
pub type KEY_MODE_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRADC_CTRL_SPEC, u8, KEY_MODE_SELECT_A, 2, O>;
impl<'a, const O: u8> KEY_MODE_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(KEY_MODE_SELECT_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(KEY_MODE_SELECT_A::SINGLE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(KEY_MODE_SELECT_A::CONTINUOUS)
    }
}
#[doc = "Field `continue_time_select` reader - Continuous Mode Time Select"]
pub type CONTINUE_TIME_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `continue_time_select` writer - Continuous Mode Time Select"]
pub type CONTINUE_TIME_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRADC_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `first_convert_dly` reader - ADC First Convert Delay Setting"]
pub type FIRST_CONVERT_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `first_convert_dly` writer - ADC First Convert Delay Setting"]
pub type FIRST_CONVERT_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LRADC_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - LRADC Enable"]
    #[inline(always)]
    pub fn lradc_en(&self) -> LRADC_EN_R {
        LRADC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - LRADC Sample Rate"]
    #[inline(always)]
    pub fn lradc_sample_rate(&self) -> LRADC_SAMPLE_RATE_R {
        LRADC_SAMPLE_RATE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Level B Corresponding Data Value Setting (the real voltage value)"]
    #[inline(always)]
    pub fn levelb_vol(&self) -> LEVELB_VOL_R {
        LEVELB_VOL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - LRADC Channel Enable"]
    #[inline(always)]
    pub fn lradc_channel_en(&self) -> LRADC_CHANNEL_EN_R {
        LRADC_CHANNEL_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LRADC Hold Key Enable"]
    #[inline(always)]
    pub fn lradc_hold_key_en(&self) -> LRADC_HOLD_KEY_EN_R {
        LRADC_HOLD_KEY_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Level A to B time threshold select"]
    #[inline(always)]
    pub fn levela_b_cnt(&self) -> LEVELA_B_CNT_R {
        LEVELA_B_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Key Mode Select"]
    #[inline(always)]
    pub fn key_mode_select(&self) -> KEY_MODE_SELECT_R {
        KEY_MODE_SELECT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Continuous Mode Time Select"]
    #[inline(always)]
    pub fn continue_time_select(&self) -> CONTINUE_TIME_SELECT_R {
        CONTINUE_TIME_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - ADC First Convert Delay Setting"]
    #[inline(always)]
    pub fn first_convert_dly(&self) -> FIRST_CONVERT_DLY_R {
        FIRST_CONVERT_DLY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LRADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lradc_en(&mut self) -> LRADC_EN_W<0> {
        LRADC_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - LRADC Sample Rate"]
    #[inline(always)]
    #[must_use]
    pub fn lradc_sample_rate(&mut self) -> LRADC_SAMPLE_RATE_W<2> {
        LRADC_SAMPLE_RATE_W::new(self)
    }
    #[doc = "Bits 4:5 - Level B Corresponding Data Value Setting (the real voltage value)"]
    #[inline(always)]
    #[must_use]
    pub fn levelb_vol(&mut self) -> LEVELB_VOL_W<4> {
        LEVELB_VOL_W::new(self)
    }
    #[doc = "Bit 6 - LRADC Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lradc_channel_en(&mut self) -> LRADC_CHANNEL_EN_W<6> {
        LRADC_CHANNEL_EN_W::new(self)
    }
    #[doc = "Bit 7 - LRADC Hold Key Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lradc_hold_key_en(&mut self) -> LRADC_HOLD_KEY_EN_W<7> {
        LRADC_HOLD_KEY_EN_W::new(self)
    }
    #[doc = "Bits 8:11 - Level A to B time threshold select"]
    #[inline(always)]
    #[must_use]
    pub fn levela_b_cnt(&mut self) -> LEVELA_B_CNT_W<8> {
        LEVELA_B_CNT_W::new(self)
    }
    #[doc = "Bits 12:13 - Key Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn key_mode_select(&mut self) -> KEY_MODE_SELECT_W<12> {
        KEY_MODE_SELECT_W::new(self)
    }
    #[doc = "Bits 16:19 - Continuous Mode Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn continue_time_select(&mut self) -> CONTINUE_TIME_SELECT_W<16> {
        CONTINUE_TIME_SELECT_W::new(self)
    }
    #[doc = "Bits 24:31 - ADC First Convert Delay Setting"]
    #[inline(always)]
    #[must_use]
    pub fn first_convert_dly(&mut self) -> FIRST_CONVERT_DLY_W<24> {
        FIRST_CONVERT_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LRADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lradc_ctrl](index.html) module"]
pub struct LRADC_CTRL_SPEC;
impl crate::RegisterSpec for LRADC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lradc_ctrl::R](R) reader structure"]
impl crate::Readable for LRADC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lradc_ctrl::W](W) writer structure"]
impl crate::Writable for LRADC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lradc_ctrl to value 0"]
impl crate::Resettable for LRADC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
