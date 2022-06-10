#[doc = "Register `EMAC_MII_CMD` reader"]
pub struct R(crate::R<EMAC_MII_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_MII_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_MII_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_MII_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_MII_CMD` writer"]
pub struct W(crate::W<EMAC_MII_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_MII_CMD_SPEC>;
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
impl From<crate::W<EMAC_MII_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_MII_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MDC Clock DIvider Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `MDC_DIV_RATIO_M` reader - MDC Clock DIvider Ratio"]
pub type MDC_DIV_RATIO_M_R = crate::FieldReader<u8, MDC_DIV_RATIO_M_A>;
impl MDC_DIV_RATIO_M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MDC_DIV_RATIO_M_A> {
        match self.bits {
            0 => Some(MDC_DIV_RATIO_M_A::R16),
            1 => Some(MDC_DIV_RATIO_M_A::R32),
            2 => Some(MDC_DIV_RATIO_M_A::R64),
            3 => Some(MDC_DIV_RATIO_M_A::R128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `R16`"]
    #[inline(always)]
    pub fn is_r16(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R16
    }
    #[doc = "Checks if the value of the field is `R32`"]
    #[inline(always)]
    pub fn is_r32(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R32
    }
    #[doc = "Checks if the value of the field is `R64`"]
    #[inline(always)]
    pub fn is_r64(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R64
    }
    #[doc = "Checks if the value of the field is `R128`"]
    #[inline(always)]
    pub fn is_r128(&self) -> bool {
        *self == MDC_DIV_RATIO_M_A::R128
    }
}
#[doc = "Field `MDC_DIV_RATIO_M` writer - MDC Clock DIvider Ratio"]
pub type MDC_DIV_RATIO_M_W<'a> =
    crate::FieldWriter<'a, u32, EMAC_MII_CMD_SPEC, u8, MDC_DIV_RATIO_M_A, 3, 20>;
impl<'a> MDC_DIV_RATIO_M_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r16(self) -> &'a mut W {
        self.variant(MDC_DIV_RATIO_M_A::R16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn r32(self) -> &'a mut W {
        self.variant(MDC_DIV_RATIO_M_A::R32)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn r64(self) -> &'a mut W {
        self.variant(MDC_DIV_RATIO_M_A::R64)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn r128(self) -> &'a mut W {
        self.variant(MDC_DIV_RATIO_M_A::R128)
    }
}
#[doc = "Field `PHY_ADDR` reader - PHY Address"]
pub type PHY_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHY_ADDR` writer - PHY Address"]
pub type PHY_ADDR_W<'a> = crate::FieldWriter<'a, u32, EMAC_MII_CMD_SPEC, u8, u8, 5, 12>;
#[doc = "Field `PHY_REG_ADDR` reader - PHY Register Address"]
pub type PHY_REG_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHY_REG_ADDR` writer - PHY Register Address"]
pub type PHY_REG_ADDR_W<'a> = crate::FieldWriter<'a, u32, EMAC_MII_CMD_SPEC, u8, u8, 5, 4>;
#[doc = "MII Write and Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `MII_WR` reader - MII Write and Read"]
pub type MII_WR_R = crate::BitReader<MII_WR_A>;
impl MII_WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MII_WR_A {
        match self.bits {
            false => MII_WR_A::R,
            true => MII_WR_A::W,
        }
    }
    #[doc = "Checks if the value of the field is `R`"]
    #[inline(always)]
    pub fn is_r(&self) -> bool {
        *self == MII_WR_A::R
    }
    #[doc = "Checks if the value of the field is `W`"]
    #[inline(always)]
    pub fn is_w(&self) -> bool {
        *self == MII_WR_A::W
    }
}
#[doc = "Field `MII_WR` writer - MII Write and Read"]
pub type MII_WR_W<'a> = crate::BitWriter<'a, u32, EMAC_MII_CMD_SPEC, MII_WR_A, 1>;
impl<'a> MII_WR_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn r(self) -> &'a mut W {
        self.variant(MII_WR_A::R)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn w(self) -> &'a mut W {
        self.variant(MII_WR_A::W)
    }
}
#[doc = "Field `MII_BUSY` reader - MII Status"]
pub type MII_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MII_BUSY` writer - MII Status"]
pub type MII_BUSY_W<'a> = crate::BitWriter<'a, u32, EMAC_MII_CMD_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 20:22 - MDC Clock DIvider Ratio"]
    #[inline(always)]
    pub fn mdc_div_ratio_m(&self) -> MDC_DIV_RATIO_M_R {
        MDC_DIV_RATIO_M_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 12:16 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 4:8 - PHY Register Address"]
    #[inline(always)]
    pub fn phy_reg_addr(&self) -> PHY_REG_ADDR_R {
        PHY_REG_ADDR_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - MII Write and Read"]
    #[inline(always)]
    pub fn mii_wr(&self) -> MII_WR_R {
        MII_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MII Status"]
    #[inline(always)]
    pub fn mii_busy(&self) -> MII_BUSY_R {
        MII_BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22 - MDC Clock DIvider Ratio"]
    #[inline(always)]
    pub fn mdc_div_ratio_m(&mut self) -> MDC_DIV_RATIO_M_W {
        MDC_DIV_RATIO_M_W::new(self)
    }
    #[doc = "Bits 12:16 - PHY Address"]
    #[inline(always)]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W {
        PHY_ADDR_W::new(self)
    }
    #[doc = "Bits 4:8 - PHY Register Address"]
    #[inline(always)]
    pub fn phy_reg_addr(&mut self) -> PHY_REG_ADDR_W {
        PHY_REG_ADDR_W::new(self)
    }
    #[doc = "Bit 1 - MII Write and Read"]
    #[inline(always)]
    pub fn mii_wr(&mut self) -> MII_WR_W {
        MII_WR_W::new(self)
    }
    #[doc = "Bit 0 - MII Status"]
    #[inline(always)]
    pub fn mii_busy(&mut self) -> MII_BUSY_W {
        MII_BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Management Interface Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_mii_cmd](index.html) module"]
pub struct EMAC_MII_CMD_SPEC;
impl crate::RegisterSpec for EMAC_MII_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_mii_cmd::R](R) reader structure"]
impl crate::Readable for EMAC_MII_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_mii_cmd::W](W) writer structure"]
impl crate::Writable for EMAC_MII_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_MII_CMD to value 0"]
impl crate::Resettable for EMAC_MII_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
