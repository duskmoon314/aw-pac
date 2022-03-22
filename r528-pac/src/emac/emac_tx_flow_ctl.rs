#[doc = "Register `EMAC_TX_FLOW_CTL` reader"]
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
#[doc = "Register `EMAC_TX_FLOW_CTL` writer"]
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
#[doc = "Field `TX_FLOW_CTL_STA` reader - "]
pub struct TX_FLOW_CTL_STA_R(crate::FieldReader<bool, bool>);
impl TX_FLOW_CTL_STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FLOW_CTL_STA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FLOW_CTL_STA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FLOW_CTL_STA` writer - "]
pub struct TX_FLOW_CTL_STA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLOW_CTL_STA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `TX_PAUSE_FRM_SLOT` reader - "]
pub struct TX_PAUSE_FRM_SLOT_R(crate::FieldReader<u8, u8>);
impl TX_PAUSE_FRM_SLOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PAUSE_FRM_SLOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PAUSE_FRM_SLOT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PAUSE_FRM_SLOT` writer - "]
pub struct TX_PAUSE_FRM_SLOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PAUSE_FRM_SLOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `PAUSE_TIME` reader - "]
pub struct PAUSE_TIME_R(crate::FieldReader<u16, u16>);
impl PAUSE_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAUSE_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSE_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSE_TIME` writer - "]
pub struct PAUSE_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 4)) | ((value as u32 & 0xffff) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ZQP_FRM_EN` reader - "]
pub struct ZQP_FRM_EN_R(crate::FieldReader<bool, ZQP_FRM_EN_A>);
impl ZQP_FRM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ZQP_FRM_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ZQP_FRM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ZQP_FRM_EN_A::ENABLE
    }
}
impl core::ops::Deref for ZQP_FRM_EN_R {
    type Target = crate::FieldReader<bool, ZQP_FRM_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQP_FRM_EN` writer - "]
pub struct ZQP_FRM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQP_FRM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZQP_FRM_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "TX Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TX_FLOW_CTL_EN` reader - TX Flow Control Enable"]
pub struct TX_FLOW_CTL_EN_R(crate::FieldReader<bool, TX_FLOW_CTL_EN_A>);
impl TX_FLOW_CTL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FLOW_CTL_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TX_FLOW_CTL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_FLOW_CTL_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_FLOW_CTL_EN_R {
    type Target = crate::FieldReader<bool, TX_FLOW_CTL_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FLOW_CTL_EN` writer - TX Flow Control Enable"]
pub struct TX_FLOW_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLOW_CTL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FLOW_CTL_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_flow_ctl_sta(&self) -> TX_FLOW_CTL_STA_R {
        TX_FLOW_CTL_STA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tx_pause_frm_slot(&self) -> TX_PAUSE_FRM_SLOT_R {
        TX_PAUSE_FRM_SLOT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn pause_time(&self) -> PAUSE_TIME_R {
        PAUSE_TIME_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn zqp_frm_en(&self) -> ZQP_FRM_EN_R {
        ZQP_FRM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TX Flow Control Enable"]
    #[inline(always)]
    pub fn tx_flow_ctl_en(&self) -> TX_FLOW_CTL_EN_R {
        TX_FLOW_CTL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_flow_ctl_sta(&mut self) -> TX_FLOW_CTL_STA_W {
        TX_FLOW_CTL_STA_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tx_pause_frm_slot(&mut self) -> TX_PAUSE_FRM_SLOT_W {
        TX_PAUSE_FRM_SLOT_W { w: self }
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn pause_time(&mut self) -> PAUSE_TIME_W {
        PAUSE_TIME_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn zqp_frm_en(&mut self) -> ZQP_FRM_EN_W {
        ZQP_FRM_EN_W { w: self }
    }
    #[doc = "Bit 0 - TX Flow Control Enable"]
    #[inline(always)]
    pub fn tx_flow_ctl_en(&mut self) -> TX_FLOW_CTL_EN_W {
        TX_FLOW_CTL_EN_W { w: self }
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
}
#[doc = "`reset()` method sets EMAC_TX_FLOW_CTL to value 0"]
impl crate::Resettable for EMAC_TX_FLOW_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
