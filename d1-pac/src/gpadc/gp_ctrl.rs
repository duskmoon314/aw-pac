#[doc = "Register `GP_CTRL` reader"]
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
#[doc = "Register `GP_CTRL` writer"]
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
#[doc = "Field `ADC_FIRST_DLY` reader - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
pub struct ADC_FIRST_DLY_R(crate::FieldReader<u8>);
impl ADC_FIRST_DLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_FIRST_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_FIRST_DLY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_FIRST_DLY` writer - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
pub struct ADC_FIRST_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_FIRST_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ADC_AUTOCALI_EN` reader - ADC Auto Calibration"]
pub struct ADC_AUTOCALI_EN_R(crate::FieldReader<bool>);
impl ADC_AUTOCALI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_AUTOCALI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_AUTOCALI_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_AUTOCALI_EN` writer - ADC Auto Calibration"]
pub struct ADC_AUTOCALI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_AUTOCALI_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `ADC_OP_BIAS` reader - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
pub struct ADC_OP_BIAS_R(crate::FieldReader<u8>);
impl ADC_OP_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_OP_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OP_BIAS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_OP_BIAS` writer - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
pub struct ADC_OP_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OP_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "GPADC Work Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `GPADC_WORK_MODE` reader - GPADC Work Mode"]
pub struct GPADC_WORK_MODE_R(crate::FieldReader<u8>);
impl GPADC_WORK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_WORK_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == GPADC_WORK_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == GPADC_WORK_MODE_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        **self == GPADC_WORK_MODE_A::BURST
    }
}
impl core::ops::Deref for GPADC_WORK_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPADC_WORK_MODE` writer - GPADC Work Mode"]
pub struct GPADC_WORK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_WORK_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPADC_WORK_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "ADC Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC_CALI_EN` reader - ADC Calibration"]
pub struct ADC_CALI_EN_R(crate::FieldReader<bool>);
impl ADC_CALI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CALI_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC_CALI_EN_A::START
    }
}
impl core::ops::Deref for ADC_CALI_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_CALI_EN` writer - ADC Calibration"]
pub struct ADC_CALI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CALI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_CALI_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start Calibration, it is cleared to 0 after calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ADC_CALI_EN_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ADC_EN` reader - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
pub struct ADC_EN_R(crate::FieldReader<bool>);
impl ADC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ADC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADC_EN_A::ENABLE
    }
}
impl core::ops::Deref for ADC_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_EN` writer - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
pub struct ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
    #[inline(always)]
    pub fn adc_first_dly(&self) -> ADC_FIRST_DLY_R {
        ADC_FIRST_DLY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - ADC Auto Calibration"]
    #[inline(always)]
    pub fn adc_autocali_en(&self) -> ADC_AUTOCALI_EN_R {
        ADC_AUTOCALI_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 20:21 - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
    #[inline(always)]
    pub fn adc_op_bias(&self) -> ADC_OP_BIAS_R {
        ADC_OP_BIAS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPADC Work Mode"]
    #[inline(always)]
    pub fn gpadc_work_mode(&self) -> GPADC_WORK_MODE_R {
        GPADC_WORK_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 17 - ADC Calibration"]
    #[inline(always)]
    pub fn adc_cali_en(&self) -> ADC_CALI_EN_R {
        ADC_CALI_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - ADC First Convert Delay Setting\n\nADC conversion of each channel is delayed by N samples"]
    #[inline(always)]
    pub fn adc_first_dly(&mut self) -> ADC_FIRST_DLY_W {
        ADC_FIRST_DLY_W { w: self }
    }
    #[doc = "Bit 23 - ADC Auto Calibration"]
    #[inline(always)]
    pub fn adc_autocali_en(&mut self) -> ADC_AUTOCALI_EN_W {
        ADC_AUTOCALI_EN_W { w: self }
    }
    #[doc = "Bits 20:21 - ADC OP Bias\n\nAdjust the bandwidth of the ADC amplifier"]
    #[inline(always)]
    pub fn adc_op_bias(&mut self) -> ADC_OP_BIAS_W {
        ADC_OP_BIAS_W { w: self }
    }
    #[doc = "Bits 18:19 - GPADC Work Mode"]
    #[inline(always)]
    pub fn gpadc_work_mode(&mut self) -> GPADC_WORK_MODE_W {
        GPADC_WORK_MODE_W { w: self }
    }
    #[doc = "Bit 17 - ADC Calibration"]
    #[inline(always)]
    pub fn adc_cali_en(&mut self) -> ADC_CALI_EN_W {
        ADC_CALI_EN_W { w: self }
    }
    #[doc = "Bit 16 - ADC Function Enable\n\nBefore the bit is enabled, configure ADC parameters including the work mode and channel number, etc."]
    #[inline(always)]
    pub fn adc_en(&mut self) -> ADC_EN_W {
        ADC_EN_W { w: self }
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
}
#[doc = "`reset()` method sets GP_CTRL to value 0x0080_0000"]
impl crate::Resettable for GP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
