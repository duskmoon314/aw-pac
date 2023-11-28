#[doc = "Register `emac_tx_flow_ctl` reader"]
pub type R = crate::R<EMAC_TX_FLOW_CTL_SPEC>;
#[doc = "Register `emac_tx_flow_ctl` writer"]
pub type W = crate::W<EMAC_TX_FLOW_CTL_SPEC>;
#[doc = "Field `tx_flow_ctl_en` reader - TX Flow Control Enable"]
pub type TX_FLOW_CTL_EN_R = crate::BitReader<TX_FLOW_CTL_EN_A>;
#[doc = "TX Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_FLOW_CTL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_FLOW_CTL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FLOW_CTL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_FLOW_CTL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_FLOW_CTL_EN_A {
        match self.bits {
            false => TX_FLOW_CTL_EN_A::DISABLE,
            true => TX_FLOW_CTL_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_FLOW_CTL_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_FLOW_CTL_EN_A::ENABLE
    }
}
#[doc = "Field `tx_flow_ctl_en` writer - TX Flow Control Enable"]
pub type TX_FLOW_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG, TX_FLOW_CTL_EN_A>;
impl<'a, REG> TX_FLOW_CTL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FLOW_CTL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FLOW_CTL_EN_A::ENABLE)
    }
}
#[doc = "Field `zqp_frm_en` reader - "]
pub type ZQP_FRM_EN_R = crate::BitReader<ZQP_FRM_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZQP_FRM_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ZQP_FRM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ZQP_FRM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ZQP_FRM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ZQP_FRM_EN_A {
        match self.bits {
            false => ZQP_FRM_EN_A::DISABLE,
            true => ZQP_FRM_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ZQP_FRM_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ZQP_FRM_EN_A::ENABLE
    }
}
#[doc = "Field `zqp_frm_en` writer - "]
pub type ZQP_FRM_EN_W<'a, REG> = crate::BitWriter<'a, REG, ZQP_FRM_EN_A>;
impl<'a, REG> ZQP_FRM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ZQP_FRM_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ZQP_FRM_EN_A::ENABLE)
    }
}
#[doc = "Field `pause_time` reader - "]
pub type PAUSE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `pause_time` writer - "]
pub type PAUSE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tx_pause_frm_slot` reader - "]
pub type TX_PAUSE_FRM_SLOT_R = crate::FieldReader;
#[doc = "Field `tx_pause_frm_slot` writer - "]
pub type TX_PAUSE_FRM_SLOT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tx_flow_ctl_sta` reader - "]
pub type TX_FLOW_CTL_STA_R = crate::BitReader;
#[doc = "Field `tx_flow_ctl_sta` writer - "]
pub type TX_FLOW_CTL_STA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Flow Control Enable"]
    #[inline(always)]
    pub fn tx_flow_ctl_en(&self) -> TX_FLOW_CTL_EN_R {
        TX_FLOW_CTL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn zqp_frm_en(&self) -> ZQP_FRM_EN_R {
        ZQP_FRM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn pause_time(&self) -> PAUSE_TIME_R {
        PAUSE_TIME_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tx_pause_frm_slot(&self) -> TX_PAUSE_FRM_SLOT_R {
        TX_PAUSE_FRM_SLOT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_flow_ctl_sta(&self) -> TX_FLOW_CTL_STA_R {
        TX_FLOW_CTL_STA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_flow_ctl_en(&mut self) -> TX_FLOW_CTL_EN_W<EMAC_TX_FLOW_CTL_SPEC> {
        TX_FLOW_CTL_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn zqp_frm_en(&mut self) -> ZQP_FRM_EN_W<EMAC_TX_FLOW_CTL_SPEC> {
        ZQP_FRM_EN_W::new(self, 1)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    #[must_use]
    pub fn pause_time(&mut self) -> PAUSE_TIME_W<EMAC_TX_FLOW_CTL_SPEC> {
        PAUSE_TIME_W::new(self, 4)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_frm_slot(&mut self) -> TX_PAUSE_FRM_SLOT_W<EMAC_TX_FLOW_CTL_SPEC> {
        TX_PAUSE_FRM_SLOT_W::new(self, 20)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_flow_ctl_sta(&mut self) -> TX_FLOW_CTL_STA_W<EMAC_TX_FLOW_CTL_SPEC> {
        TX_FLOW_CTL_STA_W::new(self, 31)
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
#[doc = "EMAC Transmit Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_flow_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_tx_flow_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_TX_FLOW_CTL_SPEC;
impl crate::RegisterSpec for EMAC_TX_FLOW_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_tx_flow_ctl::R`](R) reader structure"]
impl crate::Readable for EMAC_TX_FLOW_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_tx_flow_ctl::W`](W) writer structure"]
impl crate::Writable for EMAC_TX_FLOW_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_tx_flow_ctl to value 0"]
impl crate::Resettable for EMAC_TX_FLOW_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
