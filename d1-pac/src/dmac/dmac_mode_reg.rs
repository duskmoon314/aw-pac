#[doc = "Register `DMAC_MODE_REG%s` reader"]
pub struct R(crate::R<DMAC_MODE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_MODE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_MODE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_MODE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_MODE_REG%s` writer"]
pub struct W(crate::W<DMAC_MODE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_MODE_REG_SPEC>;
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
impl From<crate::W<DMAC_MODE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_MODE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Destination Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_DST_MODE_A {
    #[doc = "0: `0`"]
    WAITING = 0,
    #[doc = "1: `1`"]
    HANDSHAKE = 1,
}
impl From<DMA_DST_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DST_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DST_MODE` reader - Destination Communication Mode Select"]
pub struct DMA_DST_MODE_R(crate::FieldReader<bool>);
impl DMA_DST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_DST_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_DST_MODE_A {
        match self.bits {
            false => DMA_DST_MODE_A::WAITING,
            true => DMA_DST_MODE_A::HANDSHAKE,
        }
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        **self == DMA_DST_MODE_A::WAITING
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        **self == DMA_DST_MODE_A::HANDSHAKE
    }
}
impl core::ops::Deref for DMA_DST_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DST_MODE` writer - Destination Communication Mode Select"]
pub struct DMA_DST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DST_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_DST_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn waiting(self) -> &'a mut W {
        self.variant(DMA_DST_MODE_A::WAITING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut W {
        self.variant(DMA_DST_MODE_A::HANDSHAKE)
    }
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
#[doc = "Source Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_SRC_MODE_A {
    #[doc = "0: `0`"]
    WAITING = 0,
    #[doc = "1: `1`"]
    HANDSHAKE = 1,
}
impl From<DMA_SRC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_SRC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_SRC_MODE` reader - Source Communication Mode Select"]
pub struct DMA_SRC_MODE_R(crate::FieldReader<bool>);
impl DMA_SRC_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_SRC_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_SRC_MODE_A {
        match self.bits {
            false => DMA_SRC_MODE_A::WAITING,
            true => DMA_SRC_MODE_A::HANDSHAKE,
        }
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        **self == DMA_SRC_MODE_A::WAITING
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        **self == DMA_SRC_MODE_A::HANDSHAKE
    }
}
impl core::ops::Deref for DMA_SRC_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SRC_MODE` writer - Source Communication Mode Select"]
pub struct DMA_SRC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SRC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SRC_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn waiting(self) -> &'a mut W {
        self.variant(DMA_SRC_MODE_A::WAITING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut W {
        self.variant(DMA_SRC_MODE_A::HANDSHAKE)
    }
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
impl R {
    #[doc = "Bit 3 - Destination Communication Mode Select"]
    #[inline(always)]
    pub fn dma_dst_mode(&self) -> DMA_DST_MODE_R {
        DMA_DST_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Communication Mode Select"]
    #[inline(always)]
    pub fn dma_src_mode(&self) -> DMA_SRC_MODE_R {
        DMA_SRC_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Destination Communication Mode Select"]
    #[inline(always)]
    pub fn dma_dst_mode(&mut self) -> DMA_DST_MODE_W {
        DMA_DST_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Source Communication Mode Select"]
    #[inline(always)]
    pub fn dma_src_mode(&mut self) -> DMA_SRC_MODE_W {
        DMA_SRC_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_mode_reg](index.html) module"]
pub struct DMAC_MODE_REG_SPEC;
impl crate::RegisterSpec for DMAC_MODE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_mode_reg::R](R) reader structure"]
impl crate::Readable for DMAC_MODE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_mode_reg::W](W) writer structure"]
impl crate::Writable for DMAC_MODE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_MODE_REG%s to value 0"]
impl crate::Resettable for DMAC_MODE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
