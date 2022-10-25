#[doc = "Register `pll_lock_dbg_ctrl` reader"]
pub struct R(crate::R<PLL_LOCK_DBG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_LOCK_DBG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_LOCK_DBG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_LOCK_DBG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pll_lock_dbg_ctrl` writer"]
pub struct W(crate::W<PLL_LOCK_DBG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_LOCK_DBG_CTRL_SPEC>;
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
impl From<crate::W<PLL_LOCK_DBG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_LOCK_DBG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_CPUX = 0,
    #[doc = "1: `1`"]
    PLL_DDR = 1,
    #[doc = "2: `10`"]
    PLL_PERI_2X = 2,
    #[doc = "3: `11`"]
    PLL_VIDEO0_4X = 3,
    #[doc = "4: `100`"]
    PLL_VIDEO1_4X = 4,
    #[doc = "5: `101`"]
    PLL_VE = 5,
    #[doc = "6: `110`"]
    PLL_AUDIO0 = 6,
    #[doc = "7: `111`"]
    PLL_AUDIO1 = 7,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            0 => CLK_SRC_SEL_A::PLL_CPUX,
            1 => CLK_SRC_SEL_A::PLL_DDR,
            2 => CLK_SRC_SEL_A::PLL_PERI_2X,
            3 => CLK_SRC_SEL_A::PLL_VIDEO0_4X,
            4 => CLK_SRC_SEL_A::PLL_VIDEO1_4X,
            5 => CLK_SRC_SEL_A::PLL_VE,
            6 => CLK_SRC_SEL_A::PLL_AUDIO0,
            7 => CLK_SRC_SEL_A::PLL_AUDIO1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CPUX`"]
    #[inline(always)]
    pub fn is_pll_cpux(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_CPUX
    }
    #[doc = "Checks if the value of the field is `PLL_DDR`"]
    #[inline(always)]
    pub fn is_pll_ddr(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_DDR
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_4X`"]
    #[inline(always)]
    pub fn is_pll_video0_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_4X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_4X`"]
    #[inline(always)]
    pub fn is_pll_video1_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_4X
    }
    #[doc = "Checks if the value of the field is `PLL_VE`"]
    #[inline(always)]
    pub fn is_pll_ve(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VE
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO0`"]
    #[inline(always)]
    pub fn is_pll_audio0(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO0
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1`"]
    #[inline(always)]
    pub fn is_pll_audio1(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLL_LOCK_DBG_CTRL_SPEC, u8, CLK_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_cpux(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_CPUX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_ddr(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_DDR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_video0_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_4X)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_video1_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_4X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_ve(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VE)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_audio0(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn pll_audio1(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1)
    }
}
#[doc = "Field `pll_lock_flag_en` reader - Debug Enable"]
pub type PLL_LOCK_FLAG_EN_R = crate::BitReader<PLL_LOCK_FLAG_EN_A>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_LOCK_FLAG_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLL_LOCK_FLAG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_FLAG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_LOCK_FLAG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_FLAG_EN_A {
        match self.bits {
            false => PLL_LOCK_FLAG_EN_A::DISABLE,
            true => PLL_LOCK_FLAG_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_LOCK_FLAG_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_LOCK_FLAG_EN_A::ENABLE
    }
}
#[doc = "Field `pll_lock_flag_en` writer - Debug Enable"]
pub type PLL_LOCK_FLAG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_LOCK_DBG_CTRL_SPEC, PLL_LOCK_FLAG_EN_A, O>;
impl<'a, const O: u8> PLL_LOCK_FLAG_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_LOCK_FLAG_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_LOCK_FLAG_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 20:22 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 31 - Debug Enable"]
    #[inline(always)]
    pub fn pll_lock_flag_en(&self) -> PLL_LOCK_FLAG_EN_R {
        PLL_LOCK_FLAG_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<20> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_flag_en(&mut self) -> PLL_LOCK_FLAG_EN_W<31> {
        PLL_LOCK_FLAG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Lock Debug Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_lock_dbg_ctrl](index.html) module"]
pub struct PLL_LOCK_DBG_CTRL_SPEC;
impl crate::RegisterSpec for PLL_LOCK_DBG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_lock_dbg_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_LOCK_DBG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_lock_dbg_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_LOCK_DBG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_lock_dbg_ctrl to value 0"]
impl crate::Resettable for PLL_LOCK_DBG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
