#[doc = "Register `TWI_DRV_SEND_FIFO_ACC` writer"]
pub struct W(crate::W<TWI_DRV_SEND_FIFO_ACC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_SEND_FIFO_ACC_SPEC>;
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
impl From<crate::W<TWI_DRV_SEND_FIFO_ACC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_SEND_FIFO_ACC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `send_data_fifo` writer - "]
pub struct SEND_DATA_FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_DATA_FIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn send_data_fifo(&mut self) -> SEND_DATA_FIFO_W {
        SEND_DATA_FIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Send Data FIFO Access Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_send_fifo_acc](index.html) module"]
pub struct TWI_DRV_SEND_FIFO_ACC_SPEC;
impl crate::RegisterSpec for TWI_DRV_SEND_FIFO_ACC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_send_fifo_acc::W](W) writer structure"]
impl crate::Writable for TWI_DRV_SEND_FIFO_ACC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_DRV_SEND_FIFO_ACC to value 0"]
impl crate::Resettable for TWI_DRV_SEND_FIFO_ACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
