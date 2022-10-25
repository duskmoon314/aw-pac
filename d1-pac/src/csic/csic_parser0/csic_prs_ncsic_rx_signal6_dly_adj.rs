#[doc = "Register `csic_prs_ncsic_rx_signal6_dly_adj` reader"]
pub struct R(crate::R<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_prs_ncsic_rx_signal6_dly_adj` writer"]
pub struct W(crate::W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>;
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
impl From<crate::W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `d0_dly` reader - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D0_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d0_dly` writer - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D0_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `d1_dly` reader - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D1_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d1_dly` writer - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D1_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `d2_dly` reader - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D2_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d2_dly` writer - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D2_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `d3_dly` reader - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D3_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `d3_dly` writer - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D3_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d0_dly(&self) -> D0_DLY_R {
        D0_DLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d1_dly(&self) -> D1_DLY_R {
        D1_DLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d2_dly(&self) -> D2_DLY_R {
        D2_DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d3_dly(&self) -> D3_DLY_R {
        D3_DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d0_dly(&mut self) -> D0_DLY_W<0> {
        D0_DLY_W::new(self)
    }
    #[doc = "Bits 8:12 - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d1_dly(&mut self) -> D1_DLY_W<8> {
        D1_DLY_W::new(self)
    }
    #[doc = "Bits 16:20 - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d2_dly(&mut self) -> D2_DLY_W<16> {
        D2_DLY_W::new(self)
    }
    #[doc = "Bits 24:28 - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d3_dly(&mut self) -> D3_DLY_W<24> {
        D3_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Parser NCSIC RX Signal6 Delay Adjust Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_prs_ncsic_rx_signal6_dly_adj](index.html) module"]
pub struct CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_prs_ncsic_rx_signal6_dly_adj::R](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_prs_ncsic_rx_signal6_dly_adj::W](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_rx_signal6_dly_adj to value 0"]
impl crate::Resettable for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
