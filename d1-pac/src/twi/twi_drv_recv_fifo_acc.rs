#[doc = "Register `TWI_DRV_RECV_FIFO_ACC` reader"]
pub struct R(crate::R<TWI_DRV_RECV_FIFO_ACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_RECV_FIFO_ACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_RECV_FIFO_ACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_RECV_FIFO_ACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `recv_data_fifo` reader - "]
pub struct RECV_DATA_FIFO_R(crate::FieldReader<u8>);
impl RECV_DATA_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECV_DATA_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECV_DATA_FIFO_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn recv_data_fifo(&self) -> RECV_DATA_FIFO_R {
        RECV_DATA_FIFO_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TWI_DRV Receive Data FIFO Access Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_recv_fifo_acc](index.html) module"]
pub struct TWI_DRV_RECV_FIFO_ACC_SPEC;
impl crate::RegisterSpec for TWI_DRV_RECV_FIFO_ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_recv_fifo_acc::R](R) reader structure"]
impl crate::Readable for TWI_DRV_RECV_FIFO_ACC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TWI_DRV_RECV_FIFO_ACC to value 0"]
impl crate::Resettable for TWI_DRV_RECV_FIFO_ACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
