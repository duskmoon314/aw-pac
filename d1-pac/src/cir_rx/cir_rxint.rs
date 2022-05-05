#[doc = "Register `CIR_RXINT` reader"]
pub struct R(crate::R<CIR_RXINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_RXINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_RXINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_RXINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR_RXINT` writer"]
pub struct W(crate::W<CIR_RXINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_RXINT_SPEC>;
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
impl From<crate::W<CIR_RXINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_RXINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAL` reader - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
pub struct RAL_R(crate::FieldReader<u8>);
impl RAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAL` writer - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
pub struct RAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "RX FIFO DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRQ_EN` reader - RX FIFO DMA Enable"]
pub struct DRQ_EN_R(crate::FieldReader<bool>);
impl DRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRQ_EN_A {
        match self.bits {
            false => DRQ_EN_A::DISABLE,
            true => DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for DRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRQ_EN` writer - RX FIFO DMA Enable"]
pub struct DRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "RX FIFO Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RAI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RAI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAI_EN` reader - RX FIFO Available Interrupt Enable"]
pub struct RAI_EN_R(crate::FieldReader<bool>);
impl RAI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAI_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAI_EN_A {
        match self.bits {
            false => RAI_EN_A::DISABLE,
            true => RAI_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAI_EN_A::ENABLE
    }
}
impl core::ops::Deref for RAI_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAI_EN` writer - RX FIFO Available Interrupt Enable"]
pub struct RAI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAI_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAI_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Receiver Packet End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPEI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RPEI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RPEI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEI_EN` reader - Receiver Packet End Interrupt Enable"]
pub struct RPEI_EN_R(crate::FieldReader<bool>);
impl RPEI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPEI_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPEI_EN_A {
        match self.bits {
            false => RPEI_EN_A::DISABLE,
            true => RPEI_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RPEI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RPEI_EN_A::ENABLE
    }
}
impl core::ops::Deref for RPEI_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPEI_EN` writer - Receiver Packet End Interrupt Enable"]
pub struct RPEI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RPEI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPEI_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RPEI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RPEI_EN_A::ENABLE)
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
#[doc = "Receiver FIFO Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROI_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ROI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ROI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROI_EN` reader - Receiver FIFO Overrun Interrupt Enable"]
pub struct ROI_EN_R(crate::FieldReader<bool>);
impl ROI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROI_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROI_EN_A {
        match self.bits {
            false => ROI_EN_A::DISABLE,
            true => ROI_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ROI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ROI_EN_A::ENABLE
    }
}
impl core::ops::Deref for ROI_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROI_EN` writer - Receiver FIFO Overrun Interrupt Enable"]
pub struct ROI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROI_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROI_EN_A::ENABLE)
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
    #[doc = "Bits 8:13 - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
    #[inline(always)]
    pub fn ral(&self) -> RAL_R {
        RAL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 5 - RX FIFO DMA Enable"]
    #[inline(always)]
    pub fn drq_en(&self) -> DRQ_EN_R {
        DRQ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Available Interrupt Enable"]
    #[inline(always)]
    pub fn rai_en(&self) -> RAI_EN_R {
        RAI_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Packet End Interrupt Enable"]
    #[inline(always)]
    pub fn rpei_en(&self) -> RPEI_EN_R {
        RPEI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Receiver FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn roi_en(&self) -> ROI_EN_R {
        ROI_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - RX FIFO available received byte level for interrupt and DMA request\n\nTRIGGER_LEVEL = RAL + 1"]
    #[inline(always)]
    pub fn ral(&mut self) -> RAL_W {
        RAL_W { w: self }
    }
    #[doc = "Bit 5 - RX FIFO DMA Enable"]
    #[inline(always)]
    pub fn drq_en(&mut self) -> DRQ_EN_W {
        DRQ_EN_W { w: self }
    }
    #[doc = "Bit 4 - RX FIFO Available Interrupt Enable"]
    #[inline(always)]
    pub fn rai_en(&mut self) -> RAI_EN_W {
        RAI_EN_W { w: self }
    }
    #[doc = "Bit 1 - Receiver Packet End Interrupt Enable"]
    #[inline(always)]
    pub fn rpei_en(&mut self) -> RPEI_EN_W {
        RPEI_EN_W { w: self }
    }
    #[doc = "Bit 0 - Receiver FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn roi_en(&mut self) -> ROI_EN_W {
        ROI_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Receiver Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_rxint](index.html) module"]
pub struct CIR_RXINT_SPEC;
impl crate::RegisterSpec for CIR_RXINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_rxint::R](R) reader structure"]
impl crate::Readable for CIR_RXINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_rxint::W](W) writer structure"]
impl crate::Writable for CIR_RXINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR_RXINT to value 0"]
impl crate::Resettable for CIR_RXINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
