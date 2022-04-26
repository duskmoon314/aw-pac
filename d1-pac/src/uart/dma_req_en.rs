#[doc = "Register `DMA_REQ_EN` reader"]
pub struct R(crate::R<DMA_REQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_REQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_REQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_REQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_REQ_EN` writer"]
pub struct W(crate::W<DMA_REQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_REQ_EN_SPEC>;
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
impl From<crate::W<DMA_REQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_REQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Timeout Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TIMEOUT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `timeout_enable` reader - DMA Timeout Enable"]
pub struct TIMEOUT_ENABLE_R(crate::FieldReader<bool>);
impl TIMEOUT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_ENABLE_A {
        match self.bits {
            false => TIMEOUT_ENABLE_A::DISABLE,
            true => TIMEOUT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TIMEOUT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TIMEOUT_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for TIMEOUT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timeout_enable` writer - DMA Timeout Enable"]
pub struct TIMEOUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMEOUT_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMEOUT_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "DMA TX REQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_REQ_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_REQ_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_REQ_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_req_enable` reader - DMA TX REQ Enable"]
pub struct TX_REQ_ENABLE_R(crate::FieldReader<bool>);
impl TX_REQ_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_REQ_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_REQ_ENABLE_A {
        match self.bits {
            false => TX_REQ_ENABLE_A::DISABLE,
            true => TX_REQ_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_REQ_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_REQ_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for TX_REQ_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_req_enable` writer - DMA TX REQ Enable"]
pub struct TX_REQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REQ_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_REQ_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_REQ_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_REQ_ENABLE_A::ENABLE)
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
#[doc = "DMA RX REQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_REQ_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_REQ_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_REQ_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rx_req_enable` reader - DMA RX REQ Enable"]
pub struct RX_REQ_ENABLE_R(crate::FieldReader<bool>);
impl RX_REQ_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_REQ_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_REQ_ENABLE_A {
        match self.bits {
            false => RX_REQ_ENABLE_A::DISABLE,
            true => RX_REQ_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_REQ_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_REQ_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for RX_REQ_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_req_enable` writer - DMA RX REQ Enable"]
pub struct RX_REQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REQ_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_REQ_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_REQ_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_REQ_ENABLE_A::ENABLE)
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
    #[doc = "Bit 2 - DMA Timeout Enable"]
    #[inline(always)]
    pub fn timeout_enable(&self) -> TIMEOUT_ENABLE_R {
        TIMEOUT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - DMA TX REQ Enable"]
    #[inline(always)]
    pub fn tx_req_enable(&self) -> TX_REQ_ENABLE_R {
        TX_REQ_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - DMA RX REQ Enable"]
    #[inline(always)]
    pub fn rx_req_enable(&self) -> RX_REQ_ENABLE_R {
        RX_REQ_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA Timeout Enable"]
    #[inline(always)]
    pub fn timeout_enable(&mut self) -> TIMEOUT_ENABLE_W {
        TIMEOUT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - DMA TX REQ Enable"]
    #[inline(always)]
    pub fn tx_req_enable(&mut self) -> TX_REQ_ENABLE_W {
        TX_REQ_ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - DMA RX REQ Enable"]
    #[inline(always)]
    pub fn rx_req_enable(&mut self) -> RX_REQ_ENABLE_W {
        RX_REQ_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART DMA Request Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_req_en](index.html) module"]
pub struct DMA_REQ_EN_SPEC;
impl crate::RegisterSpec for DMA_REQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_req_en::R](R) reader structure"]
impl crate::Readable for DMA_REQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_req_en::W](W) writer structure"]
impl crate::Writable for DMA_REQ_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_REQ_EN to value 0"]
impl crate::Resettable for DMA_REQ_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
