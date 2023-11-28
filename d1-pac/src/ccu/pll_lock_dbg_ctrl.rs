#[doc = "Register `pll_lock_dbg_ctrl` reader"]
pub type R = crate::R<PLL_LOCK_DBG_CTRL_SPEC>;
#[doc = "Register `pll_lock_dbg_ctrl` writer"]
pub type W = crate::W<PLL_LOCK_DBG_CTRL_SPEC>;
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<CLK_SRC_SEL_A>;
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
impl crate::FieldSpec for CLK_SRC_SEL_A {
    type Ux = u8;
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_SRC_SEL_A {
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pll_cpux(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_CPUX
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pll_ddr(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_DDR
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pll_video0_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO0_4X
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pll_video1_4x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VIDEO1_4X
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pll_ve(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_VE
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_pll_audio0(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO0
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_pll_audio1(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CLK_SRC_SEL_A>;
impl<'a, REG> CLK_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_cpux(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_CPUX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_ddr(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_DDR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_video0_4x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_4X)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_video1_4x(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_4X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_ve(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_VE)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_audio0(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn pll_audio1(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> PLL_LOCK_FLAG_EN_A {
        match self.bits {
            false => PLL_LOCK_FLAG_EN_A::DISABLE,
            true => PLL_LOCK_FLAG_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_LOCK_FLAG_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_LOCK_FLAG_EN_A::ENABLE
    }
}
#[doc = "Field `pll_lock_flag_en` writer - Debug Enable"]
pub type PLL_LOCK_FLAG_EN_W<'a, REG> = crate::BitWriter<'a, REG, PLL_LOCK_FLAG_EN_A>;
impl<'a, REG> PLL_LOCK_FLAG_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PLL_LOCK_FLAG_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<PLL_LOCK_DBG_CTRL_SPEC> {
        CLK_SRC_SEL_W::new(self, 20)
    }
    #[doc = "Bit 31 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_flag_en(&mut self) -> PLL_LOCK_FLAG_EN_W<PLL_LOCK_DBG_CTRL_SPEC> {
        PLL_LOCK_FLAG_EN_W::new(self, 31)
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
#[doc = "PLL Lock Debug Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_lock_dbg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_lock_dbg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_LOCK_DBG_CTRL_SPEC;
impl crate::RegisterSpec for PLL_LOCK_DBG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_lock_dbg_ctrl::R`](R) reader structure"]
impl crate::Readable for PLL_LOCK_DBG_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_lock_dbg_ctrl::W`](W) writer structure"]
impl crate::Writable for PLL_LOCK_DBG_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_lock_dbg_ctrl to value 0"]
impl crate::Resettable for PLL_LOCK_DBG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
