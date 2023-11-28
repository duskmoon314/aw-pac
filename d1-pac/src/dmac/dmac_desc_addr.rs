#[doc = "Register `dmac_desc_addr%s` reader"]
pub type R = crate::R<DMAC_DESC_ADDR_SPEC>;
#[doc = "Register `dmac_desc_addr%s` writer"]
pub type W = crate::W<DMAC_DESC_ADDR_SPEC>;
#[doc = "Field `dma_desc_high_addr` reader - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
pub type DMA_DESC_HIGH_ADDR_R = crate::FieldReader;
#[doc = "Field `dma_desc_high_addr` writer - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
pub type DMA_DESC_HIGH_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dma_desc_addr` reader - Lower 30 bits of DMA channel descriptor address"]
pub type DMA_DESC_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `dma_desc_addr` writer - Lower 30 bits of DMA channel descriptor address"]
pub type DMA_DESC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
    #[inline(always)]
    pub fn dma_desc_high_addr(&self) -> DMA_DESC_HIGH_ADDR_R {
        DMA_DESC_HIGH_ADDR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Lower 30 bits of DMA channel descriptor address"]
    #[inline(always)]
    pub fn dma_desc_addr(&self) -> DMA_DESC_ADDR_R {
        DMA_DESC_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Higher 2 bits of DMA channel descriptor high address\n\nDMA Channel Descriptor Address = {bit\\[1:0\\], bit\\[31:2\\], 2'b00}"]
    #[inline(always)]
    #[must_use]
    pub fn dma_desc_high_addr(&mut self) -> DMA_DESC_HIGH_ADDR_W<DMAC_DESC_ADDR_SPEC> {
        DMA_DESC_HIGH_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 2:31 - Lower 30 bits of DMA channel descriptor address"]
    #[inline(always)]
    #[must_use]
    pub fn dma_desc_addr(&mut self) -> DMA_DESC_ADDR_W<DMAC_DESC_ADDR_SPEC> {
        DMA_DESC_ADDR_W::new(self, 2)
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
#[doc = "DMAC Channel Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_desc_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_desc_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_DESC_ADDR_SPEC;
impl crate::RegisterSpec for DMAC_DESC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_desc_addr::R`](R) reader structure"]
impl crate::Readable for DMAC_DESC_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_desc_addr::W`](W) writer structure"]
impl crate::Writable for DMAC_DESC_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmac_desc_addr%s to value 0"]
impl crate::Resettable for DMAC_DESC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
