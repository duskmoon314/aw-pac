#[doc = "Register `i2s_pcm_fsta` reader"]
pub struct R(crate::R<I2S_PCM_FSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_FSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_FSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_FSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_fsta` writer"]
pub struct W(crate::W<I2S_PCM_FSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_FSTA_SPEC>;
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
impl From<crate::W<I2S_PCM_FSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_FSTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxa_cnt` reader - RXFIFO Available sample word counter"]
pub type RXA_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rxa` reader - RXFIFO Available"]
pub type RXA_R = crate::BitReader<RXA_A>;
#[doc = "RXFIFO Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXA_A {
    #[doc = "0: No Available data in RXFIFO"]
    NO_AVAILABLE = 0,
    #[doc = "1: More than one sample in RXFIFO (<= 1 Word)"]
    AVAILABLE = 1,
}
impl From<RXA_A> for bool {
    #[inline(always)]
    fn from(variant: RXA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXA_A {
        match self.bits {
            false => RXA_A::NO_AVAILABLE,
            true => RXA_A::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVAILABLE`"]
    #[inline(always)]
    pub fn is_no_available(&self) -> bool {
        *self == RXA_A::NO_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXA_A::AVAILABLE
    }
}
#[doc = "Field `txe_cnt` reader - TXFIFO Empty space word counter"]
pub type TXE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `txe` reader - TXFIFO Empty"]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "TXFIFO Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    #[doc = "0: No room for new sample in TXFIFO"]
    NOT_EMPTY = 0,
    #[doc = "1: More than one sample in RXFIFO (>= 1 Word)"]
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOT_EMPTY,
            true => TXE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::EMPTY
    }
}
impl R {
    #[doc = "Bits 0:6 - RXFIFO Available sample word counter"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RXA_CNT_R {
        RXA_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - RXFIFO Available"]
    #[inline(always)]
    pub fn rxa(&self) -> RXA_R {
        RXA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - TXFIFO Empty space word counter"]
    #[inline(always)]
    pub fn txe_cnt(&self) -> TXE_CNT_R {
        TXE_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - TXFIFO Empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 28) & 1) != 0)
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
#[doc = "I2S/PCM FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_fsta](index.html) module"]
pub struct I2S_PCM_FSTA_SPEC;
impl crate::RegisterSpec for I2S_PCM_FSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_fsta::R](R) reader structure"]
impl crate::Readable for I2S_PCM_FSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_fsta::W](W) writer structure"]
impl crate::Writable for I2S_PCM_FSTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_fsta to value 0"]
impl crate::Resettable for I2S_PCM_FSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
