#[doc = "Register `pll_ve_pat0_ctrl` reader"]
pub type R = crate::R<PLL_VE_PAT0_CTRL_SPEC>;
#[doc = "Register `pll_ve_pat0_ctrl` writer"]
pub type W = crate::W<PLL_VE_PAT0_CTRL_SPEC>;
#[doc = "Field `wave_bot` reader - Wave Bottom"]
pub type WAVE_BOT_R = crate::FieldReader<u32>;
#[doc = "Field `wave_bot` writer - Wave Bottom"]
pub type WAVE_BOT_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `freq` reader - Frequency"]
pub type FREQ_R = crate::FieldReader<FREQ_A>;
#[doc = "Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for FREQ_A {
    type Ux = u8;
}
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FREQ_A {
        match self.bits {
            0 => FREQ_A::F_31_5_K,
            1 => FREQ_A::F_32_K,
            2 => FREQ_A::F_32_5_K,
            3 => FREQ_A::F_33_K,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_f_31_5_k(&self) -> bool {
        *self == FREQ_A::F_31_5_K
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_f_32_k(&self) -> bool {
        *self == FREQ_A::F_32_K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_f_32_5_k(&self) -> bool {
        *self == FREQ_A::F_32_5_K
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_f_33_k(&self) -> bool {
        *self == FREQ_A::F_33_K
    }
}
#[doc = "Field `freq` writer - Frequency"]
pub type FREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FREQ_A>;
impl<'a, REG> FREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f_31_5_k(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_A::F_31_5_K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f_32_k(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_A::F_32_K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn f_32_5_k(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_A::F_32_5_K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn f_33_k(self) -> &'a mut crate::W<REG> {
        self.variant(FREQ_A::F_33_K)
    }
}
#[doc = "Field `sdm_clk_sel` reader - SDM Clock Select"]
pub type SDM_CLK_SEL_R = crate::BitReader<SDM_CLK_SEL_A>;
#[doc = "SDM Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SDM_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDM_CLK_SEL_A {
        match self.bits {
            false => SDM_CLK_SEL_A::F_24_M,
            true => SDM_CLK_SEL_A::F_12_M,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_f_24_m(&self) -> bool {
        *self == SDM_CLK_SEL_A::F_24_M
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_f_12_m(&self) -> bool {
        *self == SDM_CLK_SEL_A::F_12_M
    }
}
#[doc = "Field `sdm_clk_sel` writer - SDM Clock Select"]
pub type SDM_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG, SDM_CLK_SEL_A>;
impl<'a, REG> SDM_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f_24_m(self) -> &'a mut crate::W<REG> {
        self.variant(SDM_CLK_SEL_A::F_24_M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f_12_m(self) -> &'a mut crate::W<REG> {
        self.variant(SDM_CLK_SEL_A::F_12_M)
    }
}
#[doc = "Field `wave_step` reader - Wave Step"]
pub type WAVE_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `wave_step` writer - Wave Step"]
pub type WAVE_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `spr_freq_mode` reader - Spread Frequency Mode"]
pub type SPR_FREQ_MODE_R = crate::FieldReader<SPR_FREQ_MODE_A>;
#[doc = "Spread Frequency Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SPR_FREQ_MODE_A {
    type Ux = u8;
}
impl SPR_FREQ_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPR_FREQ_MODE_A {
        match self.bits {
            0 => SPR_FREQ_MODE_A::DC0,
            1 => SPR_FREQ_MODE_A::DC1,
            2 => SPR_FREQ_MODE_A::TRIANGULAR_1,
            3 => SPR_FREQ_MODE_A::TRIANGULAR_N,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_dc0(&self) -> bool {
        *self == SPR_FREQ_MODE_A::DC0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_dc1(&self) -> bool {
        *self == SPR_FREQ_MODE_A::DC1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_triangular_1(&self) -> bool {
        *self == SPR_FREQ_MODE_A::TRIANGULAR_1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_triangular_n(&self) -> bool {
        *self == SPR_FREQ_MODE_A::TRIANGULAR_N
    }
}
#[doc = "Field `spr_freq_mode` writer - Spread Frequency Mode"]
pub type SPR_FREQ_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPR_FREQ_MODE_A>;
impl<'a, REG> SPR_FREQ_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dc0(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_FREQ_MODE_A::DC0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dc1(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_FREQ_MODE_A::DC1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn triangular_1(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_FREQ_MODE_A::TRIANGULAR_1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn triangular_n(self) -> &'a mut crate::W<REG> {
        self.variant(SPR_FREQ_MODE_A::TRIANGULAR_N)
    }
}
#[doc = "Field `sig_delt_pat_en` reader - Sigma-Delta Pattern Enable"]
pub type SIG_DELT_PAT_EN_R = crate::BitReader;
#[doc = "Field `sig_delt_pat_en` writer - Sigma-Delta Pattern Enable"]
pub type SIG_DELT_PAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - Wave Bottom"]
    #[inline(always)]
    pub fn wave_bot(&self) -> WAVE_BOT_R {
        WAVE_BOT_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:18 - Frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - SDM Clock Select"]
    #[inline(always)]
    pub fn sdm_clk_sel(&self) -> SDM_CLK_SEL_R {
        SDM_CLK_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28 - Wave Step"]
    #[inline(always)]
    pub fn wave_step(&self) -> WAVE_STEP_R {
        WAVE_STEP_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bits 29:30 - Spread Frequency Mode"]
    #[inline(always)]
    pub fn spr_freq_mode(&self) -> SPR_FREQ_MODE_R {
        SPR_FREQ_MODE_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Sigma-Delta Pattern Enable"]
    #[inline(always)]
    pub fn sig_delt_pat_en(&self) -> SIG_DELT_PAT_EN_R {
        SIG_DELT_PAT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Wave Bottom"]
    #[inline(always)]
    #[must_use]
    pub fn wave_bot(&mut self) -> WAVE_BOT_W<PLL_VE_PAT0_CTRL_SPEC> {
        WAVE_BOT_W::new(self, 0)
    }
    #[doc = "Bits 17:18 - Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<PLL_VE_PAT0_CTRL_SPEC> {
        FREQ_W::new(self, 17)
    }
    #[doc = "Bit 19 - SDM Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdm_clk_sel(&mut self) -> SDM_CLK_SEL_W<PLL_VE_PAT0_CTRL_SPEC> {
        SDM_CLK_SEL_W::new(self, 19)
    }
    #[doc = "Bits 20:28 - Wave Step"]
    #[inline(always)]
    #[must_use]
    pub fn wave_step(&mut self) -> WAVE_STEP_W<PLL_VE_PAT0_CTRL_SPEC> {
        WAVE_STEP_W::new(self, 20)
    }
    #[doc = "Bits 29:30 - Spread Frequency Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spr_freq_mode(&mut self) -> SPR_FREQ_MODE_W<PLL_VE_PAT0_CTRL_SPEC> {
        SPR_FREQ_MODE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Sigma-Delta Pattern Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sig_delt_pat_en(&mut self) -> SIG_DELT_PAT_EN_W<PLL_VE_PAT0_CTRL_SPEC> {
        SIG_DELT_PAT_EN_W::new(self, 31)
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
#[doc = "PLL_VE Pattern0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ve_pat0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ve_pat0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_VE_PAT0_CTRL_SPEC;
impl crate::RegisterSpec for PLL_VE_PAT0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ve_pat0_ctrl::R`](R) reader structure"]
impl crate::Readable for PLL_VE_PAT0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_ve_pat0_ctrl::W`](W) writer structure"]
impl crate::Writable for PLL_VE_PAT0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_ve_pat0_ctrl to value 0"]
impl crate::Resettable for PLL_VE_PAT0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
