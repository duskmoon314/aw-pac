#[doc = "Register `twi_drv_recv_fifo_acc` reader"]
pub type R = crate::R<TWI_DRV_RECV_FIFO_ACC_SPEC>;
#[doc = "Field `recv_data_fifo` reader - "]
pub type RECV_DATA_FIFO_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn recv_data_fifo(&self) -> RECV_DATA_FIFO_R {
        RECV_DATA_FIFO_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TWI_DRV Receive Data FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_recv_fifo_acc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_RECV_FIFO_ACC_SPEC;
impl crate::RegisterSpec for TWI_DRV_RECV_FIFO_ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_recv_fifo_acc::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_RECV_FIFO_ACC_SPEC {}
#[doc = "`reset()` method sets twi_drv_recv_fifo_acc to value 0"]
impl crate::Resettable for TWI_DRV_RECV_FIFO_ACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
