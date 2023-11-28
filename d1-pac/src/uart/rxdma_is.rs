#[doc = "Register `rxdma_is` reader"]
pub type R = crate::R<RXDMA_IS_SPEC>;
#[doc = "Register `rxdma_is` writer"]
pub type W = crate::W<RXDMA_IS_SPEC>;
#[doc = "Field `limit_done` reader - "]
pub type LIMIT_DONE_R = crate::BitReader;
#[doc = "Field `limit_done` writer - "]
pub type LIMIT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `blk_done` reader - "]
pub type BLK_DONE_R = crate::BitReader;
#[doc = "Field `blk_done` writer - "]
pub type BLK_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timeout_done` reader - "]
pub type TIMEOUT_DONE_R = crate::BitReader;
#[doc = "Field `timeout_done` writer - "]
pub type TIMEOUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `buffer_overrun` reader - "]
pub type BUFFER_OVERRUN_R = crate::BitReader;
#[doc = "Field `buffer_overrun` writer - "]
pub type BUFFER_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limit_done(&self) -> LIMIT_DONE_R {
        LIMIT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn blk_done(&self) -> BLK_DONE_R {
        BLK_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timeout_done(&self) -> TIMEOUT_DONE_R {
        TIMEOUT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn buffer_overrun(&self) -> BUFFER_OVERRUN_R {
        BUFFER_OVERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn limit_done(&mut self) -> LIMIT_DONE_W<RXDMA_IS_SPEC> {
        LIMIT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn blk_done(&mut self) -> BLK_DONE_W<RXDMA_IS_SPEC> {
        BLK_DONE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_done(&mut self) -> TIMEOUT_DONE_W<RXDMA_IS_SPEC> {
        TIMEOUT_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_overrun(&mut self) -> BUFFER_OVERRUN_W<RXDMA_IS_SPEC> {
        BUFFER_OVERRUN_W::new(self, 3)
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
#[doc = "UART RXDMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_is::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_is::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_IS_SPEC;
impl crate::RegisterSpec for RXDMA_IS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_is::R`](R) reader structure"]
impl crate::Readable for RXDMA_IS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_is::W`](W) writer structure"]
impl crate::Writable for RXDMA_IS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_is to value 0"]
impl crate::Resettable for RXDMA_IS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
