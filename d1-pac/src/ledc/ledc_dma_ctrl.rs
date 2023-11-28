#[doc = "Register `ledc_dma_ctrl` reader"]
pub type R = crate::R<LEDC_DMA_CTRL_SPEC>;
#[doc = "Register `ledc_dma_ctrl` writer"]
pub type W = crate::W<LEDC_DMA_CTRL_SPEC>;
#[doc = "Field `ledc_fifo_trig_level` reader - "]
pub type LEDC_FIFO_TRIG_LEVEL_R = crate::FieldReader;
#[doc = "Field `ledc_fifo_trig_level` writer - "]
pub type LEDC_FIFO_TRIG_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
    pub const fn variant(&self) -> LEDC_DMA_EN_A {
        match self.bits {
            false => LEDC_DMA_EN_A::DISABLE,
            true => LEDC_DMA_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LEDC_DMA_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LEDC_DMA_EN_A::ENABLE
    }
}
#[doc = "Field `ledc_dma_en` writer - "]
pub type LEDC_DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG, LEDC_DMA_EN_A>;
impl<'a, REG> LEDC_DMA_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LEDC_DMA_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
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
    pub fn ledc_fifo_trig_level(&mut self) -> LEDC_FIFO_TRIG_LEVEL_W<LEDC_DMA_CTRL_SPEC> {
        LEDC_FIFO_TRIG_LEVEL_W::new(self, 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_dma_en(&mut self) -> LEDC_DMA_EN_W<LEDC_DMA_CTRL_SPEC> {
        LEDC_DMA_EN_W::new(self, 5)
    }
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
#[doc = "LEDC DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_dma_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_dma_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_DMA_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for LEDC_DMA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for LEDC_DMA_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ledc_dma_ctrl to value 0"]
impl crate::Resettable for LEDC_DMA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
