#[doc = "Register `phy_control` reader"]
pub type R = crate::R<PHY_CONTROL_SPEC>;
#[doc = "Register `phy_control` writer"]
pub type W = crate::W<PHY_CONTROL_SPEC>;
#[doc = "Field `vc_clk` reader - vc_clk"]
pub type VC_CLK_R = crate::BitReader;
#[doc = "Field `vc_clk` writer - vc_clk"]
pub type VC_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `siddq` reader - "]
pub type SIDDQ_R = crate::BitReader<SIDDQ_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIDDQ_A {
    #[doc = "1: Write 1 to disable phy"]
    DISABLE = 1,
    #[doc = "0: Write 0 to enable phy"]
    ENABLE = 0,
}
impl From<SIDDQ_A> for bool {
    #[inline(always)]
    fn from(variant: SIDDQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SIDDQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIDDQ_A {
        match self.bits {
            true => SIDDQ_A::DISABLE,
            false => SIDDQ_A::ENABLE,
        }
    }
    #[doc = "Write 1 to disable phy"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SIDDQ_A::DISABLE
    }
    #[doc = "Write 0 to enable phy"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SIDDQ_A::ENABLE
    }
}
#[doc = "Field `siddq` writer - "]
pub type SIDDQ_W<'a, REG> = crate::BitWriter<'a, REG, SIDDQ_A>;
impl<'a, REG> SIDDQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to disable phy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SIDDQ_A::DISABLE)
    }
    #[doc = "Write 0 to enable phy"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SIDDQ_A::ENABLE)
    }
}
#[doc = "Field `vc_di` reader - vc_di"]
pub type VC_DI_R = crate::BitReader;
#[doc = "Field `vc_di` writer - vc_di"]
pub type VC_DI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vc_addr` reader - vc_addr"]
pub type VC_ADDR_R = crate::FieldReader;
#[doc = "Field `vc_addr` writer - vc_addr"]
pub type VC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `bist_en_a` reader - "]
pub type BIST_EN_A_R = crate::BitReader;
#[doc = "Field `bist_en_a` writer - "]
pub type BIST_EN_A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - vc_clk"]
    #[inline(always)]
    pub fn vc_clk(&self) -> VC_CLK_R {
        VC_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn siddq(&self) -> SIDDQ_R {
        SIDDQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - vc_di"]
    #[inline(always)]
    pub fn vc_di(&self) -> VC_DI_R {
        VC_DI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - vc_addr"]
    #[inline(always)]
    pub fn vc_addr(&self) -> VC_ADDR_R {
        VC_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bist_en_a(&self) -> BIST_EN_A_R {
        BIST_EN_A_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - vc_clk"]
    #[inline(always)]
    #[must_use]
    pub fn vc_clk(&mut self) -> VC_CLK_W<PHY_CONTROL_SPEC> {
        VC_CLK_W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn siddq(&mut self) -> SIDDQ_W<PHY_CONTROL_SPEC> {
        SIDDQ_W::new(self, 3)
    }
    #[doc = "Bit 7 - vc_di"]
    #[inline(always)]
    #[must_use]
    pub fn vc_di(&mut self) -> VC_DI_W<PHY_CONTROL_SPEC> {
        VC_DI_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - vc_addr"]
    #[inline(always)]
    #[must_use]
    pub fn vc_addr(&mut self) -> VC_ADDR_W<PHY_CONTROL_SPEC> {
        VC_ADDR_W::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn bist_en_a(&mut self) -> BIST_EN_A_W<PHY_CONTROL_SPEC> {
        BIST_EN_A_W::new(self, 16)
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
#[doc = "PHY Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_CONTROL_SPEC;
impl crate::RegisterSpec for PHY_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_control::R`](R) reader structure"]
impl crate::Readable for PHY_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_control::W`](W) writer structure"]
impl crate::Writable for PHY_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets phy_control to value 0"]
impl crate::Resettable for PHY_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
