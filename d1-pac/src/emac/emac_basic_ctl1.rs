#[doc = "Register `emac_basic_ctl1` reader"]
pub type R = crate::R<EMAC_BASIC_CTL1_SPEC>;
#[doc = "Register `emac_basic_ctl1` writer"]
pub type W = crate::W<EMAC_BASIC_CTL1_SPEC>;
#[doc = "Field `soft_rst` reader - Soft Reset all Registers and Logic"]
pub type SOFT_RST_R = crate::BitReader<SOFT_RST_A>;
#[doc = "Soft Reset all Registers and Logic\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFT_RST_A {
    #[doc = "0: `0`"]
    NO_VALID = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<SOFT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFT_RST_A {
        match self.bits {
            false => SOFT_RST_A::NO_VALID,
            true => SOFT_RST_A::RESET,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_valid(&self) -> bool {
        *self == SOFT_RST_A::NO_VALID
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RST_A::RESET
    }
}
#[doc = "Field `soft_rst` writer - Soft Reset all Registers and Logic"]
pub type SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG, SOFT_RST_A>;
impl<'a, REG> SOFT_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_valid(self) -> &'a mut crate::W<REG> {
        self.variant(SOFT_RST_A::NO_VALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SOFT_RST_A::RESET)
    }
}
#[doc = "Field `rx_tx_pri` reader - RX TX DMA Priority"]
pub type RX_TX_PRI_R = crate::BitReader<RX_TX_PRI_A>;
#[doc = "RX TX DMA Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_TX_PRI_A {
    #[doc = "0: `0`"]
    SAME = 0,
    #[doc = "1: `1`"]
    RO_T = 1,
}
impl From<RX_TX_PRI_A> for bool {
    #[inline(always)]
    fn from(variant: RX_TX_PRI_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_TX_PRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_TX_PRI_A {
        match self.bits {
            false => RX_TX_PRI_A::SAME,
            true => RX_TX_PRI_A::RO_T,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == RX_TX_PRI_A::SAME
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ro_t(&self) -> bool {
        *self == RX_TX_PRI_A::RO_T
    }
}
#[doc = "Field `rx_tx_pri` writer - RX TX DMA Priority"]
pub type RX_TX_PRI_W<'a, REG> = crate::BitWriter<'a, REG, RX_TX_PRI_A>;
impl<'a, REG> RX_TX_PRI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn same(self) -> &'a mut crate::W<REG> {
        self.variant(RX_TX_PRI_A::SAME)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ro_t(self) -> &'a mut crate::W<REG> {
        self.variant(RX_TX_PRI_A::RO_T)
    }
}
#[doc = "Field `burst_len` reader - The burst length of RX and TX DMA transfer"]
pub type BURST_LEN_R = crate::FieldReader;
#[doc = "Field `burst_len` writer - The burst length of RX and TX DMA transfer"]
pub type BURST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Soft Reset all Registers and Logic"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX TX DMA Priority"]
    #[inline(always)]
    pub fn rx_tx_pri(&self) -> RX_TX_PRI_R {
        RX_TX_PRI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 24:29 - The burst length of RX and TX DMA transfer"]
    #[inline(always)]
    pub fn burst_len(&self) -> BURST_LEN_R {
        BURST_LEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset all Registers and Logic"]
    #[inline(always)]
    #[must_use]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<EMAC_BASIC_CTL1_SPEC> {
        SOFT_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - RX TX DMA Priority"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tx_pri(&mut self) -> RX_TX_PRI_W<EMAC_BASIC_CTL1_SPEC> {
        RX_TX_PRI_W::new(self, 1)
    }
    #[doc = "Bits 24:29 - The burst length of RX and TX DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn burst_len(&mut self) -> BURST_LEN_W<EMAC_BASIC_CTL1_SPEC> {
        BURST_LEN_W::new(self, 24)
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
#[doc = "EMAC Basic Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_basic_ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_basic_ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_BASIC_CTL1_SPEC;
impl crate::RegisterSpec for EMAC_BASIC_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_basic_ctl1::R`](R) reader structure"]
impl crate::Readable for EMAC_BASIC_CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_basic_ctl1::W`](W) writer structure"]
impl crate::Writable for EMAC_BASIC_CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_basic_ctl1 to value 0"]
impl crate::Resettable for EMAC_BASIC_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
