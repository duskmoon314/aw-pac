#[doc = "Register `ac_dac_txdata` writer"]
pub struct W(crate::W<AC_DAC_TXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_TXDATA_SPEC>;
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
impl From<crate::W<AC_DAC_TXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_TXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_data` writer - Write the transmitting left and right channel sample data to this register one by one. Write the left channel sample data first and then the right channel sample."]
pub type TX_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_TXDATA_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write the transmitting left and right channel sample data to this register one by one. Write the left channel sample data first and then the right channel sample."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<0> {
        TX_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC TX DATA Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_txdata](index.html) module"]
pub struct AC_DAC_TXDATA_SPEC;
impl crate::RegisterSpec for AC_DAC_TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_txdata::W](W) writer structure"]
impl crate::Writable for AC_DAC_TXDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_txdata to value 0"]
impl crate::Resettable for AC_DAC_TXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
