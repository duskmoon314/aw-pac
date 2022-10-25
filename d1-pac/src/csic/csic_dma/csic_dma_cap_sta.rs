#[doc = "Register `csic_dma_cap_sta` reader"]
pub struct R(crate::R<CSIC_DMA_CAP_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_CAP_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_CAP_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_CAP_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_cap_sta` writer"]
pub struct W(crate::W<CSIC_DMA_CAP_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_CAP_STA_SPEC>;
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
impl From<crate::W<CSIC_DMA_CAP_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_CAP_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `scap_sta` reader - Still capture in progress\n\nIndicates the CSI is capturing still image data (single frame). The bit is set at the start of the first frame after enabling still frame capture. It clears itself after the last pixel of the first frame is captured.\n\nFor CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means filed end."]
pub type SCAP_STA_R = crate::BitReader<bool>;
#[doc = "Field `vcap_sta` reader - Video capture in progress\n\nIndicates the CSI is capturing video image data (multiple frames). The bit is set at the start of the first frame after enabling video capture. When software disables video capture, it clears itself after the last pixel of the current frame is captured."]
pub type VCAP_STA_R = crate::BitReader<bool>;
#[doc = "Field `field_sta` reader - The status of the received field"]
pub type FIELD_STA_R = crate::BitReader<FIELD_STA_A>;
#[doc = "The status of the received field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELD_STA_A {
    #[doc = "0: Field 0"]
    F_IELD_0 = 0,
    #[doc = "1: Field 1"]
    F_IELD_1 = 1,
}
impl From<FIELD_STA_A> for bool {
    #[inline(always)]
    fn from(variant: FIELD_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELD_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD_STA_A {
        match self.bits {
            false => FIELD_STA_A::F_IELD_0,
            true => FIELD_STA_A::F_IELD_1,
        }
    }
    #[doc = "Checks if the value of the field is `F_IELD_0`"]
    #[inline(always)]
    pub fn is_f_ield_0(&self) -> bool {
        *self == FIELD_STA_A::F_IELD_0
    }
    #[doc = "Checks if the value of the field is `F_IELD_1`"]
    #[inline(always)]
    pub fn is_f_ield_1(&self) -> bool {
        *self == FIELD_STA_A::F_IELD_1
    }
}
impl R {
    #[doc = "Bit 0 - Still capture in progress\n\nIndicates the CSI is capturing still image data (single frame). The bit is set at the start of the first frame after enabling still frame capture. It clears itself after the last pixel of the first frame is captured.\n\nFor CCIR656 interface, if the output format is frame planar YCbCr 420 mode, the frame end means the field2 end, the other frame end means filed end."]
    #[inline(always)]
    pub fn scap_sta(&self) -> SCAP_STA_R {
        SCAP_STA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Video capture in progress\n\nIndicates the CSI is capturing video image data (multiple frames). The bit is set at the start of the first frame after enabling video capture. When software disables video capture, it clears itself after the last pixel of the current frame is captured."]
    #[inline(always)]
    pub fn vcap_sta(&self) -> VCAP_STA_R {
        VCAP_STA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status of the received field"]
    #[inline(always)]
    pub fn field_sta(&self) -> FIELD_STA_R {
        FIELD_STA_R::new(((self.bits >> 2) & 1) != 0)
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
#[doc = "CSIC DMA Capture Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_cap_sta](index.html) module"]
pub struct CSIC_DMA_CAP_STA_SPEC;
impl crate::RegisterSpec for CSIC_DMA_CAP_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_cap_sta::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_CAP_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_cap_sta::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_CAP_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_cap_sta to value 0"]
impl crate::Resettable for CSIC_DMA_CAP_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
