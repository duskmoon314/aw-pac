#[doc = "Register `ledc_dma_ctrl` reader"]
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
#[doc = "Register `ledc_dma_ctrl` writer"]
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
#[doc = "Field `ledc_fifo_trig_level` reader - "]
pub type LEDC_FIFO_TRIG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ledc_fifo_trig_level` writer - "]
pub type LEDC_FIFO_TRIG_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LEDC_DMA_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `ledc_dma_en` reader - "]
pub type LEDC_DMA_EN_R = crate::BitReader<LEDC_DMA_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LEDC_DMA_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LEDC_DMA_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LEDC_DMA_EN_A::ENABLE
    }
}
#[doc = "Field `ledc_dma_en` writer - "]
pub type LEDC_DMA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LEDC_DMA_CTRL_SPEC, LEDC_DMA_EN_A, O>;
impl<'a, const O: u8> LEDC_DMA_EN_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ledc_fifo_trig_level(&self) -> LEDC_FIFO_TRIG_LEVEL_R {
        LEDC_FIFO_TRIG_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_dma_en(&self) -> LEDC_DMA_EN_R {
        LEDC_DMA_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_fifo_trig_level(&mut self) -> LEDC_FIFO_TRIG_LEVEL_W<0> {
        LEDC_FIFO_TRIG_LEVEL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_dma_en(&mut self) -> LEDC_DMA_EN_W<5> {
        LEDC_DMA_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_dma_ctrl to value 0"]
impl crate::Resettable for LEDC_DMA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
