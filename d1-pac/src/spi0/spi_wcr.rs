#[doc = "Register `spi_wcr` reader"]
pub type R = crate::R<SPI_WCR_SPEC>;
#[doc = "Register `spi_wcr` writer"]
pub type W = crate::W<SPI_WCR_SPEC>;
#[doc = "Field `wwc` reader - Wait clock counter"]
pub type WWC_R = crate::FieldReader<u16>;
#[doc = "Field `wwc` writer - Wait clock counter"]
pub type WWC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `swc` reader - Dual mode direction switch wait clock counter"]
pub type SWC_R = crate::FieldReader;
#[doc = "Field `swc` writer - Dual mode direction switch wait clock counter"]
pub type SWC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Wait clock counter"]
    #[inline(always)]
    pub fn wwc(&self) -> WWC_R {
        WWC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Dual mode direction switch wait clock counter"]
    #[inline(always)]
    pub fn swc(&self) -> SWC_R {
        SWC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wait clock counter"]
    #[inline(always)]
    #[must_use]
    pub fn wwc(&mut self) -> WWC_W<SPI_WCR_SPEC> {
        WWC_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Dual mode direction switch wait clock counter"]
    #[inline(always)]
    #[must_use]
    pub fn swc(&mut self) -> SWC_W<SPI_WCR_SPEC> {
        SWC_W::new(self, 16)
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
#[doc = "SPI Wait Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_wcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_wcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_WCR_SPEC;
impl crate::RegisterSpec for SPI_WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_wcr::R`](R) reader structure"]
impl crate::Readable for SPI_WCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_wcr::W`](W) writer structure"]
impl crate::Writable for SPI_WCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_wcr to value 0"]
impl crate::Resettable for SPI_WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
