#[doc = "Register `pe_dat` reader"]
pub type R = crate::R<PE_DAT_SPEC>;
#[doc = "Register `pe_dat` writer"]
pub type W = crate::W<PE_DAT_SPEC>;
#[doc = "Field `pe_dat` reader - PE Data"]
pub type PE_DAT_R = crate::FieldReader<u32>;
#[doc = "Field `pe_dat` writer - PE Data"]
pub type PE_DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - PE Data"]
    #[inline(always)]
    pub fn pe_dat(&self) -> PE_DAT_R {
        PE_DAT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - PE Data"]
    #[inline(always)]
    #[must_use]
    pub fn pe_dat(&mut self) -> PE_DAT_W<PE_DAT_SPEC> {
        PE_DAT_W::new(self, 0)
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
#[doc = "PE Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_DAT_SPEC;
impl crate::RegisterSpec for PE_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_dat::R`](R) reader structure"]
impl crate::Readable for PE_DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pe_dat::W`](W) writer structure"]
impl crate::Writable for PE_DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_dat to value 0"]
impl crate::Resettable for PE_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
