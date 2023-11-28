#[doc = "Register `pc_dat` reader"]
pub type R = crate::R<PC_DAT_SPEC>;
#[doc = "Register `pc_dat` writer"]
pub type W = crate::W<PC_DAT_SPEC>;
#[doc = "Field `pc_dat` reader - "]
pub type PC_DAT_R = crate::FieldReader;
#[doc = "Field `pc_dat` writer - "]
pub type PC_DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pc_dat(&self) -> PC_DAT_R {
        PC_DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pc_dat(&mut self) -> PC_DAT_W<PC_DAT_SPEC> {
        PC_DAT_W::new(self, 0)
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
#[doc = "PC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_DAT_SPEC;
impl crate::RegisterSpec for PC_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_dat::R`](R) reader structure"]
impl crate::Readable for PC_DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc_dat::W`](W) writer structure"]
impl crate::Writable for PC_DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pc_dat to value 0"]
impl crate::Resettable for PC_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
