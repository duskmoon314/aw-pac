#[doc = "Register `csic_prs_ncsic_rx_signal0_dly_adj` reader"]
pub struct R(crate::R<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_prs_ncsic_rx_signal0_dly_adj` writer"]
pub struct W(crate::W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>;
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
impl From<crate::W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pclk_dly` reader - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type PCLK_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pclk_dly` writer - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type PCLK_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `hsync_dly` reader - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type HSYNC_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hsync_dly` writer - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type HSYNC_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `vsync_dly` reader - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type VSYNC_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vsync_dly` writer - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type VSYNC_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC, u8, u8, 5, O>;
#[doc = "Field `filed_dly` reader - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type FILED_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `filed_dly` writer - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type FILED_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn pclk_dly(&self) -> PCLK_DLY_R {
        PCLK_DLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn hsync_dly(&self) -> HSYNC_DLY_R {
        HSYNC_DLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn vsync_dly(&self) -> VSYNC_DLY_R {
        VSYNC_DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn filed_dly(&self) -> FILED_DLY_R {
        FILED_DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dly(&mut self) -> PCLK_DLY_W<0> {
        PCLK_DLY_W::new(self)
    }
    #[doc = "Bits 8:12 - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_dly(&mut self) -> HSYNC_DLY_W<8> {
        HSYNC_DLY_W::new(self)
    }
    #[doc = "Bits 16:20 - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_dly(&mut self) -> VSYNC_DLY_W<16> {
        VSYNC_DLY_W::new(self)
    }
    #[doc = "Bits 24:28 - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn filed_dly(&mut self) -> FILED_DLY_W<24> {
        FILED_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Parser NCSIC RX Signal0 Delay Adjust Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_prs_ncsic_rx_signal0_dly_adj](index.html) module"]
pub struct CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_prs_ncsic_rx_signal0_dly_adj::R](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_prs_ncsic_rx_signal0_dly_adj::W](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_rx_signal0_dly_adj to value 0"]
impl crate::Resettable for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
