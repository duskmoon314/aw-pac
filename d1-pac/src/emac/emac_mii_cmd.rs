#[doc = "Register `emac_mii_cmd` reader"]
pub type R = crate::R<EMAC_MII_CMD_SPEC>;
#[doc = "Register `emac_mii_cmd` writer"]
pub type W = crate::W<EMAC_MII_CMD_SPEC>;
#[doc = "Field `mii_busy` reader - MII Status"]
pub type MII_BUSY_R = crate::BitReader;
#[doc = "Field `mii_busy` writer - MII Status"]
pub type MII_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mii_wr` reader - MII Write and Read"]
pub type MII_WR_R = crate::BitReader<MII_WR_A>;
#[doc = "MII Write and Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MII_WR_A {
    #[doc = "0: `0`"]
    R = 0,
    #[doc = "1: `1`"]
    W = 1,
}
impl From<MII_WR_A> for bool {
    #[inline(always)]
    fn from(variant: MII_WR_A) -> Self {
        variant as u8 != 0
    }
}
impl MII_WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MII_WR_A {
        match self.bits {
            false => MII_WR_A::R,
            true => MII_WR_A::W,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_r(&self) -> bool {
        *self == MII_WR_A::R
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_w(&self) -> bool {
        *self == MII_WR_A::W
    }
}
#[doc = "Field `mii_wr` writer - MII Write and Read"]
pub type MII_WR_W<'a, REG> = crate::BitWriter<'a, REG, MII_WR_A>;
impl<'a, REG> MII_WR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r(self) -> &'a mut crate::W<REG> {
        self.variant(MII_WR_A::R)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn w(self) -> &'a mut crate::W<REG> {
        self.variant(MII_WR_A::W)
    }
}
#[doc = "Field `phy_reg_addr` reader - PHY Register Address"]
pub type PHY_REG_ADDR_R = crate::FieldReader;
#[doc = "Field `phy_reg_addr` writer - PHY Register Address"]
pub type PHY_REG_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `phy_addr` reader - PHY Address"]
pub type PHY_ADDR_R = crate::FieldReader;
#[doc = "Field `phy_addr` writer - PHY Address"]
pub type PHY_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `mdc_div_ratio_m` reader - MDC Clock DIvider Ratio"]
pub type MDC_DIV_RATIO_M_R = crate::FieldReader<MDC_DIV_RATIO_M_A>;
#[doc = "MDC Clock DIvider Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDC_DIV_RATIO_M_A {
    #[doc = "0: `0`"]
    R16 = 0,
    #[doc = "1: `1`"]
    R32 = 1,
    #[doc = "2: `10`"]
    R64 = 2,
    #[doc = "3: `11`"]
    R128 = 3,
}
impl From<MDC_DIV_RATIO_M_A> for u8 {
    #[inline(always)]
    fn from(variant: MDC_DIV_RATIO_M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDC_DIV_RATIO_M_A {
    type Ux = u8;
}
impl MDC_DIV_RATIO_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MDC_DIV_RATIO_M_A> {
        match self.bits {
            0 => Some(MDC_DIV_RATIO_M_A::R16),
            1 => Some(MDC_DIV_RATIO_M_A::R32),
            2 => Some(MDC_DIV_RATIO_M_A::R64),
            3 => Some(MDC_DIV_RATIO_M_A::R128),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_r16(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_r32(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R32
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_r64(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R64
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_r128(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R128
    }
}
#[doc = "Field `mdc_div_ratio_m` writer - MDC Clock DIvider Ratio"]
pub type MDC_DIV_RATIO_M_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MDC_DIV_RATIO_M_A>;
impl<'a, REG> MDC_DIV_RATIO_M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r16(self) -> &'a mut crate::W<REG> {
        self.variant(MDC_DIV_RATIO_M_A::R16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r32(self) -> &'a mut crate::W<REG> {
        self.variant(MDC_DIV_RATIO_M_A::R32)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r64(self) -> &'a mut crate::W<REG> {
        self.variant(MDC_DIV_RATIO_M_A::R64)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r128(self) -> &'a mut crate::W<REG> {
        self.variant(MDC_DIV_RATIO_M_A::R128)
    }
}
impl R {
    #[doc = "Bit 0 - MII Status"]
    #[inline(always)]
    pub fn mii_busy(&self) -> MII_BUSY_R {
        MII_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII Write and Read"]
    #[inline(always)]
    pub fn mii_wr(&self) -> MII_WR_R {
        MII_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:8 - PHY Register Address"]
    #[inline(always)]
    pub fn phy_reg_addr(&self) -> PHY_REG_ADDR_R {
        PHY_REG_ADDR_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:22 - MDC Clock DIvider Ratio"]
    #[inline(always)]
    pub fn mdc_div_ratio_m(&self) -> MDC_DIV_RATIO_M_R {
        MDC_DIV_RATIO_M_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Status"]
    #[inline(always)]
    #[must_use]
    pub fn mii_busy(&mut self) -> MII_BUSY_W<EMAC_MII_CMD_SPEC> {
        MII_BUSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - MII Write and Read"]
    #[inline(always)]
    #[must_use]
    pub fn mii_wr(&mut self) -> MII_WR_W<EMAC_MII_CMD_SPEC> {
        MII_WR_W::new(self, 1)
    }
    #[doc = "Bits 4:8 - PHY Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn phy_reg_addr(&mut self) -> PHY_REG_ADDR_W<EMAC_MII_CMD_SPEC> {
        PHY_REG_ADDR_W::new(self, 4)
    }
    #[doc = "Bits 12:16 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W<EMAC_MII_CMD_SPEC> {
        PHY_ADDR_W::new(self, 12)
    }
    #[doc = "Bits 20:22 - MDC Clock DIvider Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn mdc_div_ratio_m(&mut self) -> MDC_DIV_RATIO_M_W<EMAC_MII_CMD_SPEC> {
        MDC_DIV_RATIO_M_W::new(self, 20)
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
#[doc = "EMAC Management Interface Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_mii_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_mii_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_MII_CMD_SPEC;
impl crate::RegisterSpec for EMAC_MII_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_mii_cmd::R`](R) reader structure"]
impl crate::Readable for EMAC_MII_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_mii_cmd::W`](W) writer structure"]
impl crate::Writable for EMAC_MII_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_mii_cmd to value 0"]
impl crate::Resettable for EMAC_MII_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
