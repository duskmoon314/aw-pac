#[doc = "Register `csic_dma%s_input_sel` reader"]
pub struct R(crate::R<CSIC_DMA_INPUT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_INPUT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_INPUT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_INPUT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma%s_input_sel` writer"]
pub struct W(crate::W<CSIC_DMA_INPUT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_INPUT_SEL_SPEC>;
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
impl From<crate::W<CSIC_DMA_INPUT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_INPUT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_sel` reader - Input selection for DMA\\[i\\]"]
pub type INPUT_SEL_R = crate::FieldReader<u8, INPUT_SEL_A>;
#[doc = "Input selection for DMA\\[i\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT_SEL_A {
    #[doc = "0: `0`"]
    ISP0_CH0 = 0,
    #[doc = "1: `1`"]
    ISP0_CH1 = 1,
    #[doc = "2: `10`"]
    ISP0_CH2 = 2,
    #[doc = "3: `11`"]
    ISP0_CH3 = 3,
}
impl From<INPUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT_SEL_A) -> Self {
        variant as _
    }
}
impl INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT_SEL_A> {
        match self.bits {
            0 => Some(INPUT_SEL_A::ISP0_CH0),
            1 => Some(INPUT_SEL_A::ISP0_CH1),
            2 => Some(INPUT_SEL_A::ISP0_CH2),
            3 => Some(INPUT_SEL_A::ISP0_CH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ISP0_CH0`"]
    #[inline(always)]
    pub fn is_isp0_ch0(&self) -> bool {
        *self == INPUT_SEL_A::ISP0_CH0
    }
    #[doc = "Checks if the value of the field is `ISP0_CH1`"]
    #[inline(always)]
    pub fn is_isp0_ch1(&self) -> bool {
        *self == INPUT_SEL_A::ISP0_CH1
    }
    #[doc = "Checks if the value of the field is `ISP0_CH2`"]
    #[inline(always)]
    pub fn is_isp0_ch2(&self) -> bool {
        *self == INPUT_SEL_A::ISP0_CH2
    }
    #[doc = "Checks if the value of the field is `ISP0_CH3`"]
    #[inline(always)]
    pub fn is_isp0_ch3(&self) -> bool {
        *self == INPUT_SEL_A::ISP0_CH3
    }
}
#[doc = "Field `input_sel` writer - Input selection for DMA\\[i\\]"]
pub type INPUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_INPUT_SEL_SPEC, u8, INPUT_SEL_A, 4, O>;
impl<'a, const O: u8> INPUT_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn isp0_ch0(self) -> &'a mut W {
        self.variant(INPUT_SEL_A::ISP0_CH0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isp0_ch1(self) -> &'a mut W {
        self.variant(INPUT_SEL_A::ISP0_CH1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn isp0_ch2(self) -> &'a mut W {
        self.variant(INPUT_SEL_A::ISP0_CH2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn isp0_ch3(self) -> &'a mut W {
        self.variant(INPUT_SEL_A::ISP0_CH3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input selection for DMA\\[i\\]"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input selection for DMA\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA\\[i\\] Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_input_sel](index.html) module"]
pub struct CSIC_DMA_INPUT_SEL_SPEC;
impl crate::RegisterSpec for CSIC_DMA_INPUT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_input_sel::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_INPUT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_input_sel::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_INPUT_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma%s_input_sel to value 0"]
impl crate::Resettable for CSIC_DMA_INPUT_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
