#[doc = "Register `i2s_pcm_fsta` reader"]
pub type R = crate::R<I2S_PCM_FSTA_SPEC>;
#[doc = "Register `i2s_pcm_fsta` writer"]
pub type W = crate::W<I2S_PCM_FSTA_SPEC>;
#[doc = "Field `rxa_cnt` reader - RXFIFO Available sample word counter"]
pub type RXA_CNT_R = crate::FieldReader;
#[doc = "Field `rxa` reader - RXFIFO Available"]
pub type RXA_R = crate::BitReader<RXA_A>;
#[doc = "RXFIFO Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXA_A {
    #[doc = "0: No Available data in RXFIFO"]
    NO_AVAILABLE = 0,
    #[doc = "1: More than one sample in RXFIFO (&lt;= 1 Word)"]
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
    pub const fn variant(&self) -> RXA_A {
        match self.bits {
            false => RXA_A::NO_AVAILABLE,
            true => RXA_A::AVAILABLE,
        }
    }
    #[doc = "No Available data in RXFIFO"]
    #[inline(always)]
    pub fn is_no_available(&self) -> bool {
        *self == RXA_A::NO_AVAILABLE
    }
    #[doc = "More than one sample in RXFIFO (&lt;= 1 Word)"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == RXA_A::AVAILABLE
    }
}
#[doc = "Field `txe_cnt` reader - TXFIFO Empty space word counter"]
pub type TXE_CNT_R = crate::FieldReader;
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
    pub const fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOT_EMPTY,
            true => TXE_A::EMPTY,
        }
    }
    #[doc = "No room for new sample in TXFIFO"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOT_EMPTY
    }
    #[doc = "More than one sample in RXFIFO (>= 1 Word)"]
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
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S/PCM FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fsta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fsta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_FSTA_SPEC;
impl crate::RegisterSpec for I2S_PCM_FSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_fsta::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_FSTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_fsta::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_FSTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_fsta to value 0"]
impl crate::Resettable for I2S_PCM_FSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
