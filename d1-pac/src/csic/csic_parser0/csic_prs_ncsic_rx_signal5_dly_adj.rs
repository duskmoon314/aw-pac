#[doc = "Register `csic_prs_ncsic_rx_signal5_dly_adj` reader"]
pub struct R(crate::R<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_prs_ncsic_rx_signal5_dly_adj` writer"]
pub struct W(crate::W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>;
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
impl From<crate::W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `d4_dly` reader - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D4_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d4_dly` writer - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D4_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `d5_dly` reader - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D5_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d5_dly` writer - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D5_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `d6_dly` reader - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D6_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d6_dly` writer - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D6_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `d7_dly` reader - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D7_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d7_dly` writer - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D7_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d4_dly(&self) -> D4_DLY_R {
        D4_DLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d5_dly(&self) -> D5_DLY_R {
        D5_DLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d6_dly(&self) -> D6_DLY_R {
        D6_DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d7_dly(&self) -> D7_DLY_R {
        D7_DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d4_dly(&mut self) -> D4_DLY_W<0> {
        D4_DLY_W::new(self)
    }
    #[doc = "Bits 8:12 - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d5_dly(&mut self) -> D5_DLY_W<8> {
        D5_DLY_W::new(self)
    }
    #[doc = "Bits 16:20 - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d6_dly(&mut self) -> D6_DLY_W<16> {
        D6_DLY_W::new(self)
    }
    #[doc = "Bits 24:28 - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d7_dly(&mut self) -> D7_DLY_W<24> {
        D7_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Parser NCSIC RX Signal5 Delay Adjust Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_prs_ncsic_rx_signal5_dly_adj](index.html) module"]
pub struct CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_prs_ncsic_rx_signal5_dly_adj::R](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_prs_ncsic_rx_signal5_dly_adj::W](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_rx_signal5_dly_adj to value 0"]
impl crate::Resettable for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
