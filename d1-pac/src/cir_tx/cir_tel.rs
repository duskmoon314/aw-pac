#[doc = "Register `cir_tel` reader"]
pub type R = crate::R<CIR_TEL_SPEC>;
#[doc = "Register `cir_tel` writer"]
pub type W = crate::W<CIR_TEL_SPEC>;
#[doc = "Field `tel` reader - TX FIFO empty Level for DRQ and IRQ.\n\nTRIGGER_LEVEL = TEL + 1"]
pub type TEL_R = crate::FieldReader;
#[doc = "Field `tel` writer - TX FIFO empty Level for DRQ and IRQ.\n\nTRIGGER_LEVEL = TEL + 1"]
pub type TEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX FIFO empty Level for DRQ and IRQ.\n\nTRIGGER_LEVEL = TEL + 1"]
    #[inline(always)]
    pub fn tel(&self) -> TEL_R {
        TEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX FIFO empty Level for DRQ and IRQ.\n\nTRIGGER_LEVEL = TEL + 1"]
    #[inline(always)]
    #[must_use]
    pub fn tel(&mut self) -> TEL_W<CIR_TEL_SPEC> {
        TEL_W::new(self, 0)
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
#[doc = "CIR TX FIFO Empty Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_tel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_tel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TEL_SPEC;
impl crate::RegisterSpec for CIR_TEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_tel::R`](R) reader structure"]
impl crate::Readable for CIR_TEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_tel::W`](W) writer structure"]
impl crate::Writable for CIR_TEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tel to value 0"]
impl crate::Resettable for CIR_TEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
