#[doc = "Register `csic_prs_ncsic_bt656_head_cfg` reader"]
pub struct R(crate::R<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_prs_ncsic_bt656_head_cfg` writer"]
pub struct W(crate::W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>;
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
impl From<crate::W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_id[0-3]` reader - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
pub type CH_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_id[0-3]` writer - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
pub type CH_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub unsafe fn ch_id(&self, n: u8) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> (n * 8)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch0_id(&self) -> CH_ID_R {
        CH_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch1_id(&self) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch2_id(&self) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch3_id(&self) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_id<const O: u8>(&mut self) -> CH_ID_W<O> {
        CH_ID_W::new(self)
    }
    #[doc = "Bits 0:3 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_id(&mut self) -> CH_ID_W<0> {
        CH_ID_W::new(self)
    }
    #[doc = "Bits 8:11 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_id(&mut self) -> CH_ID_W<8> {
        CH_ID_W::new(self)
    }
    #[doc = "Bits 16:19 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_id(&mut self) -> CH_ID_W<16> {
        CH_ID_W::new(self)
    }
    #[doc = "Bits 24:27 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_id(&mut self) -> CH_ID_W<24> {
        CH_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Parser NCSIC BT656 Header Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_prs_ncsic_bt656_head_cfg](index.html) module"]
pub struct CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_prs_ncsic_bt656_head_cfg::R](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_prs_ncsic_bt656_head_cfg::W](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_bt656_head_cfg to value 0x0302_0100"]
impl crate::Resettable for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302_0100;
}
