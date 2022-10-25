#[doc = "Register `emac_tx_flow_ctl` reader"]
pub struct R(crate::R<EMAC_TX_FLOW_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_TX_FLOW_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_TX_FLOW_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_TX_FLOW_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_tx_flow_ctl` writer"]
pub struct W(crate::W<EMAC_TX_FLOW_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_TX_FLOW_CTL_SPEC>;
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
impl From<crate::W<EMAC_TX_FLOW_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_TX_FLOW_CTL_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> TX_FLOW_CTL_EN_A {
        match self.bits {
            false => TX_FLOW_CTL_EN_A::DISABLE,
            true => TX_FLOW_CTL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_FLOW_CTL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_FLOW_CTL_EN_A::ENABLE
    }
}
#[doc = "Field `tx_flow_ctl_en` writer - TX Flow Control Enable"]
pub type TX_FLOW_CTL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_TX_FLOW_CTL_SPEC, TX_FLOW_CTL_EN_A, O>;
impl<'a, const O: u8> TX_FLOW_CTL_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_FLOW_CTL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn variant(&self) -> ZQP_FRM_EN_A {
        match self.bits {
            false => ZQP_FRM_EN_A::DISABLE,
            true => ZQP_FRM_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ZQP_FRM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ZQP_FRM_EN_A::ENABLE
    }
}
#[doc = "Field `zqp_frm_en` writer - "]
pub type ZQP_FRM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_TX_FLOW_CTL_SPEC, ZQP_FRM_EN_A, O>;
impl<'a, const O: u8> ZQP_FRM_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ZQP_FRM_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ZQP_FRM_EN_A::ENABLE)
    }
}
#[doc = "Field `pause_time` reader - "]
pub type PAUSE_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pause_time` writer - "]
pub type PAUSE_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMAC_TX_FLOW_CTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `tx_pause_frm_slot` reader - "]
pub type TX_PAUSE_FRM_SLOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_pause_frm_slot` writer - "]
pub type TX_PAUSE_FRM_SLOT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMAC_TX_FLOW_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `tx_flow_ctl_sta` reader - "]
pub type TX_FLOW_CTL_STA_R = crate::BitReader<bool>;
#[doc = "Field `tx_flow_ctl_sta` writer - "]
pub type TX_FLOW_CTL_STA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_TX_FLOW_CTL_SPEC, bool, O>;
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
    pub fn tx_flow_ctl_en(&mut self) -> TX_FLOW_CTL_EN_W<0> {
        TX_FLOW_CTL_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn zqp_frm_en(&mut self) -> ZQP_FRM_EN_W<1> {
        ZQP_FRM_EN_W::new(self)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    #[must_use]
    pub fn pause_time(&mut self) -> PAUSE_TIME_W<4> {
        PAUSE_TIME_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_frm_slot(&mut self) -> TX_PAUSE_FRM_SLOT_W<20> {
        TX_PAUSE_FRM_SLOT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_flow_ctl_sta(&mut self) -> TX_FLOW_CTL_STA_W<31> {
        TX_FLOW_CTL_STA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Transmit Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_tx_flow_ctl](index.html) module"]
pub struct EMAC_TX_FLOW_CTL_SPEC;
impl crate::RegisterSpec for EMAC_TX_FLOW_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_tx_flow_ctl::R](R) reader structure"]
impl crate::Readable for EMAC_TX_FLOW_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_tx_flow_ctl::W](W) writer structure"]
impl crate::Writable for EMAC_TX_FLOW_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_tx_flow_ctl to value 0"]
impl crate::Resettable for EMAC_TX_FLOW_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
