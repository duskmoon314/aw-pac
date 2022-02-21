#[doc = "Register `TWI_DRV_FIFO_CON` reader"]
pub struct R(crate::R<TWI_DRV_FIFO_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_FIFO_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_FIFO_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_FIFO_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_DRV_FIFO_CON` writer"]
pub struct W(crate::W<TWI_DRV_FIFO_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_FIFO_CON_SPEC>;
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
impl From<crate::W<TWI_DRV_FIFO_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_FIFO_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `recv_fifo_clear` reader - "]
pub struct RECV_FIFO_CLEAR_R(crate::FieldReader<bool, bool>);
impl RECV_FIFO_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECV_FIFO_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECV_FIFO_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `recv_fifo_clear` writer - "]
pub struct RECV_FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RECV_FIFO_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `recv_fifo_content` reader - "]
pub struct RECV_FIFO_CONTENT_R(crate::FieldReader<u8, u8>);
impl RECV_FIFO_CONTENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECV_FIFO_CONTENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECV_FIFO_CONTENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `recv_fifo_content` writer - "]
pub struct RECV_FIFO_CONTENT_W<'a> {
    w: &'a mut W,
}
impl<'a> RECV_FIFO_CONTENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `send_fifo_clear` reader - "]
pub struct SEND_FIFO_CLEAR_R(crate::FieldReader<bool, bool>);
impl SEND_FIFO_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEND_FIFO_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_FIFO_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `send_fifo_clear` writer - "]
pub struct SEND_FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_FIFO_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `send_fifo_content` reader - "]
pub struct SEND_FIFO_CONTENT_R(crate::FieldReader<u8, u8>);
impl SEND_FIFO_CONTENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEND_FIFO_CONTENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_FIFO_CONTENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `send_fifo_content` writer - "]
pub struct SEND_FIFO_CONTENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_FIFO_CONTENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn recv_fifo_clear(&self) -> RECV_FIFO_CLEAR_R {
        RECV_FIFO_CLEAR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn recv_fifo_content(&self) -> RECV_FIFO_CONTENT_R {
        RECV_FIFO_CONTENT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn send_fifo_clear(&self) -> SEND_FIFO_CLEAR_R {
        SEND_FIFO_CLEAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn send_fifo_content(&self) -> SEND_FIFO_CONTENT_R {
        SEND_FIFO_CONTENT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn recv_fifo_clear(&mut self) -> RECV_FIFO_CLEAR_W {
        RECV_FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn recv_fifo_content(&mut self) -> RECV_FIFO_CONTENT_W {
        RECV_FIFO_CONTENT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn send_fifo_clear(&mut self) -> SEND_FIFO_CLEAR_W {
        SEND_FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn send_fifo_content(&mut self) -> SEND_FIFO_CONTENT_W {
        SEND_FIFO_CONTENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV FIFO Content Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_fifo_con](index.html) module"]
pub struct TWI_DRV_FIFO_CON_SPEC;
impl crate::RegisterSpec for TWI_DRV_FIFO_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_fifo_con::R](R) reader structure"]
impl crate::Readable for TWI_DRV_FIFO_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_fifo_con::W](W) writer structure"]
impl crate::Writable for TWI_DRV_FIFO_CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_DRV_FIFO_CON to value 0"]
impl crate::Resettable for TWI_DRV_FIFO_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
