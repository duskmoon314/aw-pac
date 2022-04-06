#[doc = "Register `EMAC_BASIC_CTL1` reader"]
pub struct R(crate::R<EMAC_BASIC_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_BASIC_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_BASIC_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_BASIC_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_BASIC_CTL1` writer"]
pub struct W(crate::W<EMAC_BASIC_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_BASIC_CTL1_SPEC>;
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
impl From<crate::W<EMAC_BASIC_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_BASIC_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BURST_LEN` reader - The burst length of RX and TX DMA transfer"]
pub struct BURST_LEN_R(crate::FieldReader<u8, u8>);
impl BURST_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BURST_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURST_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST_LEN` writer - The burst length of RX and TX DMA transfer"]
pub struct BURST_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "RX TX DMA Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TX_PRI_A {
    #[doc = "0: `0`"]
    SAME = 0,
    #[doc = "1: `1`"]
    ROT = 1,
}
impl From<RX_TX_PRI_A> for bool {
    #[inline(always)]
    fn from(variant: RX_TX_PRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_TX_PRI` reader - RX TX DMA Priority"]
pub struct RX_TX_PRI_R(crate::FieldReader<bool, RX_TX_PRI_A>);
impl RX_TX_PRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TX_PRI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_TX_PRI_A {
        match self.bits {
            false => RX_TX_PRI_A::SAME,
            true => RX_TX_PRI_A::ROT,
        }
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        **self == RX_TX_PRI_A::SAME
    }
    #[doc = "Checks if the value of the field is `ROT`"]
    #[inline(always)]
    pub fn is_ro_t(&self) -> bool {
        **self == RX_TX_PRI_A::ROT
    }
}
impl core::ops::Deref for RX_TX_PRI_R {
    type Target = crate::FieldReader<bool, RX_TX_PRI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TX_PRI` writer - RX TX DMA Priority"]
pub struct RX_TX_PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TX_PRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_TX_PRI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn same(self) -> &'a mut W {
        self.variant(RX_TX_PRI_A::SAME)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ro_t(self) -> &'a mut W {
        self.variant(RX_TX_PRI_A::ROT)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Soft Reset all Registers and Logic\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SOFT_RST` reader - Soft Reset all Registers and Logic"]
pub struct SOFT_RST_R(crate::FieldReader<bool, SOFT_RST_A>);
impl SOFT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFT_RST_A {
        match self.bits {
            false => SOFT_RST_A::NO_VALID,
            true => SOFT_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VALID`"]
    #[inline(always)]
    pub fn is_no_valid(&self) -> bool {
        **self == SOFT_RST_A::NO_VALID
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SOFT_RST_A::RESET
    }
}
impl core::ops::Deref for SOFT_RST_R {
    type Target = crate::FieldReader<bool, SOFT_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_RST` writer - Soft Reset all Registers and Logic"]
pub struct SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFT_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_valid(self) -> &'a mut W {
        self.variant(SOFT_RST_A::NO_VALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOFT_RST_A::RESET)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - The burst length of RX and TX DMA transfer"]
    #[inline(always)]
    pub fn burst_len(&self) -> BURST_LEN_R {
        BURST_LEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - RX TX DMA Priority"]
    #[inline(always)]
    pub fn rx_tx_pri(&self) -> RX_TX_PRI_R {
        RX_TX_PRI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Soft Reset all Registers and Logic"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:29 - The burst length of RX and TX DMA transfer"]
    #[inline(always)]
    pub fn burst_len(&mut self) -> BURST_LEN_W {
        BURST_LEN_W { w: self }
    }
    #[doc = "Bit 1 - RX TX DMA Priority"]
    #[inline(always)]
    pub fn rx_tx_pri(&mut self) -> RX_TX_PRI_W {
        RX_TX_PRI_W { w: self }
    }
    #[doc = "Bit 0 - Soft Reset all Registers and Logic"]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W {
        SOFT_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Basic Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_basic_ctl1](index.html) module"]
pub struct EMAC_BASIC_CTL1_SPEC;
impl crate::RegisterSpec for EMAC_BASIC_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_basic_ctl1::R](R) reader structure"]
impl crate::Readable for EMAC_BASIC_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_basic_ctl1::W](W) writer structure"]
impl crate::Writable for EMAC_BASIC_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_BASIC_CTL1 to value 0"]
impl crate::Resettable for EMAC_BASIC_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
