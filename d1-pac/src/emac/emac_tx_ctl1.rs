#[doc = "Register `EMAC_TX_CTL1` reader"]
pub struct R(crate::R<EMAC_TX_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_TX_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_TX_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_TX_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_TX_CTL1` writer"]
pub struct W(crate::W<EMAC_TX_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_TX_CTL1_SPEC>;
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
impl From<crate::W<EMAC_TX_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_TX_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit DMA FSM Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_START_A {
    #[doc = "0: `0`"]
    NO_VALID = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<TX_DMA_START_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DMA_START` reader - Transmit DMA FSM Start"]
pub struct TX_DMA_START_R(crate::FieldReader<bool>);
impl TX_DMA_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_START_A {
        match self.bits {
            false => TX_DMA_START_A::NO_VALID,
            true => TX_DMA_START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VALID`"]
    #[inline(always)]
    pub fn is_no_valid(&self) -> bool {
        **self == TX_DMA_START_A::NO_VALID
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == TX_DMA_START_A::START
    }
}
impl core::ops::Deref for TX_DMA_START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_START` writer - Transmit DMA FSM Start"]
pub struct TX_DMA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DMA_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_valid(self) -> &'a mut W {
        self.variant(TX_DMA_START_A::NO_VALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TX_DMA_START_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Transmit DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_EN_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<TX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DMA_EN` reader - Transmit DMA Enable"]
pub struct TX_DMA_EN_R(crate::FieldReader<bool>);
impl TX_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_EN_A {
        match self.bits {
            false => TX_DMA_EN_A::STOP,
            true => TX_DMA_EN_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == TX_DMA_EN_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == TX_DMA_EN_A::START
    }
}
impl core::ops::Deref for TX_DMA_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_EN` writer - Transmit DMA Enable"]
pub struct TX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::START)
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Threshold value of TX DMA FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_TH_A {
    #[doc = "0: `0`"]
    T64 = 0,
    #[doc = "1: `1`"]
    T128 = 1,
    #[doc = "2: `10`"]
    T192 = 2,
    #[doc = "3: `11`"]
    T256 = 3,
}
impl From<TX_TH_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_TH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_TH` reader - Threshold value of TX DMA FIFO"]
pub struct TX_TH_R(crate::FieldReader<u8>);
impl TX_TH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_TH_A> {
        match self.bits {
            0 => Some(TX_TH_A::T64),
            1 => Some(TX_TH_A::T128),
            2 => Some(TX_TH_A::T192),
            3 => Some(TX_TH_A::T256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `T64`"]
    #[inline(always)]
    pub fn is_t64(&self) -> bool {
        **self == TX_TH_A::T64
    }
    #[doc = "Checks if the value of the field is `T128`"]
    #[inline(always)]
    pub fn is_t128(&self) -> bool {
        **self == TX_TH_A::T128
    }
    #[doc = "Checks if the value of the field is `T192`"]
    #[inline(always)]
    pub fn is_t192(&self) -> bool {
        **self == TX_TH_A::T192
    }
    #[doc = "Checks if the value of the field is `T256`"]
    #[inline(always)]
    pub fn is_t256(&self) -> bool {
        **self == TX_TH_A::T256
    }
}
impl core::ops::Deref for TX_TH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TH` writer - Threshold value of TX DMA FIFO"]
pub struct TX_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_TH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn t64(self) -> &'a mut W {
        self.variant(TX_TH_A::T64)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn t128(self) -> &'a mut W {
        self.variant(TX_TH_A::T128)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn t192(self) -> &'a mut W {
        self.variant(TX_TH_A::T192)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn t256(self) -> &'a mut W {
        self.variant(TX_TH_A::T256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Transmission Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_MD_A {
    #[doc = "0: `0`"]
    GREATER_THAN_TH = 0,
    #[doc = "1: `1`"]
    LOCATE_FULL_FRAME = 1,
}
impl From<TX_MD_A> for bool {
    #[inline(always)]
    fn from(variant: TX_MD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_MD` reader - Transmission Mode"]
pub struct TX_MD_R(crate::FieldReader<bool>);
impl TX_MD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_MD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_MD_A {
        match self.bits {
            false => TX_MD_A::GREATER_THAN_TH,
            true => TX_MD_A::LOCATE_FULL_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `GREATER_THAN_TH`"]
    #[inline(always)]
    pub fn is_greater_than_th(&self) -> bool {
        **self == TX_MD_A::GREATER_THAN_TH
    }
    #[doc = "Checks if the value of the field is `LOCATE_FULL_FRAME`"]
    #[inline(always)]
    pub fn is_locate_full_frame(&self) -> bool {
        **self == TX_MD_A::LOCATE_FULL_FRAME
    }
}
impl core::ops::Deref for TX_MD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MD` writer - Transmission Mode"]
pub struct TX_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_MD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn greater_than_th(self) -> &'a mut W {
        self.variant(TX_MD_A::GREATER_THAN_TH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locate_full_frame(self) -> &'a mut W {
        self.variant(TX_MD_A::LOCATE_FULL_FRAME)
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
#[doc = "Flush the data in the TX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLUSH_TX_FIFO_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<FLUSH_TX_FIFO_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_TX_FIFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH_TX_FIFO` reader - Flush the data in the TX FIFO"]
pub struct FLUSH_TX_FIFO_R(crate::FieldReader<bool>);
impl FLUSH_TX_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLUSH_TX_FIFO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLUSH_TX_FIFO_A {
        match self.bits {
            false => FLUSH_TX_FIFO_A::ENABLE,
            true => FLUSH_TX_FIFO_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FLUSH_TX_FIFO_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FLUSH_TX_FIFO_A::DISABLE
    }
}
impl core::ops::Deref for FLUSH_TX_FIFO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUSH_TX_FIFO` writer - Flush the data in the TX FIFO"]
pub struct FLUSH_TX_FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_TX_FIFO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLUSH_TX_FIFO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLUSH_TX_FIFO_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLUSH_TX_FIFO_A::DISABLE)
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
    #[doc = "Bit 31 - Transmit DMA FSM Start"]
    #[inline(always)]
    pub fn tx_dma_start(&self) -> TX_DMA_START_R {
        TX_DMA_START_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Threshold value of TX DMA FIFO"]
    #[inline(always)]
    pub fn tx_th(&self) -> TX_TH_R {
        TX_TH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 1 - Transmission Mode"]
    #[inline(always)]
    pub fn tx_md(&self) -> TX_MD_R {
        TX_MD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Flush the data in the TX FIFO"]
    #[inline(always)]
    pub fn flush_tx_fifo(&self) -> FLUSH_TX_FIFO_R {
        FLUSH_TX_FIFO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Transmit DMA FSM Start"]
    #[inline(always)]
    pub fn tx_dma_start(&mut self) -> TX_DMA_START_W {
        TX_DMA_START_W { w: self }
    }
    #[doc = "Bit 30 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W {
        TX_DMA_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - Threshold value of TX DMA FIFO"]
    #[inline(always)]
    pub fn tx_th(&mut self) -> TX_TH_W {
        TX_TH_W { w: self }
    }
    #[doc = "Bit 1 - Transmission Mode"]
    #[inline(always)]
    pub fn tx_md(&mut self) -> TX_MD_W {
        TX_MD_W { w: self }
    }
    #[doc = "Bit 0 - Flush the data in the TX FIFO"]
    #[inline(always)]
    pub fn flush_tx_fifo(&mut self) -> FLUSH_TX_FIFO_W {
        FLUSH_TX_FIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Transmit Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_tx_ctl1](index.html) module"]
pub struct EMAC_TX_CTL1_SPEC;
impl crate::RegisterSpec for EMAC_TX_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_tx_ctl1::R](R) reader structure"]
impl crate::Readable for EMAC_TX_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_tx_ctl1::W](W) writer structure"]
impl crate::Writable for EMAC_TX_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_TX_CTL1 to value 0"]
impl crate::Resettable for EMAC_TX_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
