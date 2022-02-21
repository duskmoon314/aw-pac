#[doc = "Register `RXDMA_IE` reader"]
pub struct R(crate::R<RXDMA_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_IE` writer"]
pub struct W(crate::W<RXDMA_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_IE_SPEC>;
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
impl From<crate::W<RXDMA_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buffer_overrun` reader - "]
pub struct BUFFER_OVERRUN_R(crate::FieldReader<bool, bool>);
impl BUFFER_OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUFFER_OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFFER_OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buffer_overrun` writer - "]
pub struct BUFFER_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_OVERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `timeout_done` reader - "]
pub struct TIMEOUT_DONE_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timeout_done` writer - "]
pub struct TIMEOUT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `blk_done` reader - "]
pub struct BLK_DONE_R(crate::FieldReader<bool, bool>);
impl BLK_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLK_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `blk_done` writer - "]
pub struct BLK_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_DONE_W<'a> {
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
#[doc = "Field `limit_done` reader - "]
pub struct LIMIT_DONE_R(crate::FieldReader<bool, bool>);
impl LIMIT_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIMIT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIMIT_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `limit_done` writer - "]
pub struct LIMIT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_DONE_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn buffer_overrun(&self) -> BUFFER_OVERRUN_R {
        BUFFER_OVERRUN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timeout_done(&self) -> TIMEOUT_DONE_R {
        TIMEOUT_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn blk_done(&self) -> BLK_DONE_R {
        BLK_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limit_done(&self) -> LIMIT_DONE_R {
        LIMIT_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn buffer_overrun(&mut self) -> BUFFER_OVERRUN_W {
        BUFFER_OVERRUN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timeout_done(&mut self) -> TIMEOUT_DONE_W {
        TIMEOUT_DONE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn blk_done(&mut self) -> BLK_DONE_W {
        BLK_DONE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limit_done(&mut self) -> LIMIT_DONE_W {
        LIMIT_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_ie](index.html) module"]
pub struct RXDMA_IE_SPEC;
impl crate::RegisterSpec for RXDMA_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_ie::R](R) reader structure"]
impl crate::Readable for RXDMA_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_ie::W](W) writer structure"]
impl crate::Writable for RXDMA_IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_IE to value 0"]
impl crate::Resettable for RXDMA_IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
