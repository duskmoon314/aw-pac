#[doc = "Register `EMAC_RX_FRM_FLT` reader"]
pub struct R(crate::R<EMAC_RX_FRM_FLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RX_FRM_FLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RX_FRM_FLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RX_FRM_FLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_RX_FRM_FLT` writer"]
pub struct W(crate::W<EMAC_RX_FRM_FLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RX_FRM_FLT_SPEC>;
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
impl From<crate::W<EMAC_RX_FRM_FLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RX_FRM_FLT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Receive Frame Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rx_frm_flt](index.html) module"]
pub struct EMAC_RX_FRM_FLT_SPEC;
impl crate::RegisterSpec for EMAC_RX_FRM_FLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rx_frm_flt::R](R) reader structure"]
impl crate::Readable for EMAC_RX_FRM_FLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rx_frm_flt::W](W) writer structure"]
impl crate::Writable for EMAC_RX_FRM_FLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_RX_FRM_FLT to value 0"]
impl crate::Resettable for EMAC_RX_FRM_FLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
