#[doc = "Register `mbus_mat_clk_gating` reader"]
pub type R = crate::R<MBUS_MAT_CLK_GATING_SPEC>;
#[doc = "Register `mbus_mat_clk_gating` writer"]
pub type W = crate::W<MBUS_MAT_CLK_GATING_SPEC>;
#[doc = "Field `dma_mclk_en` reader - Gating MBUS Clock"]
pub type DMA_MCLK_EN_R = crate::BitReader<DMA_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<DMA_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_MCLK_EN_A {
        match self.bits {
            false => DMA_MCLK_EN_A::MASK,
            true => DMA_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == DMA_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == DMA_MCLK_EN_A::PASS
    }
}
#[doc = "Field `dma_mclk_en` writer - Gating MBUS Clock"]
pub type DMA_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA_MCLK_EN_A>;
impl<'a, REG> DMA_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_MCLK_EN_A::PASS)
    }
}
#[doc = "Field `ve_mclk_en` reader - Gating MBUS Clock"]
pub type VE_MCLK_EN_R = crate::BitReader<VE_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VE_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<VE_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VE_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VE_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VE_MCLK_EN_A {
        match self.bits {
            false => VE_MCLK_EN_A::MASK,
            true => VE_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == VE_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == VE_MCLK_EN_A::PASS
    }
}
#[doc = "Field `ve_mclk_en` writer - Gating MBUS Clock"]
pub type VE_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, VE_MCLK_EN_A>;
impl<'a, REG> VE_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(VE_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(VE_MCLK_EN_A::PASS)
    }
}
#[doc = "Field `ce_mclk_en` reader - Gating MBUS Clock"]
pub type CE_MCLK_EN_R = crate::BitReader<CE_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CE_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<CE_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CE_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CE_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CE_MCLK_EN_A {
        match self.bits {
            false => CE_MCLK_EN_A::MASK,
            true => CE_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == CE_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CE_MCLK_EN_A::PASS
    }
}
#[doc = "Field `ce_mclk_en` writer - Gating MBUS Clock"]
pub type CE_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, CE_MCLK_EN_A>;
impl<'a, REG> CE_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(CE_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(CE_MCLK_EN_A::PASS)
    }
}
#[doc = "Field `tvin_mclk_en` reader - Gating MBUS Clock"]
pub type TVIN_MCLK_EN_R = crate::BitReader<TVIN_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TVIN_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<TVIN_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TVIN_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TVIN_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TVIN_MCLK_EN_A {
        match self.bits {
            false => TVIN_MCLK_EN_A::MASK,
            true => TVIN_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == TVIN_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == TVIN_MCLK_EN_A::PASS
    }
}
#[doc = "Field `tvin_mclk_en` writer - Gating MBUS Clock"]
pub type TVIN_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, TVIN_MCLK_EN_A>;
impl<'a, REG> TVIN_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(TVIN_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(TVIN_MCLK_EN_A::PASS)
    }
}
#[doc = "Field `csi_mclk_en` reader - Gating MBUS Clock"]
pub type CSI_MCLK_EN_R = crate::BitReader<CSI_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSI_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<CSI_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CSI_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CSI_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSI_MCLK_EN_A {
        match self.bits {
            false => CSI_MCLK_EN_A::MASK,
            true => CSI_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == CSI_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CSI_MCLK_EN_A::PASS
    }
}
#[doc = "Field `csi_mclk_en` writer - Gating MBUS Clock"]
pub type CSI_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, CSI_MCLK_EN_A>;
impl<'a, REG> CSI_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(CSI_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(CSI_MCLK_EN_A::PASS)
    }
}
#[doc = "Field `g2d_mclk_en` reader - Gating MBUS Clock"]
pub type G2D_MCLK_EN_R = crate::BitReader<G2D_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G2D_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<G2D_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: G2D_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl G2D_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> G2D_MCLK_EN_A {
        match self.bits {
            false => G2D_MCLK_EN_A::MASK,
            true => G2D_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == G2D_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == G2D_MCLK_EN_A::PASS
    }
}
#[doc = "Field `g2d_mclk_en` writer - Gating MBUS Clock"]
pub type G2D_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, G2D_MCLK_EN_A>;
impl<'a, REG> G2D_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(G2D_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(G2D_MCLK_EN_A::PASS)
    }
}
#[doc = "Field `riscv_mclk_en` reader - Gating MBUS Clock"]
pub type RISCV_MCLK_EN_R = crate::BitReader<RISCV_MCLK_EN_A>;
#[doc = "Gating MBUS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISCV_MCLK_EN_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<RISCV_MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RISCV_MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RISCV_MCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RISCV_MCLK_EN_A {
        match self.bits {
            false => RISCV_MCLK_EN_A::MASK,
            true => RISCV_MCLK_EN_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == RISCV_MCLK_EN_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == RISCV_MCLK_EN_A::PASS
    }
}
#[doc = "Field `riscv_mclk_en` writer - Gating MBUS Clock"]
pub type RISCV_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG, RISCV_MCLK_EN_A>;
impl<'a, REG> RISCV_MCLK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(RISCV_MCLK_EN_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(RISCV_MCLK_EN_A::PASS)
    }
}
impl R {
    #[doc = "Bit 0 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn dma_mclk_en(&self) -> DMA_MCLK_EN_R {
        DMA_MCLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn ve_mclk_en(&self) -> VE_MCLK_EN_R {
        VE_MCLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn ce_mclk_en(&self) -> CE_MCLK_EN_R {
        CE_MCLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn tvin_mclk_en(&self) -> TVIN_MCLK_EN_R {
        TVIN_MCLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn csi_mclk_en(&self) -> CSI_MCLK_EN_R {
        CSI_MCLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn g2d_mclk_en(&self) -> G2D_MCLK_EN_R {
        G2D_MCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Gating MBUS Clock"]
    #[inline(always)]
    pub fn riscv_mclk_en(&self) -> RISCV_MCLK_EN_R {
        RISCV_MCLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dma_mclk_en(&mut self) -> DMA_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        DMA_MCLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn ve_mclk_en(&mut self) -> VE_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        VE_MCLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn ce_mclk_en(&mut self) -> CE_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        CE_MCLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 7 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn tvin_mclk_en(&mut self) -> TVIN_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        TVIN_MCLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn csi_mclk_en(&mut self) -> CSI_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        CSI_MCLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn g2d_mclk_en(&mut self) -> G2D_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        G2D_MCLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Gating MBUS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn riscv_mclk_en(&mut self) -> RISCV_MCLK_EN_W<MBUS_MAT_CLK_GATING_SPEC> {
        RISCV_MCLK_EN_W::new(self, 11)
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
#[doc = "MBUS Master Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbus_mat_clk_gating::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbus_mat_clk_gating::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBUS_MAT_CLK_GATING_SPEC;
impl crate::RegisterSpec for MBUS_MAT_CLK_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbus_mat_clk_gating::R`](R) reader structure"]
impl crate::Readable for MBUS_MAT_CLK_GATING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mbus_mat_clk_gating::W`](W) writer structure"]
impl crate::Writable for MBUS_MAT_CLK_GATING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mbus_mat_clk_gating to value 0"]
impl crate::Resettable for MBUS_MAT_CLK_GATING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
