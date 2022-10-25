#[doc = "Register `twi_drv_fifo_con` reader"]
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
#[doc = "Register `twi_drv_fifo_con` writer"]
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
#[doc = "Field `send_fifo_content` reader - "]
pub type SEND_FIFO_CONTENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `send_fifo_content` writer - "]
pub type SEND_FIFO_CONTENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_FIFO_CON_SPEC, u8, u8, 6, O>;
#[doc = "Field `send_fifo_clear` reader - "]
pub type SEND_FIFO_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `send_fifo_clear` writer - "]
pub type SEND_FIFO_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_FIFO_CON_SPEC, bool, O>;
#[doc = "Field `recv_fifo_content` reader - "]
pub type RECV_FIFO_CONTENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `recv_fifo_content` writer - "]
pub type RECV_FIFO_CONTENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_FIFO_CON_SPEC, u8, u8, 6, O>;
#[doc = "Field `recv_fifo_clear` reader - "]
pub type RECV_FIFO_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `recv_fifo_clear` writer - "]
pub type RECV_FIFO_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_FIFO_CON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn send_fifo_content(&self) -> SEND_FIFO_CONTENT_R {
        SEND_FIFO_CONTENT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn send_fifo_clear(&self) -> SEND_FIFO_CLEAR_R {
        SEND_FIFO_CLEAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn recv_fifo_content(&self) -> RECV_FIFO_CONTENT_R {
        RECV_FIFO_CONTENT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn recv_fifo_clear(&self) -> RECV_FIFO_CLEAR_R {
        RECV_FIFO_CLEAR_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn send_fifo_content(&mut self) -> SEND_FIFO_CONTENT_W<0> {
        SEND_FIFO_CONTENT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn send_fifo_clear(&mut self) -> SEND_FIFO_CLEAR_W<6> {
        SEND_FIFO_CLEAR_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn recv_fifo_content(&mut self) -> RECV_FIFO_CONTENT_W<16> {
        RECV_FIFO_CONTENT_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn recv_fifo_clear(&mut self) -> RECV_FIFO_CLEAR_W<22> {
        RECV_FIFO_CLEAR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_fifo_con to value 0"]
impl crate::Resettable for TWI_DRV_FIFO_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
