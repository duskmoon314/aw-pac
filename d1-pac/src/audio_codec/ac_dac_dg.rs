#[doc = "Register `ac_dac_dg` reader"]
pub struct R(crate::R<AC_DAC_DG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_dg` writer"]
pub struct W(crate::W<AC_DAC_DG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DG_SPEC>;
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
impl From<crate::W<AC_DAC_DG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adda_loop_mode` reader - ADDA Loop Mode Select"]
pub type ADDA_LOOP_MODE_R = crate::FieldReader<u8, ADDA_LOOP_MODE_A>;
#[doc = "ADDA Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADDA_LOOP_MODE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: ADDA LOOP MODE DACL/DACR is connected to ADC1/ADC2"]
    ADC12 = 1,
    #[doc = "2: ADDA LOOP MODE DACL/DACR is connected to ADC3"]
    ADC3 = 2,
}
impl From<ADDA_LOOP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDA_LOOP_MODE_A) -> Self {
        variant as _
    }
}
impl ADDA_LOOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDA_LOOP_MODE_A> {
        match self.bits {
            0 => Some(ADDA_LOOP_MODE_A::DISABLE),
            1 => Some(ADDA_LOOP_MODE_A::ADC12),
            2 => Some(ADDA_LOOP_MODE_A::ADC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADDA_LOOP_MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ADC12`"]
    #[inline(always)]
    pub fn is_adc12(&self) -> bool {
        *self == ADDA_LOOP_MODE_A::ADC12
    }
    #[doc = "Checks if the value of the field is `ADC3`"]
    #[inline(always)]
    pub fn is_adc3(&self) -> bool {
        *self == ADDA_LOOP_MODE_A::ADC3
    }
}
#[doc = "Field `adda_loop_mode` writer - ADDA Loop Mode Select"]
pub type ADDA_LOOP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_DG_SPEC, u8, ADDA_LOOP_MODE_A, 3, O>;
impl<'a, const O: u8> ADDA_LOOP_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDA_LOOP_MODE_A::DISABLE)
    }
    #[doc = "ADDA LOOP MODE DACL/DACR is connected to ADC1/ADC2"]
    #[inline(always)]
    pub fn adc12(self) -> &'a mut W {
        self.variant(ADDA_LOOP_MODE_A::ADC12)
    }
    #[doc = "ADDA LOOP MODE DACL/DACR is connected to ADC3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut W {
        self.variant(ADDA_LOOP_MODE_A::ADC3)
    }
}
#[doc = "Field `da_swp` reader - DAC Output Channel Swap Enable"]
pub type DA_SWP_R = crate::BitReader<DA_SWP_A>;
#[doc = "DAC Output Channel Swap Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DA_SWP_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DA_SWP_A> for bool {
    #[inline(always)]
    fn from(variant: DA_SWP_A) -> Self {
        variant as u8 != 0
    }
}
impl DA_SWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DA_SWP_A {
        match self.bits {
            false => DA_SWP_A::DISABLE,
            true => DA_SWP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DA_SWP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DA_SWP_A::ENABLE
    }
}
#[doc = "Field `da_swp` writer - DAC Output Channel Swap Enable"]
pub type DA_SWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_DAC_DG_SPEC, DA_SWP_A, O>;
impl<'a, const O: u8> DA_SWP_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DA_SWP_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DA_SWP_A::ENABLE)
    }
}
#[doc = "Field `codec_clk_select` reader - CODEC Clock Source Select"]
pub type CODEC_CLK_SELECT_R = crate::BitReader<CODEC_CLK_SELECT_A>;
#[doc = "CODEC Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CODEC_CLK_SELECT_A {
    #[doc = "0: CODEC clock from PLL"]
    PLL = 0,
    #[doc = "1: CODEC clock from OSC (for Debug)"]
    OSC = 1,
}
impl From<CODEC_CLK_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CODEC_CLK_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CODEC_CLK_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CODEC_CLK_SELECT_A {
        match self.bits {
            false => CODEC_CLK_SELECT_A::PLL,
            true => CODEC_CLK_SELECT_A::OSC,
        }
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CODEC_CLK_SELECT_A::PLL
    }
    #[doc = "Checks if the value of the field is `OSC`"]
    #[inline(always)]
    pub fn is_osc(&self) -> bool {
        *self == CODEC_CLK_SELECT_A::OSC
    }
}
#[doc = "Field `codec_clk_select` writer - CODEC Clock Source Select"]
pub type CODEC_CLK_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_DAC_DG_SPEC, CODEC_CLK_SELECT_A, O>;
impl<'a, const O: u8> CODEC_CLK_SELECT_W<'a, O> {
    #[doc = "CODEC clock from PLL"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CODEC_CLK_SELECT_A::PLL)
    }
    #[doc = "CODEC clock from OSC (for Debug)"]
    #[inline(always)]
    pub fn osc(self) -> &'a mut W {
        self.variant(CODEC_CLK_SELECT_A::OSC)
    }
}
#[doc = "Field `dac_pattern_select` reader - DAC Pattern Select"]
pub type DAC_PATTERN_SELECT_R = crate::FieldReader<u8, DAC_PATTERN_SELECT_A>;
#[doc = "DAC Pattern Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_PATTERN_SELECT_A {
    #[doc = "0: Normal (Audio sample from TX FIFO)"]
    NORMAL = 0,
    #[doc = "1: -6 dB Sin wave"]
    SIN6 = 1,
    #[doc = "2: -60 dB Sin wave"]
    SIN60 = 2,
    #[doc = "3: Silent wave"]
    SILENT = 3,
}
impl From<DAC_PATTERN_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_PATTERN_SELECT_A) -> Self {
        variant as _
    }
}
impl DAC_PATTERN_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC_PATTERN_SELECT_A {
        match self.bits {
            0 => DAC_PATTERN_SELECT_A::NORMAL,
            1 => DAC_PATTERN_SELECT_A::SIN6,
            2 => DAC_PATTERN_SELECT_A::SIN60,
            3 => DAC_PATTERN_SELECT_A::SILENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAC_PATTERN_SELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SIN6`"]
    #[inline(always)]
    pub fn is_sin6(&self) -> bool {
        *self == DAC_PATTERN_SELECT_A::SIN6
    }
    #[doc = "Checks if the value of the field is `SIN60`"]
    #[inline(always)]
    pub fn is_sin60(&self) -> bool {
        *self == DAC_PATTERN_SELECT_A::SIN60
    }
    #[doc = "Checks if the value of the field is `SILENT`"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == DAC_PATTERN_SELECT_A::SILENT
    }
}
#[doc = "Field `dac_pattern_select` writer - DAC Pattern Select"]
pub type DAC_PATTERN_SELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AC_DAC_DG_SPEC, u8, DAC_PATTERN_SELECT_A, 2, O>;
impl<'a, const O: u8> DAC_PATTERN_SELECT_W<'a, O> {
    #[doc = "Normal (Audio sample from TX FIFO)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAC_PATTERN_SELECT_A::NORMAL)
    }
    #[doc = "-6 dB Sin wave"]
    #[inline(always)]
    pub fn sin6(self) -> &'a mut W {
        self.variant(DAC_PATTERN_SELECT_A::SIN6)
    }
    #[doc = "-60 dB Sin wave"]
    #[inline(always)]
    pub fn sin60(self) -> &'a mut W {
        self.variant(DAC_PATTERN_SELECT_A::SIN60)
    }
    #[doc = "Silent wave"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut W {
        self.variant(DAC_PATTERN_SELECT_A::SILENT)
    }
}
#[doc = "Field `dac_modu_select` reader - DAC Modulator Debug Mode"]
pub type DAC_MODU_SELECT_R = crate::BitReader<DAC_MODU_SELECT_A>;
#[doc = "DAC Modulator Debug Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_MODU_SELECT_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    DEBUG = 1,
}
impl From<DAC_MODU_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_MODU_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_MODU_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC_MODU_SELECT_A {
        match self.bits {
            false => DAC_MODU_SELECT_A::NORMAL,
            true => DAC_MODU_SELECT_A::DEBUG,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAC_MODU_SELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DEBUG`"]
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == DAC_MODU_SELECT_A::DEBUG
    }
}
#[doc = "Field `dac_modu_select` writer - DAC Modulator Debug Mode"]
pub type DAC_MODU_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_DAC_DG_SPEC, DAC_MODU_SELECT_A, O>;
impl<'a, const O: u8> DAC_MODU_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAC_MODU_SELECT_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn debug(self) -> &'a mut W {
        self.variant(DAC_MODU_SELECT_A::DEBUG)
    }
}
impl R {
    #[doc = "Bits 0:2 - ADDA Loop Mode Select"]
    #[inline(always)]
    pub fn adda_loop_mode(&self) -> ADDA_LOOP_MODE_R {
        ADDA_LOOP_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - DAC Output Channel Swap Enable"]
    #[inline(always)]
    pub fn da_swp(&self) -> DA_SWP_R {
        DA_SWP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CODEC Clock Source Select"]
    #[inline(always)]
    pub fn codec_clk_select(&self) -> CODEC_CLK_SELECT_R {
        CODEC_CLK_SELECT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - DAC Pattern Select"]
    #[inline(always)]
    pub fn dac_pattern_select(&self) -> DAC_PATTERN_SELECT_R {
        DAC_PATTERN_SELECT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - DAC Modulator Debug Mode"]
    #[inline(always)]
    pub fn dac_modu_select(&self) -> DAC_MODU_SELECT_R {
        DAC_MODU_SELECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADDA Loop Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn adda_loop_mode(&mut self) -> ADDA_LOOP_MODE_W<0> {
        ADDA_LOOP_MODE_W::new(self)
    }
    #[doc = "Bit 6 - DAC Output Channel Swap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn da_swp(&mut self) -> DA_SWP_W<6> {
        DA_SWP_W::new(self)
    }
    #[doc = "Bit 8 - CODEC Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn codec_clk_select(&mut self) -> CODEC_CLK_SELECT_W<8> {
        CODEC_CLK_SELECT_W::new(self)
    }
    #[doc = "Bits 9:10 - DAC Pattern Select"]
    #[inline(always)]
    #[must_use]
    pub fn dac_pattern_select(&mut self) -> DAC_PATTERN_SELECT_W<9> {
        DAC_PATTERN_SELECT_W::new(self)
    }
    #[doc = "Bit 11 - DAC Modulator Debug Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac_modu_select(&mut self) -> DAC_MODU_SELECT_W<11> {
        DAC_MODU_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_dg](index.html) module"]
pub struct AC_DAC_DG_SPEC;
impl crate::RegisterSpec for AC_DAC_DG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_dg::R](R) reader structure"]
impl crate::Readable for AC_DAC_DG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_dg::W](W) writer structure"]
impl crate::Writable for AC_DAC_DG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_dg to value 0"]
impl crate::Resettable for AC_DAC_DG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
