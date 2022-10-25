#[doc = "Register `smhc_idie` reader"]
pub struct R(crate::R<SMHC_IDIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_IDIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_IDIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_IDIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_idie` writer"]
pub struct W(crate::W<SMHC_IDIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_IDIE_SPEC>;
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
impl From<crate::W<SMHC_IDIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_IDIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_int_enb` reader - Transmit Interrupt Enable"]
pub type TX_INT_ENB_R = crate::BitReader<bool>;
#[doc = "Field `tx_int_enb` writer - Transmit Interrupt Enable"]
pub type TX_INT_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_IDIE_SPEC, bool, O>;
#[doc = "Field `rx_int_enb` reader - Receive Interrupt Enables"]
pub type RX_INT_ENB_R = crate::BitReader<bool>;
#[doc = "Field `rx_int_enb` writer - Receive Interrupt Enables"]
pub type RX_INT_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_IDIE_SPEC, bool, O>;
#[doc = "Field `ferr_int_enb` reader - Fatal Bus Error Enable"]
pub type FERR_INT_ENB_R = crate::BitReader<bool>;
#[doc = "Field `ferr_int_enb` writer - Fatal Bus Error Enable"]
pub type FERR_INT_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_IDIE_SPEC, bool, O>;
#[doc = "Field `des_unavl_int_enb` reader - Descriptor Unavailable Interrupt"]
pub type DES_UNAVL_INT_ENB_R = crate::BitReader<bool>;
#[doc = "Field `des_unavl_int_enb` writer - Descriptor Unavailable Interrupt"]
pub type DES_UNAVL_INT_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_IDIE_SPEC, bool, O>;
#[doc = "Field `err_sum_int_enb` reader - Card Error Summary Interrupt Enable"]
pub type ERR_SUM_INT_ENB_R = crate::BitReader<bool>;
#[doc = "Field `err_sum_int_enb` writer - Card Error Summary Interrupt Enable"]
pub type ERR_SUM_INT_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_IDIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tx_int_enb(&self) -> TX_INT_ENB_R {
        TX_INT_ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enables"]
    #[inline(always)]
    pub fn rx_int_enb(&self) -> RX_INT_ENB_R {
        RX_INT_ENB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn ferr_int_enb(&self) -> FERR_INT_ENB_R {
        FERR_INT_ENB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt"]
    #[inline(always)]
    pub fn des_unavl_int_enb(&self) -> DES_UNAVL_INT_ENB_R {
        DES_UNAVL_INT_ENB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary Interrupt Enable"]
    #[inline(always)]
    pub fn err_sum_int_enb(&self) -> ERR_SUM_INT_ENB_R {
        ERR_SUM_INT_ENB_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_int_enb(&mut self) -> TX_INT_ENB_W<0> {
        TX_INT_ENB_W::new(self)
    }
    #[doc = "Bit 1 - Receive Interrupt Enables"]
    #[inline(always)]
    #[must_use]
    pub fn rx_int_enb(&mut self) -> RX_INT_ENB_W<1> {
        RX_INT_ENB_W::new(self)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr_int_enb(&mut self) -> FERR_INT_ENB_W<2> {
        FERR_INT_ENB_W::new(self)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn des_unavl_int_enb(&mut self) -> DES_UNAVL_INT_ENB_W<4> {
        DES_UNAVL_INT_ENB_W::new(self)
    }
    #[doc = "Bit 5 - Card Error Summary Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_sum_int_enb(&mut self) -> ERR_SUM_INT_ENB_W<5> {
        ERR_SUM_INT_ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMAC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_idie](index.html) module"]
pub struct SMHC_IDIE_SPEC;
impl crate::RegisterSpec for SMHC_IDIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_idie::R](R) reader structure"]
impl crate::Readable for SMHC_IDIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_idie::W](W) writer structure"]
impl crate::Writable for SMHC_IDIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_idie to value 0"]
impl crate::Resettable for SMHC_IDIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
