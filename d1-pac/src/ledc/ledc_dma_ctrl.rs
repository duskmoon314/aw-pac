#[doc = "Register `LEDC_DMA_CTRL` reader"]
pub struct R(crate::R<LEDC_DMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_DMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_DMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_DMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_DMA_CTRL` writer"]
pub struct W(crate::W<LEDC_DMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_DMA_CTRL_SPEC>;
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
impl From<crate::W<LEDC_DMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_DMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDC_DMA_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LEDC_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LEDC_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDC_DMA_EN` reader - "]
pub struct LEDC_DMA_EN_R(crate::FieldReader<bool>);
impl LEDC_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEDC_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDC_DMA_EN_A {
        match self.bits {
            false => LEDC_DMA_EN_A::DISABLE,
            true => LEDC_DMA_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LEDC_DMA_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LEDC_DMA_EN_A::ENABLE
    }
}
impl core::ops::Deref for LEDC_DMA_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_DMA_EN` writer - "]
pub struct LEDC_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDC_DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LEDC_DMA_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LEDC_DMA_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `LEDC_FIFO_TRIG_LEVEL` reader - "]
pub struct LEDC_FIFO_TRIG_LEVEL_R(crate::FieldReader<u8>);
impl LEDC_FIFO_TRIG_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEDC_FIFO_TRIG_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDC_FIFO_TRIG_LEVEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_FIFO_TRIG_LEVEL` writer - "]
pub struct LEDC_FIFO_TRIG_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_FIFO_TRIG_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_dma_en(&self) -> LEDC_DMA_EN_R {
        LEDC_DMA_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ledc_fifo_trig_level(&self) -> LEDC_FIFO_TRIG_LEVEL_R {
        LEDC_FIFO_TRIG_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_dma_en(&mut self) -> LEDC_DMA_EN_W {
        LEDC_DMA_EN_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ledc_fifo_trig_level(&mut self) -> LEDC_FIFO_TRIG_LEVEL_W {
        LEDC_FIFO_TRIG_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_dma_ctrl](index.html) module"]
pub struct LEDC_DMA_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_dma_ctrl::R](R) reader structure"]
impl crate::Readable for LEDC_DMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_dma_ctrl::W](W) writer structure"]
impl crate::Writable for LEDC_DMA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDC_DMA_CTRL to value 0"]
impl crate::Resettable for LEDC_DMA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
