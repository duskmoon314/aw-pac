#[doc = "Register `spi_bcc` reader"]
pub struct R(crate::R<SPI_BCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_bcc` writer"]
pub struct W(crate::W<SPI_BCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BCC_SPEC>;
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
impl From<crate::W<SPI_BCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `stc` reader - Master Single Mode Transmit Counter"]
pub type STC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `stc` writer - Master Single Mode Transmit Counter"]
pub type STC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_BCC_SPEC, u32, u32, 24, O>;
#[doc = "Field `dbc` reader - Master Dummy Burst Counter"]
pub type DBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dbc` writer - Master Dummy Burst Counter"]
pub type DBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_BCC_SPEC, u8, u8, 4, O>;
#[doc = "Field `drm` reader - Master Dual Mode RX Enable"]
pub type DRM_R = crate::BitReader<DRM_A>;
#[doc = "Master Dual Mode RX Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRM_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DUAL = 1,
}
impl From<DRM_A> for bool {
    #[inline(always)]
    fn from(variant: DRM_A) -> Self {
        variant as u8 != 0
    }
}
impl DRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRM_A {
        match self.bits {
            false => DRM_A::SINGLE,
            true => DRM_A::DUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DRM_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DRM_A::DUAL
    }
}
#[doc = "Field `drm` writer - Master Dual Mode RX Enable"]
pub type DRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BCC_SPEC, DRM_A, O>;
impl<'a, const O: u8> DRM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DRM_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DRM_A::DUAL)
    }
}
#[doc = "Field `quad_en` reader - Quad Mode Enable"]
pub type QUAD_EN_R = crate::BitReader<QUAD_EN_A>;
#[doc = "Quad Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUAD_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<QUAD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: QUAD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl QUAD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUAD_EN_A {
        match self.bits {
            false => QUAD_EN_A::DISABLE,
            true => QUAD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == QUAD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == QUAD_EN_A::ENABLE
    }
}
#[doc = "Field `quad_en` writer - Quad Mode Enable"]
pub type QUAD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BCC_SPEC, QUAD_EN_A, O>;
impl<'a, const O: u8> QUAD_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(QUAD_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(QUAD_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:23 - Master Single Mode Transmit Counter"]
    #[inline(always)]
    pub fn stc(&self) -> STC_R {
        STC_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - Master Dummy Burst Counter"]
    #[inline(always)]
    pub fn dbc(&self) -> DBC_R {
        DBC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Master Dual Mode RX Enable"]
    #[inline(always)]
    pub fn drm(&self) -> DRM_R {
        DRM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Quad Mode Enable"]
    #[inline(always)]
    pub fn quad_en(&self) -> QUAD_EN_R {
        QUAD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Master Single Mode Transmit Counter"]
    #[inline(always)]
    #[must_use]
    pub fn stc(&mut self) -> STC_W<0> {
        STC_W::new(self)
    }
    #[doc = "Bits 24:27 - Master Dummy Burst Counter"]
    #[inline(always)]
    #[must_use]
    pub fn dbc(&mut self) -> DBC_W<24> {
        DBC_W::new(self)
    }
    #[doc = "Bit 28 - Master Dual Mode RX Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drm(&mut self) -> DRM_W<28> {
        DRM_W::new(self)
    }
    #[doc = "Bit 29 - Quad Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn quad_en(&mut self) -> QUAD_EN_W<29> {
        QUAD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Burst Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bcc](index.html) module"]
pub struct SPI_BCC_SPEC;
impl crate::RegisterSpec for SPI_BCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_bcc::R](R) reader structure"]
impl crate::Readable for SPI_BCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_bcc::W](W) writer structure"]
impl crate::Writable for SPI_BCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_bcc to value 0"]
impl crate::Resettable for SPI_BCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
