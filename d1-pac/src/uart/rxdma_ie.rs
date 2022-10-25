#[doc = "Register `rxdma_ie` reader"]
pub struct R(crate::R<RXDMA_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxdma_ie` writer"]
pub struct W(crate::W<RXDMA_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_IE_SPEC>;
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
impl From<crate::W<RXDMA_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `limit_done` reader - "]
pub type LIMIT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `limit_done` writer - "]
pub type LIMIT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXDMA_IE_SPEC, bool, O>;
#[doc = "Field `blk_done` reader - "]
pub type BLK_DONE_R = crate::BitReader<bool>;
#[doc = "Field `blk_done` writer - "]
pub type BLK_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXDMA_IE_SPEC, bool, O>;
#[doc = "Field `timeout_done` reader - "]
pub type TIMEOUT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `timeout_done` writer - "]
pub type TIMEOUT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXDMA_IE_SPEC, bool, O>;
#[doc = "Field `buffer_overrun` reader - "]
pub type BUFFER_OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `buffer_overrun` writer - "]
pub type BUFFER_OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXDMA_IE_SPEC, bool, O>;
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
    pub fn limit_done(&mut self) -> LIMIT_DONE_W<0> {
        LIMIT_DONE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn blk_done(&mut self) -> BLK_DONE_W<1> {
        BLK_DONE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_done(&mut self) -> TIMEOUT_DONE_W<2> {
        TIMEOUT_DONE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_overrun(&mut self) -> BUFFER_OVERRUN_W<3> {
        BUFFER_OVERRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_ie](index.html) module"]
pub struct RXDMA_IE_SPEC;
impl crate::RegisterSpec for RXDMA_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_ie::R](R) reader structure"]
impl crate::Readable for RXDMA_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_ie::W](W) writer structure"]
impl crate::Writable for RXDMA_IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_ie to value 0"]
impl crate::Resettable for RXDMA_IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
