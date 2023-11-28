#[doc = "Register `twi_drv_int_ctrl` reader"]
pub type R = crate::R<TWI_DRV_INT_CTRL_SPEC>;
#[doc = "Register `twi_drv_int_ctrl` writer"]
pub type W = crate::W<TWI_DRV_INT_CTRL_SPEC>;
#[doc = "Field `tran_com_pd` reader - "]
pub type TRAN_COM_PD_R = crate::BitReader;
#[doc = "Field `tran_com_pd` writer - "]
pub type TRAN_COM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tran_err_pd` reader - "]
pub type TRAN_ERR_PD_R = crate::BitReader;
#[doc = "Field `tran_err_pd` writer - "]
pub type TRAN_ERR_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_req_pd` reader - "]
pub type TX_REQ_PD_R = crate::BitReader;
#[doc = "Field `tx_req_pd` writer - "]
pub type TX_REQ_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_req_pd` reader - "]
pub type RX_REQ_PD_R = crate::BitReader;
#[doc = "Field `rx_req_pd` writer - "]
pub type RX_REQ_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tran_com_int_en` reader - "]
pub type TRAN_COM_INT_EN_R = crate::BitReader;
#[doc = "Field `tran_com_int_en` writer - "]
pub type TRAN_COM_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tran_err_int_en` reader - "]
pub type TRAN_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `tran_err_int_en` writer - "]
pub type TRAN_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_req_int_en` reader - "]
pub type TX_REQ_INT_EN_R = crate::BitReader;
#[doc = "Field `tx_req_int_en` writer - "]
pub type TX_REQ_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_req_int_en` reader - "]
pub type RX_REQ_INT_EN_R = crate::BitReader;
#[doc = "Field `rx_req_int_en` writer - "]
pub type RX_REQ_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tran_com_pd(&self) -> TRAN_COM_PD_R {
        TRAN_COM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tran_err_pd(&self) -> TRAN_ERR_PD_R {
        TRAN_ERR_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_req_pd(&self) -> TX_REQ_PD_R {
        TX_REQ_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_req_pd(&self) -> RX_REQ_PD_R {
        RX_REQ_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tran_com_int_en(&self) -> TRAN_COM_INT_EN_R {
        TRAN_COM_INT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tran_err_int_en(&self) -> TRAN_ERR_INT_EN_R {
        TRAN_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_req_int_en(&self) -> TX_REQ_INT_EN_R {
        TX_REQ_INT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_req_int_en(&self) -> RX_REQ_INT_EN_R {
        RX_REQ_INT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tran_com_pd(&mut self) -> TRAN_COM_PD_W<TWI_DRV_INT_CTRL_SPEC> {
        TRAN_COM_PD_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tran_err_pd(&mut self) -> TRAN_ERR_PD_W<TWI_DRV_INT_CTRL_SPEC> {
        TRAN_ERR_PD_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req_pd(&mut self) -> TX_REQ_PD_W<TWI_DRV_INT_CTRL_SPEC> {
        TX_REQ_PD_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_pd(&mut self) -> RX_REQ_PD_W<TWI_DRV_INT_CTRL_SPEC> {
        RX_REQ_PD_W::new(self, 3)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tran_com_int_en(&mut self) -> TRAN_COM_INT_EN_W<TWI_DRV_INT_CTRL_SPEC> {
        TRAN_COM_INT_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tran_err_int_en(&mut self) -> TRAN_ERR_INT_EN_W<TWI_DRV_INT_CTRL_SPEC> {
        TRAN_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req_int_en(&mut self) -> TX_REQ_INT_EN_W<TWI_DRV_INT_CTRL_SPEC> {
        TX_REQ_INT_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_int_en(&mut self) -> RX_REQ_INT_EN_W<TWI_DRV_INT_CTRL_SPEC> {
        RX_REQ_INT_EN_W::new(self, 19)
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
#[doc = "TWI_DRV Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_INT_CTRL_SPEC;
impl crate::RegisterSpec for TWI_DRV_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_int_ctrl::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_INT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_int_ctrl::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_INT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_int_ctrl to value 0"]
impl crate::Resettable for TWI_DRV_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
