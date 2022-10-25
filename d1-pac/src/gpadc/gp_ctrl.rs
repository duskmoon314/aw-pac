#[doc = "Register `gp_ctrl` reader"]
pub struct R(crate::R<GP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_ctrl` writer"]
pub struct W(crate::W<GP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_CTRL_SPEC>;
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
impl From<crate::W<GP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_en` reader - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
pub type ADC_EN_R = crate::BitReader<ADC_EN_A>;
#[doc = "ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ADC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_EN_A {
        match self.bits {
            false => ADC_EN_A::DISABLE,
            true => ADC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_EN_A::ENABLE
    }
}
#[doc = "Field `adc_en` writer - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
pub type ADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GP_CTRL_SPEC, ADC_EN_A, O>;
impl<'a, const O: u8> ADC_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_EN_A::ENABLE)
    }
}
#[doc = "Field `adc_cali_en` reader - ADC Calibration"]
pub type ADC_CALI_EN_R = crate::BitReader<ADC_CALI_EN_A>;
#[doc = "ADC Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_CALI_EN_A {
    #[doc = "1: Start Calibration, it is cleared to 0 after calibration"]
    START = 1,
}
impl From<ADC_CALI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CALI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_CALI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_CALI_EN_A> {
        match self.bits {
            true => Some(ADC_CALI_EN_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ADC_CALI_EN_A::START
    }
}
#[doc = "Field `adc_cali_en` writer - ADC Calibration"]
pub type ADC_CALI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GP_CTRL_SPEC, ADC_CALI_EN_A, O>;
impl<'a, const O: u8> ADC_CALI_EN_W<'a, O> {
    #[doc = "Start Calibration, it is cleared to 0 after calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ADC_CALI_EN_A::START)
    }
}
#[doc = "Field `gpadc_work_mode` reader - GPADC Work Mode"]
pub type GPADC_WORK_MODE_R = crate::FieldReader<u8, GPADC_WORK_MODE_A>;
#[doc = "GPADC Work Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPADC_WORK_MODE_A {
    #[doc = "0: Single conversion mode"]
    SINGLE = 0,
    #[doc = "2: Continuous conversion mode"]
    CONTINUOUS = 2,
    #[doc = "3: Burst conversion mode"]
    BURST = 3,
}
impl From<GPADC_WORK_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPADC_WORK_MODE_A) -> Self {
        variant as _
    }
}
impl GPADC_WORK_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPADC_WORK_MODE_A> {
        match self.bits {
            0 => Some(GPADC_WORK_MODE_A::SINGLE),
            2 => Some(GPADC_WORK_MODE_A::CONTINUOUS),
            3 => Some(GPADC_WORK_MODE_A::BURST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == GPADC_WORK_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == GPADC_WORK_MODE_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == GPADC_WORK_MODE_A::BURST
    }
}
#[doc = "Field `gpadc_work_mode` writer - GPADC Work Mode"]
pub type GPADC_WORK_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GP_CTRL_SPEC, u8, GPADC_WORK_MODE_A, 2, O>;
impl<'a, const O: u8> GPADC_WORK_MODE_W<'a, O> {
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(GPADC_WORK_MODE_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(GPADC_WORK_MODE_A::CONTINUOUS)
    }
    #[doc = "Burst conversion mode"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(GPADC_WORK_MODE_A::BURST)
    }
}
#[doc = "Field `adc_op_bias` reader - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
pub type ADC_OP_BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_op_bias` writer - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
pub type ADC_OP_BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `adc_autocali_en` reader - ADC Auto Calibration"]
pub type ADC_AUTOCALI_EN_R = crate::BitReader<bool>;
#[doc = "Field `adc_autocali_en` writer - ADC Auto Calibration"]
pub type ADC_AUTOCALI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GP_CTRL_SPEC, bool, O>;
#[doc = "Field `adc_first_dly` reader - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
pub type ADC_FIRST_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_first_dly` writer - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
pub type ADC_FIRST_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 16 - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Calibration"]
    #[inline(always)]
    pub fn adc_cali_en(&self) -> ADC_CALI_EN_R {
        ADC_CALI_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - GPADC Work Mode"]
    #[inline(always)]
    pub fn gpadc_work_mode(&self) -> GPADC_WORK_MODE_R {
        GPADC_WORK_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
    #[inline(always)]
    pub fn adc_op_bias(&self) -> ADC_OP_BIAS_R {
        ADC_OP_BIAS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - ADC Auto Calibration"]
    #[inline(always)]
    pub fn adc_autocali_en(&self) -> ADC_AUTOCALI_EN_R {
        ADC_AUTOCALI_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
    #[inline(always)]
    pub fn adc_first_dly(&self) -> ADC_FIRST_DLY_R {
        ADC_FIRST_DLY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
    #[inline(always)]
    #[must_use]
    pub fn adc_en(&mut self) -> ADC_EN_W<16> {
        ADC_EN_W::new(self)
    }
    #[doc = "Bit 17 - ADC Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adc_cali_en(&mut self) -> ADC_CALI_EN_W<17> {
        ADC_CALI_EN_W::new(self)
    }
    #[doc = "Bits 18:19 - GPADC Work Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_work_mode(&mut self) -> GPADC_WORK_MODE_W<18> {
        GPADC_WORK_MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
    #[inline(always)]
    #[must_use]
    pub fn adc_op_bias(&mut self) -> ADC_OP_BIAS_W<20> {
        ADC_OP_BIAS_W::new(self)
    }
    #[doc = "Bit 23 - ADC Auto Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adc_autocali_en(&mut self) -> ADC_AUTOCALI_EN_W<23> {
        ADC_AUTOCALI_EN_W::new(self)
    }
    #[doc = "Bits 24:31 - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
    #[inline(always)]
    #[must_use]
    pub fn adc_first_dly(&mut self) -> ADC_FIRST_DLY_W<24> {
        ADC_FIRST_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_ctrl](index.html) module"]
pub struct GP_CTRL_SPEC;
impl crate::RegisterSpec for GP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_ctrl::R](R) reader structure"]
impl crate::Readable for GP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_ctrl::W](W) writer structure"]
impl crate::Writable for GP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_ctrl to value 0x0080_0000"]
impl crate::Resettable for GP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
