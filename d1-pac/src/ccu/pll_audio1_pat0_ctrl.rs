#[doc = "Register `PLL_AUDIO1_PAT0_CTRL` reader"]
pub struct R(crate::R<PLL_AUDIO1_PAT0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_AUDIO1_PAT0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_AUDIO1_PAT0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_AUDIO1_PAT0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_AUDIO1_PAT0_CTRL` writer"]
pub struct W(crate::W<PLL_AUDIO1_PAT0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_AUDIO1_PAT0_CTRL_SPEC>;
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
impl From<crate::W<PLL_AUDIO1_PAT0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_AUDIO1_PAT0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_DELT_PAT_EN` reader - Sigma-Delta Pattern Enable"]
pub struct SIG_DELT_PAT_EN_R(crate::FieldReader<bool, bool>);
impl SIG_DELT_PAT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIG_DELT_PAT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_DELT_PAT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_DELT_PAT_EN` writer - Sigma-Delta Pattern Enable"]
pub struct SIG_DELT_PAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_DELT_PAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Spread Frequency Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPR_FREQ_MODE_A {
    #[doc = "0: `0`"]
    DC0 = 0,
    #[doc = "1: `1`"]
    DC1 = 1,
    #[doc = "2: `10`"]
    TRIANGULAR_1 = 2,
    #[doc = "3: `11`"]
    TRIANGULAR_N = 3,
}
impl From<SPR_FREQ_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SPR_FREQ_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPR_FREQ_MODE` reader - Spread Frequency Mode"]
pub struct SPR_FREQ_MODE_R(crate::FieldReader<u8, SPR_FREQ_MODE_A>);
impl SPR_FREQ_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPR_FREQ_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPR_FREQ_MODE_A {
        match self.bits {
            0 => SPR_FREQ_MODE_A::DC0,
            1 => SPR_FREQ_MODE_A::DC1,
            2 => SPR_FREQ_MODE_A::TRIANGULAR_1,
            3 => SPR_FREQ_MODE_A::TRIANGULAR_N,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DC0`"]
    #[inline(always)]
    pub fn is_dc0(&self) -> bool {
        **self == SPR_FREQ_MODE_A::DC0
    }
    #[doc = "Checks if the value of the field is `DC1`"]
    #[inline(always)]
    pub fn is_dc1(&self) -> bool {
        **self == SPR_FREQ_MODE_A::DC1
    }
    #[doc = "Checks if the value of the field is `TRIANGULAR_1`"]
    #[inline(always)]
    pub fn is_triangular_1(&self) -> bool {
        **self == SPR_FREQ_MODE_A::TRIANGULAR_1
    }
    #[doc = "Checks if the value of the field is `TRIANGULAR_N`"]
    #[inline(always)]
    pub fn is_triangular_n(&self) -> bool {
        **self == SPR_FREQ_MODE_A::TRIANGULAR_N
    }
}
impl core::ops::Deref for SPR_FREQ_MODE_R {
    type Target = crate::FieldReader<u8, SPR_FREQ_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPR_FREQ_MODE` writer - Spread Frequency Mode"]
pub struct SPR_FREQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPR_FREQ_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPR_FREQ_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dc0(self) -> &'a mut W {
        self.variant(SPR_FREQ_MODE_A::DC0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dc1(self) -> &'a mut W {
        self.variant(SPR_FREQ_MODE_A::DC1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn triangular_1(self) -> &'a mut W {
        self.variant(SPR_FREQ_MODE_A::TRIANGULAR_1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn triangular_n(self) -> &'a mut W {
        self.variant(SPR_FREQ_MODE_A::TRIANGULAR_N)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `WAVE_STEP` reader - Wave Step"]
pub struct WAVE_STEP_R(crate::FieldReader<u16, u16>);
impl WAVE_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WAVE_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE_STEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE_STEP` writer - Wave Step"]
pub struct WAVE_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | ((value as u32 & 0x01ff) << 20);
        self.w
    }
}
#[doc = "SDM Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDM_CLK_SEL_A {
    #[doc = "0: `0`"]
    F_24_M = 0,
    #[doc = "1: `1`"]
    F_12_M = 1,
}
impl From<SDM_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDM_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDM_CLK_SEL` reader - SDM Clock Select"]
pub struct SDM_CLK_SEL_R(crate::FieldReader<bool, SDM_CLK_SEL_A>);
impl SDM_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDM_CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDM_CLK_SEL_A {
        match self.bits {
            false => SDM_CLK_SEL_A::F_24_M,
            true => SDM_CLK_SEL_A::F_12_M,
        }
    }
    #[doc = "Checks if the value of the field is `F_24_M`"]
    #[inline(always)]
    pub fn is_f_24_m(&self) -> bool {
        **self == SDM_CLK_SEL_A::F_24_M
    }
    #[doc = "Checks if the value of the field is `F_12_M`"]
    #[inline(always)]
    pub fn is_f_12_m(&self) -> bool {
        **self == SDM_CLK_SEL_A::F_12_M
    }
}
impl core::ops::Deref for SDM_CLK_SEL_R {
    type Target = crate::FieldReader<bool, SDM_CLK_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDM_CLK_SEL` writer - SDM Clock Select"]
pub struct SDM_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDM_CLK_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f_24_m(self) -> &'a mut W {
        self.variant(SDM_CLK_SEL_A::F_24_M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f_12_m(self) -> &'a mut W {
        self.variant(SDM_CLK_SEL_A::F_12_M)
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQ_A {
    #[doc = "0: `0`"]
    F_31_5_K = 0,
    #[doc = "1: `1`"]
    F_32_K = 1,
    #[doc = "2: `10`"]
    F_32_5_K = 2,
    #[doc = "3: `11`"]
    F_33_K = 3,
}
impl From<FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQ` reader - Frequency"]
pub struct FREQ_R(crate::FieldReader<u8, FREQ_A>);
impl FREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQ_A {
        match self.bits {
            0 => FREQ_A::F_31_5_K,
            1 => FREQ_A::F_32_K,
            2 => FREQ_A::F_32_5_K,
            3 => FREQ_A::F_33_K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `F_31_5_K`"]
    #[inline(always)]
    pub fn is_f_31_5_k(&self) -> bool {
        **self == FREQ_A::F_31_5_K
    }
    #[doc = "Checks if the value of the field is `F_32_K`"]
    #[inline(always)]
    pub fn is_f_32_k(&self) -> bool {
        **self == FREQ_A::F_32_K
    }
    #[doc = "Checks if the value of the field is `F_32_5_K`"]
    #[inline(always)]
    pub fn is_f_32_5_k(&self) -> bool {
        **self == FREQ_A::F_32_5_K
    }
    #[doc = "Checks if the value of the field is `F_33_K`"]
    #[inline(always)]
    pub fn is_f_33_k(&self) -> bool {
        **self == FREQ_A::F_33_K
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<u8, FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` writer - Frequency"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f_31_5_k(self) -> &'a mut W {
        self.variant(FREQ_A::F_31_5_K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f_32_k(self) -> &'a mut W {
        self.variant(FREQ_A::F_32_K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn f_32_5_k(self) -> &'a mut W {
        self.variant(FREQ_A::F_32_5_K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn f_33_k(self) -> &'a mut W {
        self.variant(FREQ_A::F_33_K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 17)) | ((value as u32 & 3) << 17);
        self.w
    }
}
#[doc = "Field `WAVE_BOT` reader - Wave Bottom"]
pub struct WAVE_BOT_R(crate::FieldReader<u32, u32>);
impl WAVE_BOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WAVE_BOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE_BOT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE_BOT` writer - Wave Bottom"]
pub struct WAVE_BOT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_BOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Sigma-Delta Pattern Enable"]
    #[inline(always)]
    pub fn sig_delt_pat_en(&self) -> SIG_DELT_PAT_EN_R {
        SIG_DELT_PAT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Spread Frequency Mode"]
    #[inline(always)]
    pub fn spr_freq_mode(&self) -> SPR_FREQ_MODE_R {
        SPR_FREQ_MODE_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bits 20:28 - Wave Step"]
    #[inline(always)]
    pub fn wave_step(&self) -> WAVE_STEP_R {
        WAVE_STEP_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 19 - SDM Clock Select"]
    #[inline(always)]
    pub fn sdm_clk_sel(&self) -> SDM_CLK_SEL_R {
        SDM_CLK_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 0:16 - Wave Bottom"]
    #[inline(always)]
    pub fn wave_bot(&self) -> WAVE_BOT_R {
        WAVE_BOT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - Sigma-Delta Pattern Enable"]
    #[inline(always)]
    pub fn sig_delt_pat_en(&mut self) -> SIG_DELT_PAT_EN_W {
        SIG_DELT_PAT_EN_W { w: self }
    }
    #[doc = "Bits 29:30 - Spread Frequency Mode"]
    #[inline(always)]
    pub fn spr_freq_mode(&mut self) -> SPR_FREQ_MODE_W {
        SPR_FREQ_MODE_W { w: self }
    }
    #[doc = "Bits 20:28 - Wave Step"]
    #[inline(always)]
    pub fn wave_step(&mut self) -> WAVE_STEP_W {
        WAVE_STEP_W { w: self }
    }
    #[doc = "Bit 19 - SDM Clock Select"]
    #[inline(always)]
    pub fn sdm_clk_sel(&mut self) -> SDM_CLK_SEL_W {
        SDM_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - Frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Bits 0:16 - Wave Bottom"]
    #[inline(always)]
    pub fn wave_bot(&mut self) -> WAVE_BOT_W {
        WAVE_BOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_AUDIO1 Pattern0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_audio1_pat0_ctrl](index.html) module"]
pub struct PLL_AUDIO1_PAT0_CTRL_SPEC;
impl crate::RegisterSpec for PLL_AUDIO1_PAT0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_audio1_pat0_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_AUDIO1_PAT0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_audio1_pat0_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_AUDIO1_PAT0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_AUDIO1_PAT0_CTRL to value 0"]
impl crate::Resettable for PLL_AUDIO1_PAT0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
