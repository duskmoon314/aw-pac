#[doc = "Register `twi_drv_send_fifo_acc` writer"]
pub type W = crate::W<TWI_DRV_SEND_FIFO_ACC_SPEC>;
#[doc = "Field `send_data_fifo` writer - "]
pub type SEND_DATA_FIFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn send_data_fifo(&mut self) -> SEND_DATA_FIFO_W<TWI_DRV_SEND_FIFO_ACC_SPEC> {
        SEND_DATA_FIFO_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWI_DRV Send Data FIFO Access Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_send_fifo_acc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_SEND_FIFO_ACC_SPEC;
impl crate::RegisterSpec for TWI_DRV_SEND_FIFO_ACC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`twi_drv_send_fifo_acc::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_SEND_FIFO_ACC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_send_fifo_acc to value 0"]
impl crate::Resettable for TWI_DRV_SEND_FIFO_ACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
