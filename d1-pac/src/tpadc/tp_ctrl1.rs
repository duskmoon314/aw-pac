#[doc = "Register `tp_ctrl1` reader"]
pub struct R(crate::R<TP_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tp_ctrl1` writer"]
pub struct W(crate::W<TP_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CTRL1_SPEC>;
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
impl From<crate::W<TP_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_chan_select[0-3]` reader - Analog Input Channel Select"]
pub type ADC_CHAN_SELECT_R = crate::BitReader<ADC_CHAN_SELECT_A>;
#[doc = "Analog Input Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_CHAN_SELECT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC_CHAN_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CHAN_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_CHAN_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_CHAN_SELECT_A {
        match self.bits {
            false => ADC_CHAN_SELECT_A::DISABLE,
            true => ADC_CHAN_SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_CHAN_SELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_CHAN_SELECT_A::ENABLE
    }
}
#[doc = "Field `adc_chan_select[0-3]` writer - Analog Input Channel Select"]
pub type ADC_CHAN_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_CTRL1_SPEC, ADC_CHAN_SELECT_A, O>;
impl<'a, const O: u8> ADC_CHAN_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_CHAN_SELECT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_CHAN_SELECT_A::ENABLE)
    }
}
#[doc = "Field `tp_mode_select` reader - Touch Panel Mode and Auxiliary ADC Mode Select"]
pub type TP_MODE_SELECT_R = crate::BitReader<TP_MODE_SELECT_A>;
#[doc = "Touch Panel Mode and Auxiliary ADC Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_MODE_SELECT_A {
    #[doc = "0: `0`"]
    TP = 0,
    #[doc = "1: `1`"]
    AUXILIARY_ADC = 1,
}
impl From<TP_MODE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TP_MODE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_MODE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_MODE_SELECT_A {
        match self.bits {
            false => TP_MODE_SELECT_A::TP,
            true => TP_MODE_SELECT_A::AUXILIARY_ADC,
        }
    }
    #[doc = "Checks if the value of the field is `TP`"]
    #[inline(always)]
    pub fn is_tp(&self) -> bool {
        *self == TP_MODE_SELECT_A::TP
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_ADC`"]
    #[inline(always)]
    pub fn is_auxiliary_adc(&self) -> bool {
        *self == TP_MODE_SELECT_A::AUXILIARY_ADC
    }
}
#[doc = "Field `tp_mode_select` writer - Touch Panel Mode and Auxiliary ADC Mode Select"]
pub type TP_MODE_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_CTRL1_SPEC, TP_MODE_SELECT_A, O>;
impl<'a, const O: u8> TP_MODE_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tp(self) -> &'a mut W {
        self.variant(TP_MODE_SELECT_A::TP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn auxiliary_adc(self) -> &'a mut W {
        self.variant(TP_MODE_SELECT_A::AUXILIARY_ADC)
    }
}
#[doc = "Field `tp_en` reader - TP Function Enable"]
pub type TP_EN_R = crate::BitReader<TP_EN_A>;
#[doc = "TP Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_EN_A {
        match self.bits {
            false => TP_EN_A::DISABLE,
            true => TP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_EN_A::ENABLE
    }
}
#[doc = "Field `tp_en` writer - TP Function Enable"]
pub type TP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TP_CTRL1_SPEC, TP_EN_A, O>;
impl<'a, const O: u8> TP_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_EN_A::ENABLE)
    }
}
#[doc = "Field `tp_dual_en` reader - Touch Panel Double Point Enable"]
pub type TP_DUAL_EN_R = crate::BitReader<TP_DUAL_EN_A>;
#[doc = "Touch Panel Double Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_DUAL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DUAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DUAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_DUAL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DUAL_EN_A {
        match self.bits {
            false => TP_DUAL_EN_A::DISABLE,
            true => TP_DUAL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_DUAL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_DUAL_EN_A::ENABLE
    }
}
#[doc = "Field `tp_dual_en` writer - Touch Panel Double Point Enable"]
pub type TP_DUAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TP_CTRL1_SPEC, TP_DUAL_EN_A, O>;
impl<'a, const O: u8> TP_DUAL_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DUAL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DUAL_EN_A::ENABLE)
    }
}
#[doc = "Field `touch_pan_cali_en` reader - Touch Panel Calibration"]
pub type TOUCH_PAN_CALI_EN_R = crate::BitReader<TOUCH_PAN_CALI_EN_A>;
#[doc = "Touch Panel Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOUCH_PAN_CALI_EN_A {
    #[doc = "1: `1`"]
    START = 1,
}
impl From<TOUCH_PAN_CALI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TOUCH_PAN_CALI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TOUCH_PAN_CALI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOUCH_PAN_CALI_EN_A> {
        match self.bits {
            true => Some(TOUCH_PAN_CALI_EN_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == TOUCH_PAN_CALI_EN_A::START
    }
}
#[doc = "Field `touch_pan_cali_en` writer - Touch Panel Calibration"]
pub type TOUCH_PAN_CALI_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_CTRL1_SPEC, TOUCH_PAN_CALI_EN_A, O>;
impl<'a, const O: u8> TOUCH_PAN_CALI_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TOUCH_PAN_CALI_EN_A::START)
    }
}
#[doc = "Field `chopper_en` reader - T-sensor Chopping Enable"]
pub type CHOPPER_EN_R = crate::BitReader<CHOPPER_EN_A>;
#[doc = "T-sensor Chopping Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHOPPER_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CHOPPER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CHOPPER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHOPPER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHOPPER_EN_A {
        match self.bits {
            false => CHOPPER_EN_A::DISABLE,
            true => CHOPPER_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHOPPER_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHOPPER_EN_A::ENABLE
    }
}
#[doc = "Field `chopper_en` writer - T-sensor Chopping Enable"]
pub type CHOPPER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TP_CTRL1_SPEC, CHOPPER_EN_A, O>;
impl<'a, const O: u8> CHOPPER_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHOPPER_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHOPPER_EN_A::ENABLE)
    }
}
#[doc = "Field `stylus_up_debounce_en` reader - Stylus Up Debounce Function Select"]
pub type STYLUS_UP_DEBOUNCE_EN_R = crate::BitReader<STYLUS_UP_DEBOUNCE_EN_A>;
#[doc = "Stylus Up Debounce Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STYLUS_UP_DEBOUNCE_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STYLUS_UP_DEBOUNCE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: STYLUS_UP_DEBOUNCE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl STYLUS_UP_DEBOUNCE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STYLUS_UP_DEBOUNCE_EN_A {
        match self.bits {
            false => STYLUS_UP_DEBOUNCE_EN_A::DISABLE,
            true => STYLUS_UP_DEBOUNCE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STYLUS_UP_DEBOUNCE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STYLUS_UP_DEBOUNCE_EN_A::ENABLE
    }
}
#[doc = "Field `stylus_up_debounce_en` writer - Stylus Up Debounce Function Select"]
pub type STYLUS_UP_DEBOUNCE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_CTRL1_SPEC, STYLUS_UP_DEBOUNCE_EN_A, O>;
impl<'a, const O: u8> STYLUS_UP_DEBOUNCE_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STYLUS_UP_DEBOUNCE_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STYLUS_UP_DEBOUNCE_EN_A::ENABLE)
    }
}
#[doc = "Field `stylus_up_debounce` reader - Stylus Up Debounce Time Setting"]
pub type STYLUS_UP_DEBOUNCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `stylus_up_debounce` writer - Stylus Up Debounce Time Setting"]
pub type STYLUS_UP_DEBOUNCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TP_CTRL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Analog Input Channel Select"]
    #[inline(always)]
    pub unsafe fn adc_chan_select(&self, n: u8) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan0_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan1_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan2_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan3_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Touch Panel Mode and Auxiliary ADC Mode Select"]
    #[inline(always)]
    pub fn tp_mode_select(&self) -> TP_MODE_SELECT_R {
        TP_MODE_SELECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TP Function Enable"]
    #[inline(always)]
    pub fn tp_en(&self) -> TP_EN_R {
        TP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Touch Panel Double Point Enable"]
    #[inline(always)]
    pub fn tp_dual_en(&self) -> TP_DUAL_EN_R {
        TP_DUAL_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Touch Panel Calibration"]
    #[inline(always)]
    pub fn touch_pan_cali_en(&self) -> TOUCH_PAN_CALI_EN_R {
        TOUCH_PAN_CALI_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - T-sensor Chopping Enable"]
    #[inline(always)]
    pub fn chopper_en(&self) -> CHOPPER_EN_R {
        CHOPPER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stylus Up Debounce Function Select"]
    #[inline(always)]
    pub fn stylus_up_debounce_en(&self) -> STYLUS_UP_DEBOUNCE_EN_R {
        STYLUS_UP_DEBOUNCE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:19 - Stylus Up Debounce Time Setting"]
    #[inline(always)]
    pub fn stylus_up_debounce(&self) -> STYLUS_UP_DEBOUNCE_R {
        STYLUS_UP_DEBOUNCE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_chan_select<const O: u8>(&mut self) -> ADC_CHAN_SELECT_W<O> {
        ADC_CHAN_SELECT_W::new(self)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_chan0_select(&mut self) -> ADC_CHAN_SELECT_W<0> {
        ADC_CHAN_SELECT_W::new(self)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_chan1_select(&mut self) -> ADC_CHAN_SELECT_W<1> {
        ADC_CHAN_SELECT_W::new(self)
    }
    #[doc = "Bit 2 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_chan2_select(&mut self) -> ADC_CHAN_SELECT_W<2> {
        ADC_CHAN_SELECT_W::new(self)
    }
    #[doc = "Bit 3 - Analog Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_chan3_select(&mut self) -> ADC_CHAN_SELECT_W<3> {
        ADC_CHAN_SELECT_W::new(self)
    }
    #[doc = "Bit 4 - Touch Panel Mode and Auxiliary ADC Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tp_mode_select(&mut self) -> TP_MODE_SELECT_W<4> {
        TP_MODE_SELECT_W::new(self)
    }
    #[doc = "Bit 5 - TP Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_en(&mut self) -> TP_EN_W<5> {
        TP_EN_W::new(self)
    }
    #[doc = "Bit 6 - Touch Panel Double Point Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_dual_en(&mut self) -> TP_DUAL_EN_W<6> {
        TP_DUAL_EN_W::new(self)
    }
    #[doc = "Bit 7 - Touch Panel Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pan_cali_en(&mut self) -> TOUCH_PAN_CALI_EN_W<7> {
        TOUCH_PAN_CALI_EN_W::new(self)
    }
    #[doc = "Bit 8 - T-sensor Chopping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chopper_en(&mut self) -> CHOPPER_EN_W<8> {
        CHOPPER_EN_W::new(self)
    }
    #[doc = "Bit 9 - Stylus Up Debounce Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn stylus_up_debounce_en(&mut self) -> STYLUS_UP_DEBOUNCE_EN_W<9> {
        STYLUS_UP_DEBOUNCE_EN_W::new(self)
    }
    #[doc = "Bits 12:19 - Stylus Up Debounce Time Setting"]
    #[inline(always)]
    #[must_use]
    pub fn stylus_up_debounce(&mut self) -> STYLUS_UP_DEBOUNCE_W<12> {
        STYLUS_UP_DEBOUNCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_ctrl1](index.html) module"]
pub struct TP_CTRL1_SPEC;
impl crate::RegisterSpec for TP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_ctrl1::R](R) reader structure"]
impl crate::Readable for TP_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_ctrl1::W](W) writer structure"]
impl crate::Writable for TP_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tp_ctrl1 to value 0"]
impl crate::Resettable for TP_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
