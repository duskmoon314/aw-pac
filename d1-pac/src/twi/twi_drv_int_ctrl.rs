#[doc = "Register `twi_drv_int_ctrl` reader"]
pub struct R(crate::R<TWI_DRV_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_drv_int_ctrl` writer"]
pub struct W(crate::W<TWI_DRV_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_INT_CTRL_SPEC>;
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
impl From<crate::W<TWI_DRV_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tran_com_pd` reader - "]
pub type TRAN_COM_PD_R = crate::BitReader<bool>;
#[doc = "Field `tran_com_pd` writer - "]
pub type TRAN_COM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `tran_err_pd` reader - "]
pub type TRAN_ERR_PD_R = crate::BitReader<bool>;
#[doc = "Field `tran_err_pd` writer - "]
pub type TRAN_ERR_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `tx_req_pd` reader - "]
pub type TX_REQ_PD_R = crate::BitReader<bool>;
#[doc = "Field `tx_req_pd` writer - "]
pub type TX_REQ_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `rx_req_pd` reader - "]
pub type RX_REQ_PD_R = crate::BitReader<bool>;
#[doc = "Field `rx_req_pd` writer - "]
pub type RX_REQ_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `tran_com_int_en` reader - "]
pub type TRAN_COM_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `tran_com_int_en` writer - "]
pub type TRAN_COM_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `tran_err_int_en` reader - "]
pub type TRAN_ERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `tran_err_int_en` writer - "]
pub type TRAN_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `tx_req_int_en` reader - "]
pub type TX_REQ_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_req_int_en` writer - "]
pub type TX_REQ_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `rx_req_int_en` reader - "]
pub type RX_REQ_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_req_int_en` writer - "]
pub type RX_REQ_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_INT_CTRL_SPEC, bool, O>;
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
    pub fn tran_com_pd(&mut self) -> TRAN_COM_PD_W<0> {
        TRAN_COM_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tran_err_pd(&mut self) -> TRAN_ERR_PD_W<1> {
        TRAN_ERR_PD_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req_pd(&mut self) -> TX_REQ_PD_W<2> {
        TX_REQ_PD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_pd(&mut self) -> RX_REQ_PD_W<3> {
        RX_REQ_PD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tran_com_int_en(&mut self) -> TRAN_COM_INT_EN_W<16> {
        TRAN_COM_INT_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tran_err_int_en(&mut self) -> TRAN_ERR_INT_EN_W<17> {
        TRAN_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req_int_en(&mut self) -> TX_REQ_INT_EN_W<18> {
        TX_REQ_INT_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_int_en(&mut self) -> RX_REQ_INT_EN_W<19> {
        RX_REQ_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_int_ctrl](index.html) module"]
pub struct TWI_DRV_INT_CTRL_SPEC;
impl crate::RegisterSpec for TWI_DRV_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_int_ctrl::R](R) reader structure"]
impl crate::Readable for TWI_DRV_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_int_ctrl::W](W) writer structure"]
impl crate::Writable for TWI_DRV_INT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_int_ctrl to value 0"]
impl crate::Resettable for TWI_DRV_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
