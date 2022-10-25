#[doc = "Register `adc%s` reader"]
pub struct R(crate::R<ADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adc%s` writer"]
pub struct W(crate::W<ADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SPEC>;
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
impl From<crate::W<ADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_iopmic` reader - ADC OP MIC Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
pub type ADC_IOPMIC_R = crate::FieldReader<u8, ADC_IOPMIC_A>;
#[doc = "ADC OP MIC Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_IOPMIC_A {
    #[doc = "0: 1.50*IOPADC"]
    _1_50 = 0,
    #[doc = "1: 1.75*IOPADC"]
    _1_75 = 1,
    #[doc = "2: 2.00*IOPADC"]
    _2_00 = 2,
    #[doc = "3: 2.25*IOPADC"]
    _2_25 = 3,
}
impl From<ADC_IOPMIC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_IOPMIC_A) -> Self {
        variant as _
    }
}
impl ADC_IOPMIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IOPMIC_A {
        match self.bits {
            0 => ADC_IOPMIC_A::_1_50,
            1 => ADC_IOPMIC_A::_1_75,
            2 => ADC_IOPMIC_A::_2_00,
            3 => ADC_IOPMIC_A::_2_25,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_50`"]
    #[inline(always)]
    pub fn is_1_50(&self) -> bool {
        *self == ADC_IOPMIC_A::_1_50
    }
    #[doc = "Checks if the value of the field is `_1_75`"]
    #[inline(always)]
    pub fn is_1_75(&self) -> bool {
        *self == ADC_IOPMIC_A::_1_75
    }
    #[doc = "Checks if the value of the field is `_2_00`"]
    #[inline(always)]
    pub fn is_2_00(&self) -> bool {
        *self == ADC_IOPMIC_A::_2_00
    }
    #[doc = "Checks if the value of the field is `_2_25`"]
    #[inline(always)]
    pub fn is_2_25(&self) -> bool {
        *self == ADC_IOPMIC_A::_2_25
    }
}
#[doc = "Field `adc_iopmic` writer - ADC OP MIC Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
pub type ADC_IOPMIC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, ADC_IOPMIC_A, 2, O>;
impl<'a, const O: u8> ADC_IOPMIC_W<'a, O> {
    #[doc = "1.50*IOPADC"]
    #[inline(always)]
    pub fn _1_50(self) -> &'a mut W {
        self.variant(ADC_IOPMIC_A::_1_50)
    }
    #[doc = "1.75*IOPADC"]
    #[inline(always)]
    pub fn _1_75(self) -> &'a mut W {
        self.variant(ADC_IOPMIC_A::_1_75)
    }
    #[doc = "2.00*IOPADC"]
    #[inline(always)]
    pub fn _2_00(self) -> &'a mut W {
        self.variant(ADC_IOPMIC_A::_2_00)
    }
    #[doc = "2.25*IOPADC"]
    #[inline(always)]
    pub fn _2_25(self) -> &'a mut W {
        self.variant(ADC_IOPMIC_A::_2_25)
    }
}
#[doc = "Field `adc_iopsdm[2,1]` reader - ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
pub type ADC_IOPSDM_R = crate::FieldReader<u8, ADC_IOPSDM_A>;
#[doc = "ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_IOPSDM_A {
    #[doc = "0: 1.50*IOPADC"]
    _1_50 = 0,
    #[doc = "1: 1.75*IOPADC"]
    _1_75 = 1,
    #[doc = "2: 2.00*IOPADC"]
    _2_00 = 2,
    #[doc = "3: 2.25*IOPADC"]
    _2_25 = 3,
}
impl From<ADC_IOPSDM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_IOPSDM_A) -> Self {
        variant as _
    }
}
impl ADC_IOPSDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IOPSDM_A {
        match self.bits {
            0 => ADC_IOPSDM_A::_1_50,
            1 => ADC_IOPSDM_A::_1_75,
            2 => ADC_IOPSDM_A::_2_00,
            3 => ADC_IOPSDM_A::_2_25,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_50`"]
    #[inline(always)]
    pub fn is_1_50(&self) -> bool {
        *self == ADC_IOPSDM_A::_1_50
    }
    #[doc = "Checks if the value of the field is `_1_75`"]
    #[inline(always)]
    pub fn is_1_75(&self) -> bool {
        *self == ADC_IOPSDM_A::_1_75
    }
    #[doc = "Checks if the value of the field is `_2_00`"]
    #[inline(always)]
    pub fn is_2_00(&self) -> bool {
        *self == ADC_IOPSDM_A::_2_00
    }
    #[doc = "Checks if the value of the field is `_2_25`"]
    #[inline(always)]
    pub fn is_2_25(&self) -> bool {
        *self == ADC_IOPSDM_A::_2_25
    }
}
#[doc = "Field `adc_iopsdm[2,1]` writer - ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
pub type ADC_IOPSDM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, ADC_IOPSDM_A, 2, O>;
impl<'a, const O: u8> ADC_IOPSDM_W<'a, O> {
    #[doc = "1.50*IOPADC"]
    #[inline(always)]
    pub fn _1_50(self) -> &'a mut W {
        self.variant(ADC_IOPSDM_A::_1_50)
    }
    #[doc = "1.75*IOPADC"]
    #[inline(always)]
    pub fn _1_75(self) -> &'a mut W {
        self.variant(ADC_IOPSDM_A::_1_75)
    }
    #[doc = "2.00*IOPADC"]
    #[inline(always)]
    pub fn _2_00(self) -> &'a mut W {
        self.variant(ADC_IOPSDM_A::_2_00)
    }
    #[doc = "2.25*IOPADC"]
    #[inline(always)]
    pub fn _2_25(self) -> &'a mut W {
        self.variant(ADC_IOPSDM_A::_2_25)
    }
}
#[doc = "Field `adc_iopaaf` reader - ADC OP AAF Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nFor example: ADC_REG<15:14> = 11, IOPADC = 4 uA \n\n00: 1.50*4 uA = 6 uA \n\n01: 1.75*4 uA = 7 uA \n\n10: 2.00*4 uA = 8 uA \n\n11: 2.25*4 uA = 9 uA"]
pub type ADC_IOPAAF_R = crate::FieldReader<u8, ADC_IOPAAF_A>;
#[doc = "ADC OP AAF Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nFor example: ADC_REG<15:14> = 11, IOPADC = 4 uA \n\n00: 1.50*4 uA = 6 uA \n\n01: 1.75*4 uA = 7 uA \n\n10: 2.00*4 uA = 8 uA \n\n11: 2.25*4 uA = 9 uA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_IOPAAF_A {
    #[doc = "0: 1.50*IOPADC"]
    _1_50 = 0,
    #[doc = "1: 1.75*IOPADC"]
    _1_75 = 1,
    #[doc = "2: 2.00*IOPADC"]
    _2_00 = 2,
    #[doc = "3: 2.25*IOPADC"]
    _2_25 = 3,
}
impl From<ADC_IOPAAF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_IOPAAF_A) -> Self {
        variant as _
    }
}
impl ADC_IOPAAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IOPAAF_A {
        match self.bits {
            0 => ADC_IOPAAF_A::_1_50,
            1 => ADC_IOPAAF_A::_1_75,
            2 => ADC_IOPAAF_A::_2_00,
            3 => ADC_IOPAAF_A::_2_25,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_50`"]
    #[inline(always)]
    pub fn is_1_50(&self) -> bool {
        *self == ADC_IOPAAF_A::_1_50
    }
    #[doc = "Checks if the value of the field is `_1_75`"]
    #[inline(always)]
    pub fn is_1_75(&self) -> bool {
        *self == ADC_IOPAAF_A::_1_75
    }
    #[doc = "Checks if the value of the field is `_2_00`"]
    #[inline(always)]
    pub fn is_2_00(&self) -> bool {
        *self == ADC_IOPAAF_A::_2_00
    }
    #[doc = "Checks if the value of the field is `_2_25`"]
    #[inline(always)]
    pub fn is_2_25(&self) -> bool {
        *self == ADC_IOPAAF_A::_2_25
    }
}
#[doc = "Field `adc_iopaaf` writer - ADC OP AAF Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nFor example: ADC_REG<15:14> = 11, IOPADC = 4 uA \n\n00: 1.50*4 uA = 6 uA \n\n01: 1.75*4 uA = 7 uA \n\n10: 2.00*4 uA = 8 uA \n\n11: 2.25*4 uA = 9 uA"]
pub type ADC_IOPAAF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, ADC_IOPAAF_A, 2, O>;
impl<'a, const O: u8> ADC_IOPAAF_W<'a, O> {
    #[doc = "1.50*IOPADC"]
    #[inline(always)]
    pub fn _1_50(self) -> &'a mut W {
        self.variant(ADC_IOPAAF_A::_1_50)
    }
    #[doc = "1.75*IOPADC"]
    #[inline(always)]
    pub fn _1_75(self) -> &'a mut W {
        self.variant(ADC_IOPAAF_A::_1_75)
    }
    #[doc = "2.00*IOPADC"]
    #[inline(always)]
    pub fn _2_00(self) -> &'a mut W {
        self.variant(ADC_IOPAAF_A::_2_00)
    }
    #[doc = "2.25*IOPADC"]
    #[inline(always)]
    pub fn _2_25(self) -> &'a mut W {
        self.variant(ADC_IOPAAF_A::_2_25)
    }
}
#[doc = "Field `adc_pga_gain_ctrl` reader - ADC PGA gain settings:\n\n\n\n0x0: 0 dB \\t0x10: 21 dB \n\n0x1: 6 dB \\t0x11: 22 dB \n\n0x2: 6 dB \\t0x12: 23 dB \n\n0x3: 6 dB \\t0x13: 24 dB \n\n0x4: 9 dB \\t0x14: 25 dB \n\n0x5: 10 dB\\t0x15: 26 dB \n\n0x6: 11 dB\\t0x16: 27 dB \n\n0x7: 12 dB\\t0x17: 28 dB \n\n0x8: 13 dB\\t0x18: 29 dB \n\n0x9: 14 dB\\t0x19: 30 dB \n\n0xA: 15 dB\\t0x1A: 31 dB \n\n0xB: 16 dB\\t0x1B: 32 dB \n\n0xC: 17 dB\\t0x1C: 33 dB \n\n0xD: 18 dB\\t0x1D: 34 dB \n\n0xE: 19 dB\\t0x1E: 35 dB \n\n0xF: 20 dB\\t0x1F: 36 dB"]
pub type ADC_PGA_GAIN_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_pga_gain_ctrl` writer - ADC PGA gain settings:\n\n\n\n0x0: 0 dB \\t0x10: 21 dB \n\n0x1: 6 dB \\t0x11: 22 dB \n\n0x2: 6 dB \\t0x12: 23 dB \n\n0x3: 6 dB \\t0x13: 24 dB \n\n0x4: 9 dB \\t0x14: 25 dB \n\n0x5: 10 dB\\t0x15: 26 dB \n\n0x6: 11 dB\\t0x16: 27 dB \n\n0x7: 12 dB\\t0x17: 28 dB \n\n0x8: 13 dB\\t0x18: 29 dB \n\n0x9: 14 dB\\t0x19: 30 dB \n\n0xA: 15 dB\\t0x1A: 31 dB \n\n0xB: 16 dB\\t0x1B: 32 dB \n\n0xC: 17 dB\\t0x1C: 33 dB \n\n0xD: 18 dB\\t0x1D: 34 dB \n\n0xE: 19 dB\\t0x1E: 35 dB \n\n0xF: 20 dB\\t0x1F: 36 dB"]
pub type ADC_PGA_GAIN_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_SPEC, u8, u8, 5, O>;
#[doc = "Field `iopadc` reader - ADC1-ADC3 Bias Current Select"]
pub type IOPADC_R = crate::FieldReader<u8, IOPADC_A>;
#[doc = "ADC1-ADC3 Bias Current Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOPADC_A {
    #[doc = "0: 1 uA"]
    _1U = 0,
    #[doc = "1: 2 uA"]
    _2U = 1,
    #[doc = "2: 3 uA"]
    _3U = 2,
    #[doc = "3: 4 uA"]
    _4U = 3,
}
impl From<IOPADC_A> for u8 {
    #[inline(always)]
    fn from(variant: IOPADC_A) -> Self {
        variant as _
    }
}
impl IOPADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPADC_A {
        match self.bits {
            0 => IOPADC_A::_1U,
            1 => IOPADC_A::_2U,
            2 => IOPADC_A::_3U,
            3 => IOPADC_A::_4U,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1U`"]
    #[inline(always)]
    pub fn is_1u(&self) -> bool {
        *self == IOPADC_A::_1U
    }
    #[doc = "Checks if the value of the field is `_2U`"]
    #[inline(always)]
    pub fn is_2u(&self) -> bool {
        *self == IOPADC_A::_2U
    }
    #[doc = "Checks if the value of the field is `_3U`"]
    #[inline(always)]
    pub fn is_3u(&self) -> bool {
        *self == IOPADC_A::_3U
    }
    #[doc = "Checks if the value of the field is `_4U`"]
    #[inline(always)]
    pub fn is_4u(&self) -> bool {
        *self == IOPADC_A::_4U
    }
}
#[doc = "Field `iopadc` writer - ADC1-ADC3 Bias Current Select"]
pub type IOPADC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, IOPADC_A, 2, O>;
impl<'a, const O: u8> IOPADC_W<'a, O> {
    #[doc = "1 uA"]
    #[inline(always)]
    pub fn _1u(self) -> &'a mut W {
        self.variant(IOPADC_A::_1U)
    }
    #[doc = "2 uA"]
    #[inline(always)]
    pub fn _2u(self) -> &'a mut W {
        self.variant(IOPADC_A::_2U)
    }
    #[doc = "3 uA"]
    #[inline(always)]
    pub fn _3u(self) -> &'a mut W {
        self.variant(IOPADC_A::_3U)
    }
    #[doc = "4 uA"]
    #[inline(always)]
    pub fn _4u(self) -> &'a mut W {
        self.variant(IOPADC_A::_4U)
    }
}
#[doc = "Field `adc_pga_in_vcm_ctrl` reader - ADC PGA Common-Mode Voltage Control"]
pub type ADC_PGA_IN_VCM_CTRL_R = crate::FieldReader<u8, ADC_PGA_IN_VCM_CTRL_A>;
#[doc = "ADC PGA Common-Mode Voltage Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_PGA_IN_VCM_CTRL_A {
    #[doc = "0: 900 mV"]
    _900M = 0,
    #[doc = "1: 800 mV"]
    _800M = 1,
    #[doc = "2: 750 mV"]
    _750M = 2,
    #[doc = "3: 700 mV"]
    _700M = 3,
}
impl From<ADC_PGA_IN_VCM_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PGA_IN_VCM_CTRL_A) -> Self {
        variant as _
    }
}
impl ADC_PGA_IN_VCM_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_PGA_IN_VCM_CTRL_A {
        match self.bits {
            0 => ADC_PGA_IN_VCM_CTRL_A::_900M,
            1 => ADC_PGA_IN_VCM_CTRL_A::_800M,
            2 => ADC_PGA_IN_VCM_CTRL_A::_750M,
            3 => ADC_PGA_IN_VCM_CTRL_A::_700M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_900M`"]
    #[inline(always)]
    pub fn is_900m(&self) -> bool {
        *self == ADC_PGA_IN_VCM_CTRL_A::_900M
    }
    #[doc = "Checks if the value of the field is `_800M`"]
    #[inline(always)]
    pub fn is_800m(&self) -> bool {
        *self == ADC_PGA_IN_VCM_CTRL_A::_800M
    }
    #[doc = "Checks if the value of the field is `_750M`"]
    #[inline(always)]
    pub fn is_750m(&self) -> bool {
        *self == ADC_PGA_IN_VCM_CTRL_A::_750M
    }
    #[doc = "Checks if the value of the field is `_700M`"]
    #[inline(always)]
    pub fn is_700m(&self) -> bool {
        *self == ADC_PGA_IN_VCM_CTRL_A::_700M
    }
}
#[doc = "Field `adc_pga_in_vcm_ctrl` writer - ADC PGA Common-Mode Voltage Control"]
pub type ADC_PGA_IN_VCM_CTRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, ADC_PGA_IN_VCM_CTRL_A, 2, O>;
impl<'a, const O: u8> ADC_PGA_IN_VCM_CTRL_W<'a, O> {
    #[doc = "900 mV"]
    #[inline(always)]
    pub fn _900m(self) -> &'a mut W {
        self.variant(ADC_PGA_IN_VCM_CTRL_A::_900M)
    }
    #[doc = "800 mV"]
    #[inline(always)]
    pub fn _800m(self) -> &'a mut W {
        self.variant(ADC_PGA_IN_VCM_CTRL_A::_800M)
    }
    #[doc = "750 mV"]
    #[inline(always)]
    pub fn _750m(self) -> &'a mut W {
        self.variant(ADC_PGA_IN_VCM_CTRL_A::_750M)
    }
    #[doc = "700 mV"]
    #[inline(always)]
    pub fn _700m(self) -> &'a mut W {
        self.variant(ADC_PGA_IN_VCM_CTRL_A::_700M)
    }
}
#[doc = "Field `adc_pga_ctrl_rcm` reader - ADC PGA Common Mode Input Impedance Control for MICIN"]
pub type ADC_PGA_CTRL_RCM_R = crate::FieldReader<u8, ADC_PGA_CTRL_RCM_A>;
#[doc = "ADC PGA Common Mode Input Impedance Control for MICIN\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_PGA_CTRL_RCM_A {
    #[doc = "0: 100 kΩ"]
    _100K = 0,
    #[doc = "1: 75 kΩ"]
    _75K = 1,
    #[doc = "2: 50 kΩ"]
    _50K = 2,
    #[doc = "3: 25 kΩ"]
    _25K = 3,
}
impl From<ADC_PGA_CTRL_RCM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PGA_CTRL_RCM_A) -> Self {
        variant as _
    }
}
impl ADC_PGA_CTRL_RCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_PGA_CTRL_RCM_A {
        match self.bits {
            0 => ADC_PGA_CTRL_RCM_A::_100K,
            1 => ADC_PGA_CTRL_RCM_A::_75K,
            2 => ADC_PGA_CTRL_RCM_A::_50K,
            3 => ADC_PGA_CTRL_RCM_A::_25K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_100K`"]
    #[inline(always)]
    pub fn is_100k(&self) -> bool {
        *self == ADC_PGA_CTRL_RCM_A::_100K
    }
    #[doc = "Checks if the value of the field is `_75K`"]
    #[inline(always)]
    pub fn is_75k(&self) -> bool {
        *self == ADC_PGA_CTRL_RCM_A::_75K
    }
    #[doc = "Checks if the value of the field is `_50K`"]
    #[inline(always)]
    pub fn is_50k(&self) -> bool {
        *self == ADC_PGA_CTRL_RCM_A::_50K
    }
    #[doc = "Checks if the value of the field is `_25K`"]
    #[inline(always)]
    pub fn is_25k(&self) -> bool {
        *self == ADC_PGA_CTRL_RCM_A::_25K
    }
}
#[doc = "Field `adc_pga_ctrl_rcm` writer - ADC PGA Common Mode Input Impedance Control for MICIN"]
pub type ADC_PGA_CTRL_RCM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, ADC_PGA_CTRL_RCM_A, 2, O>;
impl<'a, const O: u8> ADC_PGA_CTRL_RCM_W<'a, O> {
    #[doc = "100 kΩ"]
    #[inline(always)]
    pub fn _100k(self) -> &'a mut W {
        self.variant(ADC_PGA_CTRL_RCM_A::_100K)
    }
    #[doc = "75 kΩ"]
    #[inline(always)]
    pub fn _75k(self) -> &'a mut W {
        self.variant(ADC_PGA_CTRL_RCM_A::_75K)
    }
    #[doc = "50 kΩ"]
    #[inline(always)]
    pub fn _50k(self) -> &'a mut W {
        self.variant(ADC_PGA_CTRL_RCM_A::_50K)
    }
    #[doc = "25 kΩ"]
    #[inline(always)]
    pub fn _25k(self) -> &'a mut W {
        self.variant(ADC_PGA_CTRL_RCM_A::_25K)
    }
}
#[doc = "Field `iopbuffer` reader - PGA Vcm Buffer OP Bias Current Select"]
pub type IOPBUFFER_R = crate::FieldReader<u8, IOPBUFFER_A>;
#[doc = "PGA Vcm Buffer OP Bias Current Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOPBUFFER_A {
    #[doc = "0: 6 uA"]
    _6UA = 0,
    #[doc = "1: 7 uA"]
    _7UA = 1,
    #[doc = "2: 8 uA"]
    _8UA = 2,
    #[doc = "3: 9 uA"]
    _9UA = 3,
}
impl From<IOPBUFFER_A> for u8 {
    #[inline(always)]
    fn from(variant: IOPBUFFER_A) -> Self {
        variant as _
    }
}
impl IOPBUFFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPBUFFER_A {
        match self.bits {
            0 => IOPBUFFER_A::_6UA,
            1 => IOPBUFFER_A::_7UA,
            2 => IOPBUFFER_A::_8UA,
            3 => IOPBUFFER_A::_9UA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_6UA`"]
    #[inline(always)]
    pub fn is_6ua(&self) -> bool {
        *self == IOPBUFFER_A::_6UA
    }
    #[doc = "Checks if the value of the field is `_7UA`"]
    #[inline(always)]
    pub fn is_7ua(&self) -> bool {
        *self == IOPBUFFER_A::_7UA
    }
    #[doc = "Checks if the value of the field is `_8UA`"]
    #[inline(always)]
    pub fn is_8ua(&self) -> bool {
        *self == IOPBUFFER_A::_8UA
    }
    #[doc = "Checks if the value of the field is `_9UA`"]
    #[inline(always)]
    pub fn is_9ua(&self) -> bool {
        *self == IOPBUFFER_A::_9UA
    }
}
#[doc = "Field `iopbuffer` writer - PGA Vcm Buffer OP Bias Current Select"]
pub type IOPBUFFER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, IOPBUFFER_A, 2, O>;
impl<'a, const O: u8> IOPBUFFER_W<'a, O> {
    #[doc = "6 uA"]
    #[inline(always)]
    pub fn _6ua(self) -> &'a mut W {
        self.variant(IOPBUFFER_A::_6UA)
    }
    #[doc = "7 uA"]
    #[inline(always)]
    pub fn _7ua(self) -> &'a mut W {
        self.variant(IOPBUFFER_A::_7UA)
    }
    #[doc = "8 uA"]
    #[inline(always)]
    pub fn _8ua(self) -> &'a mut W {
        self.variant(IOPBUFFER_A::_8UA)
    }
    #[doc = "9 uA"]
    #[inline(always)]
    pub fn _9ua(self) -> &'a mut W {
        self.variant(IOPBUFFER_A::_9UA)
    }
}
#[doc = "Field `lineinlg` reader - LINEINL Gain Control"]
pub type LINEINLG_R = crate::BitReader<LINEINLG_A>;
#[doc = "LINEINL Gain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEINLG_A {
    #[doc = "0: 0 dB"]
    _0DB = 0,
    #[doc = "1: 6 dB"]
    _6DB = 1,
}
impl From<LINEINLG_A> for bool {
    #[inline(always)]
    fn from(variant: LINEINLG_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEINLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEINLG_A {
        match self.bits {
            false => LINEINLG_A::_0DB,
            true => LINEINLG_A::_6DB,
        }
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == LINEINLG_A::_0DB
    }
    #[doc = "Checks if the value of the field is `_6DB`"]
    #[inline(always)]
    pub fn is_6db(&self) -> bool {
        *self == LINEINLG_A::_6DB
    }
}
#[doc = "Field `lineinlg` writer - LINEINL Gain Control"]
pub type LINEINLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, LINEINLG_A, O>;
impl<'a, const O: u8> LINEINLG_W<'a, O> {
    #[doc = "0 dB"]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(LINEINLG_A::_0DB)
    }
    #[doc = "6 dB"]
    #[inline(always)]
    pub fn _6db(self) -> &'a mut W {
        self.variant(LINEINLG_A::_6DB)
    }
}
#[doc = "Field `lineinlen` reader - LINEINL Enable"]
pub type LINEINLEN_R = crate::BitReader<LINEINLEN_A>;
#[doc = "LINEINL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEINLEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LINEINLEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINEINLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEINLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEINLEN_A {
        match self.bits {
            false => LINEINLEN_A::DISABLE,
            true => LINEINLEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LINEINLEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LINEINLEN_A::ENABLE
    }
}
#[doc = "Field `lineinlen` writer - LINEINL Enable"]
pub type LINEINLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, LINEINLEN_A, O>;
impl<'a, const O: u8> LINEINLEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LINEINLEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LINEINLEN_A::ENABLE)
    }
}
#[doc = "Field `dsm_dither_lvl` reader - Dither Level Control (Dither level is positive ralated to the ctrl bits)"]
pub type DSM_DITHER_LVL_R = crate::FieldReader<u8, DSM_DITHER_LVL_A>;
#[doc = "Dither Level Control (Dither level is positive ralated to the ctrl bits)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSM_DITHER_LVL_A {
    #[doc = "0: No Level"]
    N_O = 0,
    #[doc = "1: Min Level"]
    M_IN = 1,
    #[doc = "2: Middle Level"]
    M_IDDLE = 2,
    #[doc = "3: Max Level"]
    M_AX = 3,
}
impl From<DSM_DITHER_LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: DSM_DITHER_LVL_A) -> Self {
        variant as _
    }
}
impl DSM_DITHER_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSM_DITHER_LVL_A {
        match self.bits {
            0 => DSM_DITHER_LVL_A::N_O,
            1 => DSM_DITHER_LVL_A::M_IN,
            2 => DSM_DITHER_LVL_A::M_IDDLE,
            3 => DSM_DITHER_LVL_A::M_AX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `N_O`"]
    #[inline(always)]
    pub fn is_n_o(&self) -> bool {
        *self == DSM_DITHER_LVL_A::N_O
    }
    #[doc = "Checks if the value of the field is `M_IN`"]
    #[inline(always)]
    pub fn is_m_in(&self) -> bool {
        *self == DSM_DITHER_LVL_A::M_IN
    }
    #[doc = "Checks if the value of the field is `M_IDDLE`"]
    #[inline(always)]
    pub fn is_m_iddle(&self) -> bool {
        *self == DSM_DITHER_LVL_A::M_IDDLE
    }
    #[doc = "Checks if the value of the field is `M_AX`"]
    #[inline(always)]
    pub fn is_m_ax(&self) -> bool {
        *self == DSM_DITHER_LVL_A::M_AX
    }
}
#[doc = "Field `dsm_dither_lvl` writer - Dither Level Control (Dither level is positive ralated to the ctrl bits)"]
pub type DSM_DITHER_LVL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC_SPEC, u8, DSM_DITHER_LVL_A, 2, O>;
impl<'a, const O: u8> DSM_DITHER_LVL_W<'a, O> {
    #[doc = "No Level"]
    #[inline(always)]
    pub fn n_o(self) -> &'a mut W {
        self.variant(DSM_DITHER_LVL_A::N_O)
    }
    #[doc = "Min Level"]
    #[inline(always)]
    pub fn m_in(self) -> &'a mut W {
        self.variant(DSM_DITHER_LVL_A::M_IN)
    }
    #[doc = "Middle Level"]
    #[inline(always)]
    pub fn m_iddle(self) -> &'a mut W {
        self.variant(DSM_DITHER_LVL_A::M_IDDLE)
    }
    #[doc = "Max Level"]
    #[inline(always)]
    pub fn m_ax(self) -> &'a mut W {
        self.variant(DSM_DITHER_LVL_A::M_AX)
    }
}
#[doc = "Field `fminlg` reader - FMINL Gain Control"]
pub type FMINLG_R = crate::BitReader<FMINLG_A>;
#[doc = "FMINL Gain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMINLG_A {
    #[doc = "0: 0 dB"]
    _0DB = 0,
    #[doc = "1: 6 dB"]
    _6DB = 1,
}
impl From<FMINLG_A> for bool {
    #[inline(always)]
    fn from(variant: FMINLG_A) -> Self {
        variant as u8 != 0
    }
}
impl FMINLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMINLG_A {
        match self.bits {
            false => FMINLG_A::_0DB,
            true => FMINLG_A::_6DB,
        }
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == FMINLG_A::_0DB
    }
    #[doc = "Checks if the value of the field is `_6DB`"]
    #[inline(always)]
    pub fn is_6db(&self) -> bool {
        *self == FMINLG_A::_6DB
    }
}
#[doc = "Field `fminlg` writer - FMINL Gain Control"]
pub type FMINLG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, FMINLG_A, O>;
impl<'a, const O: u8> FMINLG_W<'a, O> {
    #[doc = "0 dB"]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(FMINLG_A::_0DB)
    }
    #[doc = "6 dB"]
    #[inline(always)]
    pub fn _6db(self) -> &'a mut W {
        self.variant(FMINLG_A::_6DB)
    }
}
#[doc = "Field `fminlen` reader - FMINL Enable"]
pub type FMINLEN_R = crate::BitReader<FMINLEN_A>;
#[doc = "FMINL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMINLEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FMINLEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMINLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMINLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMINLEN_A {
        match self.bits {
            false => FMINLEN_A::DISABLE,
            true => FMINLEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FMINLEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FMINLEN_A::ENABLE
    }
}
#[doc = "Field `fminlen` writer - FMINL Enable"]
pub type FMINLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, FMINLEN_A, O>;
impl<'a, const O: u8> FMINLEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FMINLEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FMINLEN_A::ENABLE)
    }
}
#[doc = "Field `mic_sin_en` reader - MIC Single Input Enable"]
pub type MIC_SIN_EN_R = crate::BitReader<MIC_SIN_EN_A>;
#[doc = "MIC Single Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIC_SIN_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<MIC_SIN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MIC_SIN_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MIC_SIN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIC_SIN_EN_A {
        match self.bits {
            false => MIC_SIN_EN_A::DISABLE,
            true => MIC_SIN_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MIC_SIN_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MIC_SIN_EN_A::ENABLE
    }
}
#[doc = "Field `mic_sin_en` writer - MIC Single Input Enable"]
pub type MIC_SIN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, MIC_SIN_EN_A, O>;
impl<'a, const O: u8> MIC_SIN_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MIC_SIN_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MIC_SIN_EN_A::ENABLE)
    }
}
#[doc = "Field `adc_d_itcher_c_ontrol` reader - Dither Control"]
pub type ADC_D_ITCHER_C_ONTROL_R = crate::BitReader<ADC_D_ITCHER_C_ONTROL_A>;
#[doc = "Dither Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_D_ITCHER_C_ONTROL_A {
    #[doc = "0: New Dither Off"]
    N_EW_D_ITHER_O_FF = 0,
    #[doc = "1: New Dither On"]
    N_EW_D_ITHER_O_N = 1,
}
impl From<ADC_D_ITCHER_C_ONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_D_ITCHER_C_ONTROL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_D_ITCHER_C_ONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_D_ITCHER_C_ONTROL_A {
        match self.bits {
            false => ADC_D_ITCHER_C_ONTROL_A::N_EW_D_ITHER_O_FF,
            true => ADC_D_ITCHER_C_ONTROL_A::N_EW_D_ITHER_O_N,
        }
    }
    #[doc = "Checks if the value of the field is `N_EW_D_ITHER_O_FF`"]
    #[inline(always)]
    pub fn is_n_ew_d_ither_o_ff(&self) -> bool {
        *self == ADC_D_ITCHER_C_ONTROL_A::N_EW_D_ITHER_O_FF
    }
    #[doc = "Checks if the value of the field is `N_EW_D_ITHER_O_N`"]
    #[inline(always)]
    pub fn is_n_ew_d_ither_o_n(&self) -> bool {
        *self == ADC_D_ITCHER_C_ONTROL_A::N_EW_D_ITHER_O_N
    }
}
#[doc = "Field `adc_d_itcher_c_ontrol` writer - Dither Control"]
pub type ADC_D_ITCHER_C_ONTROL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC_SPEC, ADC_D_ITCHER_C_ONTROL_A, O>;
impl<'a, const O: u8> ADC_D_ITCHER_C_ONTROL_W<'a, O> {
    #[doc = "New Dither Off"]
    #[inline(always)]
    pub fn n_ew_d_ither_o_ff(self) -> &'a mut W {
        self.variant(ADC_D_ITCHER_C_ONTROL_A::N_EW_D_ITHER_O_FF)
    }
    #[doc = "New Dither On"]
    #[inline(always)]
    pub fn n_ew_d_ither_o_n(self) -> &'a mut W {
        self.variant(ADC_D_ITCHER_C_ONTROL_A::N_EW_D_ITHER_O_N)
    }
}
#[doc = "Field `mic_pga_en` reader - MIC PGA Enable"]
pub type MIC_PGA_EN_R = crate::BitReader<MIC_PGA_EN_A>;
#[doc = "MIC PGA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIC_PGA_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<MIC_PGA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MIC_PGA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MIC_PGA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIC_PGA_EN_A {
        match self.bits {
            false => MIC_PGA_EN_A::DISABLED,
            true => MIC_PGA_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MIC_PGA_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MIC_PGA_EN_A::ENABLED
    }
}
#[doc = "Field `mic_pga_en` writer - MIC PGA Enable"]
pub type MIC_PGA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, MIC_PGA_EN_A, O>;
impl<'a, const O: u8> MIC_PGA_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MIC_PGA_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MIC_PGA_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_en` reader - ADC Channel Enable"]
pub type ADC_EN_R = crate::BitReader<ADC_EN_A>;
#[doc = "ADC Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => ADC_EN_A::DISABLED,
            true => ADC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_EN_A::ENABLED
    }
}
#[doc = "Field `adc_en` writer - ADC Channel Enable"]
pub type ADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_SPEC, ADC_EN_A, O>;
impl<'a, const O: u8> ADC_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC OP MIC Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    pub fn adc_iopmic(&self) -> ADC_IOPMIC_R {
        ADC_IOPMIC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    pub fn adc_iopsdm2(&self) -> ADC_IOPSDM_R {
        ADC_IOPSDM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    pub fn adc_iopsdm1(&self) -> ADC_IOPSDM_R {
        ADC_IOPSDM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ADC OP AAF Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nFor example: ADC_REG<15:14> = 11, IOPADC = 4 uA \n\n00: 1.50*4 uA = 6 uA \n\n01: 1.75*4 uA = 7 uA \n\n10: 2.00*4 uA = 8 uA \n\n11: 2.25*4 uA = 9 uA"]
    #[inline(always)]
    pub fn adc_iopaaf(&self) -> ADC_IOPAAF_R {
        ADC_IOPAAF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - ADC PGA gain settings:\n\n\n\n0x0: 0 dB \\t0x10: 21 dB \n\n0x1: 6 dB \\t0x11: 22 dB \n\n0x2: 6 dB \\t0x12: 23 dB \n\n0x3: 6 dB \\t0x13: 24 dB \n\n0x4: 9 dB \\t0x14: 25 dB \n\n0x5: 10 dB\\t0x15: 26 dB \n\n0x6: 11 dB\\t0x16: 27 dB \n\n0x7: 12 dB\\t0x17: 28 dB \n\n0x8: 13 dB\\t0x18: 29 dB \n\n0x9: 14 dB\\t0x19: 30 dB \n\n0xA: 15 dB\\t0x1A: 31 dB \n\n0xB: 16 dB\\t0x1B: 32 dB \n\n0xC: 17 dB\\t0x1C: 33 dB \n\n0xD: 18 dB\\t0x1D: 34 dB \n\n0xE: 19 dB\\t0x1E: 35 dB \n\n0xF: 20 dB\\t0x1F: 36 dB"]
    #[inline(always)]
    pub fn adc_pga_gain_ctrl(&self) -> ADC_PGA_GAIN_CTRL_R {
        ADC_PGA_GAIN_CTRL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - ADC1-ADC3 Bias Current Select"]
    #[inline(always)]
    pub fn iopadc(&self) -> IOPADC_R {
        IOPADC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC PGA Common-Mode Voltage Control"]
    #[inline(always)]
    pub fn adc_pga_in_vcm_ctrl(&self) -> ADC_PGA_IN_VCM_CTRL_R {
        ADC_PGA_IN_VCM_CTRL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - ADC PGA Common Mode Input Impedance Control for MICIN"]
    #[inline(always)]
    pub fn adc_pga_ctrl_rcm(&self) -> ADC_PGA_CTRL_RCM_R {
        ADC_PGA_CTRL_RCM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PGA Vcm Buffer OP Bias Current Select"]
    #[inline(always)]
    pub fn iopbuffer(&self) -> IOPBUFFER_R {
        IOPBUFFER_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - LINEINL Gain Control"]
    #[inline(always)]
    pub fn lineinlg(&self) -> LINEINLG_R {
        LINEINLG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LINEINL Enable"]
    #[inline(always)]
    pub fn lineinlen(&self) -> LINEINLEN_R {
        LINEINLEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Dither Level Control (Dither level is positive ralated to the ctrl bits)"]
    #[inline(always)]
    pub fn dsm_dither_lvl(&self) -> DSM_DITHER_LVL_R {
        DSM_DITHER_LVL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - FMINL Gain Control"]
    #[inline(always)]
    pub fn fminlg(&self) -> FMINLG_R {
        FMINLG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FMINL Enable"]
    #[inline(always)]
    pub fn fminlen(&self) -> FMINLEN_R {
        FMINLEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MIC Single Input Enable"]
    #[inline(always)]
    pub fn mic_sin_en(&self) -> MIC_SIN_EN_R {
        MIC_SIN_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Dither Control"]
    #[inline(always)]
    pub fn adc_d_itcher_c_ontrol(&self) -> ADC_D_ITCHER_C_ONTROL_R {
        ADC_D_ITCHER_C_ONTROL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MIC PGA Enable"]
    #[inline(always)]
    pub fn mic_pga_en(&self) -> MIC_PGA_EN_R {
        MIC_PGA_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC Channel Enable"]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC OP MIC Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    #[must_use]
    pub fn adc_iopmic(&mut self) -> ADC_IOPMIC_W<0> {
        ADC_IOPMIC_W::new(self)
    }
    #[doc = "ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn adc_iopsdm<const O: u8>(&mut self) -> ADC_IOPSDM_W<O> {
        ADC_IOPSDM_W::new(self)
    }
    #[doc = "Bits 4:5 - ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    #[must_use]
    pub fn adc_iopsdm2(&mut self) -> ADC_IOPSDM_W<4> {
        ADC_IOPSDM_W::new(self)
    }
    #[doc = "Bits 6:7 - ADC OP SDM Bias Current Select \\[i\\]\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA."]
    #[inline(always)]
    #[must_use]
    pub fn adc_iopsdm1(&mut self) -> ADC_IOPSDM_W<6> {
        ADC_IOPSDM_W::new(self)
    }
    #[doc = "Bits 6:7 - ADC OP AAF Bias Current Select\n\nIOPADC is defined by ADC_REG<15:14> from 1 uA to 4 uA.\n\nFor example: ADC_REG<15:14> = 11, IOPADC = 4 uA \n\n00: 1.50*4 uA = 6 uA \n\n01: 1.75*4 uA = 7 uA \n\n10: 2.00*4 uA = 8 uA \n\n11: 2.25*4 uA = 9 uA"]
    #[inline(always)]
    #[must_use]
    pub fn adc_iopaaf(&mut self) -> ADC_IOPAAF_W<6> {
        ADC_IOPAAF_W::new(self)
    }
    #[doc = "Bits 8:12 - ADC PGA gain settings:\n\n\n\n0x0: 0 dB \\t0x10: 21 dB \n\n0x1: 6 dB \\t0x11: 22 dB \n\n0x2: 6 dB \\t0x12: 23 dB \n\n0x3: 6 dB \\t0x13: 24 dB \n\n0x4: 9 dB \\t0x14: 25 dB \n\n0x5: 10 dB\\t0x15: 26 dB \n\n0x6: 11 dB\\t0x16: 27 dB \n\n0x7: 12 dB\\t0x17: 28 dB \n\n0x8: 13 dB\\t0x18: 29 dB \n\n0x9: 14 dB\\t0x19: 30 dB \n\n0xA: 15 dB\\t0x1A: 31 dB \n\n0xB: 16 dB\\t0x1B: 32 dB \n\n0xC: 17 dB\\t0x1C: 33 dB \n\n0xD: 18 dB\\t0x1D: 34 dB \n\n0xE: 19 dB\\t0x1E: 35 dB \n\n0xF: 20 dB\\t0x1F: 36 dB"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pga_gain_ctrl(&mut self) -> ADC_PGA_GAIN_CTRL_W<8> {
        ADC_PGA_GAIN_CTRL_W::new(self)
    }
    #[doc = "Bits 14:15 - ADC1-ADC3 Bias Current Select"]
    #[inline(always)]
    #[must_use]
    pub fn iopadc(&mut self) -> IOPADC_W<14> {
        IOPADC_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC PGA Common-Mode Voltage Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pga_in_vcm_ctrl(&mut self) -> ADC_PGA_IN_VCM_CTRL_W<16> {
        ADC_PGA_IN_VCM_CTRL_W::new(self)
    }
    #[doc = "Bits 18:19 - ADC PGA Common Mode Input Impedance Control for MICIN"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pga_ctrl_rcm(&mut self) -> ADC_PGA_CTRL_RCM_W<18> {
        ADC_PGA_CTRL_RCM_W::new(self)
    }
    #[doc = "Bits 20:21 - PGA Vcm Buffer OP Bias Current Select"]
    #[inline(always)]
    #[must_use]
    pub fn iopbuffer(&mut self) -> IOPBUFFER_W<20> {
        IOPBUFFER_W::new(self)
    }
    #[doc = "Bit 22 - LINEINL Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn lineinlg(&mut self) -> LINEINLG_W<22> {
        LINEINLG_W::new(self)
    }
    #[doc = "Bit 23 - LINEINL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lineinlen(&mut self) -> LINEINLEN_W<23> {
        LINEINLEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Dither Level Control (Dither level is positive ralated to the ctrl bits)"]
    #[inline(always)]
    #[must_use]
    pub fn dsm_dither_lvl(&mut self) -> DSM_DITHER_LVL_W<24> {
        DSM_DITHER_LVL_W::new(self)
    }
    #[doc = "Bit 26 - FMINL Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn fminlg(&mut self) -> FMINLG_W<26> {
        FMINLG_W::new(self)
    }
    #[doc = "Bit 27 - FMINL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fminlen(&mut self) -> FMINLEN_W<27> {
        FMINLEN_W::new(self)
    }
    #[doc = "Bit 28 - MIC Single Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mic_sin_en(&mut self) -> MIC_SIN_EN_W<28> {
        MIC_SIN_EN_W::new(self)
    }
    #[doc = "Bit 29 - Dither Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc_d_itcher_c_ontrol(&mut self) -> ADC_D_ITCHER_C_ONTROL_W<29> {
        ADC_D_ITCHER_C_ONTROL_W::new(self)
    }
    #[doc = "Bit 30 - MIC PGA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mic_pga_en(&mut self) -> MIC_PGA_EN_W<30> {
        MIC_PGA_EN_W::new(self)
    }
    #[doc = "Bit 31 - ADC Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_en(&mut self) -> ADC_EN_W<31> {
        ADC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC\\[i\\] Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc](index.html) module"]
pub struct ADC_SPEC;
impl crate::RegisterSpec for ADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc::R](R) reader structure"]
impl crate::Readable for ADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc::W](W) writer structure"]
impl crate::Writable for ADC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adc%s to value 0x001c_c055"]
impl crate::Resettable for ADC_SPEC {
    const RESET_VALUE: Self::Ux = 0x001c_c055;
}
