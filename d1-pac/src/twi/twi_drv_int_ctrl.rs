#[doc = "Register `TWI_DRV_INT_CTRL` reader"]
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
#[doc = "Register `TWI_DRV_INT_CTRL` writer"]
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
#[doc = "Field `rx_req_int_en` reader - "]
pub struct RX_REQ_INT_EN_R(crate::FieldReader<bool>);
impl RX_REQ_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_REQ_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_REQ_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_req_int_en` writer - "]
pub struct RX_REQ_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REQ_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `tx_req_int_en` reader - "]
pub struct TX_REQ_INT_EN_R(crate::FieldReader<bool>);
impl TX_REQ_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_REQ_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REQ_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_req_int_en` writer - "]
pub struct TX_REQ_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REQ_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `tran_err_int_en` reader - "]
pub struct TRAN_ERR_INT_EN_R(crate::FieldReader<bool>);
impl TRAN_ERR_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRAN_ERR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAN_ERR_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tran_err_int_en` writer - "]
pub struct TRAN_ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAN_ERR_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `tran_com_int_en` reader - "]
pub struct TRAN_COM_INT_EN_R(crate::FieldReader<bool>);
impl TRAN_COM_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRAN_COM_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAN_COM_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tran_com_int_en` writer - "]
pub struct TRAN_COM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAN_COM_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `rx_req_pd` reader - "]
pub struct RX_REQ_PD_R(crate::FieldReader<bool>);
impl RX_REQ_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_REQ_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_REQ_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_req_pd` writer - "]
pub struct RX_REQ_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REQ_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `tx_req_pd` reader - "]
pub struct TX_REQ_PD_R(crate::FieldReader<bool>);
impl TX_REQ_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_REQ_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REQ_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_req_pd` writer - "]
pub struct TX_REQ_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REQ_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `tran_err_pd` reader - "]
pub struct TRAN_ERR_PD_R(crate::FieldReader<bool>);
impl TRAN_ERR_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRAN_ERR_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAN_ERR_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tran_err_pd` writer - "]
pub struct TRAN_ERR_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAN_ERR_PD_W<'a> {
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
#[doc = "Field `tran_com_pd` reader - "]
pub struct TRAN_COM_PD_R(crate::FieldReader<bool>);
impl TRAN_COM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRAN_COM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAN_COM_PD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tran_com_pd` writer - "]
pub struct TRAN_COM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAN_COM_PD_W<'a> {
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
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_req_int_en(&self) -> RX_REQ_INT_EN_R {
        RX_REQ_INT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_req_int_en(&self) -> TX_REQ_INT_EN_R {
        TX_REQ_INT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tran_err_int_en(&self) -> TRAN_ERR_INT_EN_R {
        TRAN_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tran_com_int_en(&self) -> TRAN_COM_INT_EN_R {
        TRAN_COM_INT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_req_pd(&self) -> RX_REQ_PD_R {
        RX_REQ_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_req_pd(&self) -> TX_REQ_PD_R {
        TX_REQ_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tran_err_pd(&self) -> TRAN_ERR_PD_R {
        TRAN_ERR_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tran_com_pd(&self) -> TRAN_COM_PD_R {
        TRAN_COM_PD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_req_int_en(&mut self) -> RX_REQ_INT_EN_W {
        RX_REQ_INT_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_req_int_en(&mut self) -> TX_REQ_INT_EN_W {
        TX_REQ_INT_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tran_err_int_en(&mut self) -> TRAN_ERR_INT_EN_W {
        TRAN_ERR_INT_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tran_com_int_en(&mut self) -> TRAN_COM_INT_EN_W {
        TRAN_COM_INT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_req_pd(&mut self) -> RX_REQ_PD_W {
        RX_REQ_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_req_pd(&mut self) -> TX_REQ_PD_W {
        TX_REQ_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tran_err_pd(&mut self) -> TRAN_ERR_PD_W {
        TRAN_ERR_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tran_com_pd(&mut self) -> TRAN_COM_PD_W {
        TRAN_COM_PD_W { w: self }
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
}
#[doc = "`reset()` method sets TWI_DRV_INT_CTRL to value 0"]
impl crate::Resettable for TWI_DRV_INT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
